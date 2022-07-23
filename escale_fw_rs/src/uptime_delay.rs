use embedded_hal::blocking::delay::DelayMs;
use fugit::Duration;

use crate::uptime::Uptime;

impl DelayMs<u64> for Uptime {
    // Does not mutate `self`.
    fn delay_ms(&mut self, ms: u64) {
        let start = Self::get_instant();
        let delay = Duration::<u64, 1, 1_000>::from_ticks(ms);
        let end = start
            .checked_add_duration(delay)
            .expect("uptime must not overflow during the delay");
        while Self::get_instant() < end {}
    }
}
