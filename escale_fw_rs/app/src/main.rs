#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(trait_alias)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod flash_static;
mod ssd1306_terminal;
mod uptime;
mod uptime_delay;

extern crate alloc;

use alloc::rc::Rc;
use alloc_cortex_m::CortexMHeap;
use core::{alloc::Layout, cell::RefCell};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::InputPin;
use embedded_time::rate::Extensions;
use flash_static::FlashSector;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::init_clocks_and_plls,
    gpio::{Pin, PullUpInput},
    pac,
    sio::Sio,
    Clock, Watchdog, I2C,
};

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
    loop {}
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[repr(C)]
#[derive(Copy, Clone)]
struct Conf {
    format: u16,
    scale_unit: f32,
}

impl Conf {
    const fn initial() -> Self {
        Self {
            format: 1,
            scale_unit: 1.0,
        }
    }
}

#[link_section = ".rodata"]
static mut CONF_FLASH_SECTOR: FlashSector<Conf> = FlashSector::uninit();

unsafe fn write_conf(conf: Conf) {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    CONF_FLASH_SECTOR.write(conf);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

fn init_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 128 * 1024;
    static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
}

#[entry]
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
        pins.gpio16.into_mode(),
        pins.gpio17.into_mode(),
        400.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );

    let mut cx = AppContext::default();
    let mut schedule: Schedule<AppTask, AppContext> = Schedule::default();

    let button_a_pin: Pin<_, PullUpInput> = pins.gpio20.into_mode();
    let button_b_pin: Pin<_, PullUpInput> = pins.gpio26.into_mode();

    schedule.push(AppTask::InputScanner(InputScanner::new(
        move || button_a_pin.is_low().unwrap(),
        move || button_b_pin.is_low().unwrap(),
        || Uptime::get_instant(),
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
        shared_terminal.clone(),
        Uptime::get_instant,
    )));

    let conf = {
        let conf = unsafe { CONF_FLASH_SECTOR.read().assume_init() };

        let init_conf = Conf::initial();
        if conf.format == init_conf.format {
            conf
        } else {
            init_conf
        }
    };

    let i2c1 = I2C::i2c1(
        pac.I2C1,
        pins.gpio2.into_mode(),
        pins.gpio3.into_mode(),
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
                        let mut conf = Conf::initial();
                        conf.scale_unit = scale.get_unit();
                        unsafe { write_conf(conf) }
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
