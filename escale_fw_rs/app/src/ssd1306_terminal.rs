use ssd1306::{
    mode::{TerminalDisplaySize, TerminalMode},
    prelude::WriteOnlyDataCommand,
    Ssd1306,
};

use app_core::terminal::Terminal;

pub struct Ssd1306Terminal<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: TerminalDisplaySize,
{
    ssd1306: Ssd1306<DI, SIZE, TerminalMode>,
}

impl<DI, SIZE> Ssd1306Terminal<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: TerminalDisplaySize,
{
    pub fn new(ssd1306: Ssd1306<DI, SIZE, TerminalMode>) -> Self {
        Self { ssd1306 }
    }
}

impl<DI, SIZE> core::fmt::Write for Ssd1306Terminal<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: TerminalDisplaySize,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.ssd1306.write_str(s)
    }
}

impl<DI, SIZE> Terminal for Ssd1306Terminal<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: TerminalDisplaySize,
{
    fn clear(&mut self) -> core::fmt::Result {
        self.ssd1306.clear().map_err(|_| core::fmt::Error)
    }

    fn set_position(&mut self, column: u8, row: u8) -> core::fmt::Result {
        self.ssd1306
            .set_position(column, row)
            .map_err(|_| core::fmt::Error)
    }
}
