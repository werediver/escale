#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(trait_alias)]

mod common;
mod dashboard;
mod input_scanner;
mod run_loop;
mod uptime;

extern crate alloc;

use alloc::rc::Rc;
use alloc_cortex_m::CortexMHeap;
use core::{alloc::Layout, cell::RefCell};
use cortex_m_rt::entry;
use dashboard::Dashboard;
use embedded_hal::digital::v2::InputPin;
use embedded_time::rate::Extensions;
use panic_probe as _;
use uptime::Uptime;

use rp_pico as bsp;

use bsp::hal::{
    clocks::init_clocks_and_plls,
    gpio::{Pin, PullUpInput},
    pac,
    sio::Sio,
    Watchdog, I2C,
};

use ssd1306::{
    mode::{TerminalDisplaySize, TerminalMode},
    prelude::{WriteOnlyDataCommand, *},
    I2CDisplayInterface, Ssd1306,
};

use crate::{input_scanner::*, run_loop::*};

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

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
    let mut _uptime = Uptime::new(core.SYST);

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

    let i2c = I2C::i2c0(
        pac.I2C0,
        pins.gpio16.into_mode(),
        pins.gpio17.into_mode(),
        400.kHz(),
        &mut pac.RESETS,
        clocks.system_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let shared_terminal = Rc::new(RefCell::new(
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode(),
    ));
    {
        let mut terminal = shared_terminal.borrow_mut();
        terminal.init().unwrap();
        terminal.clear().unwrap();
    }

    let mut cx = AppContext::default();
    let mut schedule: Schedule<AppTask, AppContext> = Schedule::default();

    let button_a_pin: Pin<_, PullUpInput> = pins.gpio20.into_mode();
    let button_b_pin: Pin<_, PullUpInput> = pins.gpio26.into_mode();

    schedule.push(AppTask::InputScanner(InputScanner::new(
        move || button_a_pin.is_low().ok().unwrap(),
        move || button_b_pin.is_low().ok().unwrap(),
    )));

    let dashboard = Dashboard::new(shared_terminal.clone(), Uptime::get_instant);
    schedule.push(AppTask::Dashboard(dashboard));

    loop {
        schedule.run(&mut cx);
    }
}

#[derive(Default)]
struct AppContext {
    mq: MessageQueue<InputEvent>,
    state: AppState,
}

#[derive(Default)]
struct AppState {
    weight: f32,
}

enum AppTask {
    InputScanner(InputScanner),
    Dashboard(Dashboard),
}

impl<'a> AsMut<dyn Task<AppContext> + 'a> for AppTask {
    fn as_mut(&mut self) -> &mut (dyn Task<AppContext> + 'a) {
        match self {
            AppTask::InputScanner(input_scanner) => input_scanner,
            AppTask::Dashboard(dashboard) => dashboard,
        }
    }
}

pub trait Terminal: core::fmt::Write {
    fn clear(&mut self) -> core::fmt::Result;
    fn set_position(&mut self, column: u8, row: u8) -> core::fmt::Result;
}

impl<DI, SIZE> Terminal for Ssd1306<DI, SIZE, TerminalMode>
where
    DI: WriteOnlyDataCommand,
    SIZE: TerminalDisplaySize,
{
    fn clear(&mut self) -> core::fmt::Result {
        self.clear().map_err(|_| core::fmt::Error)
    }

    fn set_position(&mut self, column: u8, row: u8) -> core::fmt::Result {
        self.set_position(column, row).map_err(|_| core::fmt::Error)
    }
}
