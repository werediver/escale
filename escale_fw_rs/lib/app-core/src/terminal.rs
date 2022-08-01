pub trait Terminal: core::fmt::Write {
    fn clear(&mut self) -> core::fmt::Result;
    fn set_position(&mut self, column: u8, row: u8) -> core::fmt::Result;
}
