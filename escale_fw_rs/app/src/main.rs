#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(trait_alias)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod flash;
mod ssd1306_terminal;
mod uptime;
mod uptime_delay;

extern crate alloc;

use alloc::rc::Rc;
use alloc_cortex_m::CortexMHeap;
use core::mem::size_of;
use core::{alloc::Layout, cell::RefCell};
use embedded_hal::digital::v2::InputPin;
use flash::{Flash, FLASH_ORIGIN};
use fugit::RateExtU32;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal;
use bsp::hal::{clocks::init_clocks_and_plls, pac, sio::Sio, Clock, Watchdog, I2C};

use nau7802::{Gain, Ldo, Nau7802, SamplesPerSecond};
use ssd1306::{
    mode::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface,
    Ssd1306,
};

use app_core::{
    common::{AppContext, AppMessage, AppTask},
    dashboard::Dashboard,
    input_scanner::InputScanner,
    scale::Scale,
    terminal::Terminal,
};
use ssd1306_terminal::Ssd1306Terminal;
use stuff::{
    mq::MessageProcessingStatus,
    run_loop::{FnTask, Schedule, Task, TaskStatus},
};
use uptime::Uptime;

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[allow(non_upper_case_globals)]
const MiB: usize = 1024 * 1024;
const FLASH_END: usize = FLASH_ORIGIN + 2 * MiB;
const FLASH_CONF_ADDR: usize = FLASH_END - size_of::<Flash<Conf>>();

#[repr(C)]
#[derive(Copy, Clone)]
struct Conf {
    format: u16,
    scale_unit: f32,
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            format: 1,
            scale_unit: 1.0,
        }
    }
}

impl Conf {
    fn is_valid(&self) -> bool {
        self.format != 0 && !self.format != 0
    }
}

fn init_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 128 * 1024;
    static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
}

#[hal::entry]
fn _main() -> ! {
    init_heap();

    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    let core = pac::CorePeripherals::take().unwrap();
    let mut uptime = Uptime::new(core.SYST);

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let i2c0 = I2C::i2c0(
        pac.I2C0,
        pins.gpio16.into_function(),
        pins.gpio17.into_function(),
        400.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );

    let mut cx = AppContext::default();
    let mut schedule: Schedule<AppTask, AppContext> = Schedule::default();

    let button_a_pin = pins.gpio20.into_pull_up_input();
    let button_b_pin = pins.gpio26.into_pull_up_input();

    schedule.push(AppTask::InputScanner(InputScanner::new(
        move || button_a_pin.is_low().unwrap(),
        move || button_b_pin.is_low().unwrap(),
        Uptime::get_instant,
    )));

    let interface = I2CDisplayInterface::new(i2c0);
    let mut ssd1306 =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    ssd1306.init().unwrap();
    let shared_terminal = Rc::new(RefCell::new(Ssd1306Terminal::new(ssd1306)));
    {
        let mut terminal = shared_terminal.borrow_mut();
        terminal.clear().unwrap();
    }
    schedule.push(AppTask::Dashboard(Dashboard::new(
        shared_terminal,
        Uptime::get_instant,
    )));

    let conf = {
        let conf: Conf = unsafe { Flash::read(FLASH_CONF_ADDR).value().assume_init() };

        if conf.is_valid() {
            conf
        } else {
            Default::default()
        }
    };

    let i2c1 = I2C::i2c1(
        pac.I2C1,
        pins.gpio2.into_function(),
        pins.gpio3.into_function(),
        400.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );
    let mut nau7802 = Nau7802::new_with_settings(
        i2c1,
        Ldo::L3v0,
        Gain::G128,
        SamplesPerSecond::SPS20,
        &mut uptime,
    )
    .unwrap();
    let mut scale = Scale::<i32, f32, 20>::default();
    scale.set_unit(conf.scale_unit);

    schedule.push(AppTask::Fn(FnTask::new(move |cx: &mut AppContext| {
        if nau7802.data_available().unwrap() {
            let raw = nau7802.read_unchecked().unwrap();
            scale.push(raw);
            if scale.is_filled() {
                cx.mq.process(|m, _push| match m {
                    AppMessage::Tare => {
                        scale.capture_tare().unwrap();
                        MessageProcessingStatus::Processed
                    }
                    AppMessage::Calibrate => {
                        scale.capture_unit(100.0).unwrap();
                        let conf = Conf {
                            scale_unit: scale.get_unit(),
                            ..Default::default()
                        };
                        cortex_m::interrupt::free(|_cs| unsafe {
                            Flash::new(conf).write(FLASH_CONF_ADDR)
                        });
                        MessageProcessingStatus::Processed
                    }
                    _ => MessageProcessingStatus::Ignored,
                });
                cx.state.weight = scale.read().unwrap();
            }
        }
        TaskStatus::Pending
    })));

    cx.mq.push(AppMessage::Tare);

    loop {
        schedule.run(&mut cx);
    }
}
