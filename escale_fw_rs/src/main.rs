#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(trait_alias)]

mod input_scanner;
mod run_loop;

extern crate alloc;

use alloc::boxed::Box;
use alloc_cortex_m::CortexMHeap;
use core::{alloc::Layout, convert::Infallible, fmt::Write};
use cortex_m_rt::entry;
use embedded_time::rate::Extensions;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::init_clocks_and_plls,
    gpio::{Pin, PullUpInput},
    pac,
    sio::Sio,
    Watchdog, I2C,
};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

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
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    display.init().unwrap();

    let mut cx = AppContext::default();
    let mut schedule: Schedule<AppTask, AppContext> = Schedule::default();

    let pin_a: Pin<_, PullUpInput> = pins.gpio20.into_mode();
    schedule.push(AppTask::InputScanner(InputScanner::new(Box::new(pin_a))));

    loop {
        schedule.run(&mut cx);
        cx.mq.process(|e| match e {
            InputEvent::PinA(pin_a_state) => {
                display.clear().unwrap();
                display
                    .write_fmt(format_args!("{} {}", cx.state.count, pin_a_state))
                    .unwrap();
                cx.state.count += 1;
                MessageProcessingStatus::Processed
            }
        });
    }
}

trait InputPin = embedded_hal::digital::v2::InputPin<Error = Infallible>;

#[derive(Default)]
struct AppContext {
    mq: MessageQueue<InputEvent>,
    state: AppState,
}

#[derive(Default)]
struct AppState {
    count: u32,
}

enum AppTask {
    InputScanner(InputScanner),
}

impl<'a> AsMut<dyn Task<AppContext> + 'a> for AppTask {
    fn as_mut(&mut self) -> &mut (dyn Task<AppContext> + 'a) {
        match self {
            AppTask::InputScanner(input_scanner) => input_scanner,
        }
    }
}
