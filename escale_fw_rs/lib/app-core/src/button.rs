use alloc::boxed::Box;

use crate::common::{Duration, Instant};

pub struct Button<'a> {
    is_down: bool,
    down_since: Option<Instant>,
    get_is_down: Box<dyn Fn() -> bool + 'a>,
    get_instant: Box<dyn Fn() -> Instant + 'a>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ButtonEvent {
    Press,
    LongPress,
}

const LONG_PRESS_DURATION: Duration = Duration::from_ticks(500_000);

impl<'a> Button<'a> {
    pub fn new<F1, F2>(get_is_down: F1, get_instant: F2) -> Self
    where
        F1: Fn() -> bool + 'a,
        F2: Fn() -> Instant + 'a,
    {
        Self {
            is_down: (get_is_down)(),
            down_since: None,
            get_is_down: Box::new(get_is_down),
            get_instant: Box::new(get_instant),
        }
    }

    pub fn refresh<F>(&mut self, on_event: F)
    where
        F: FnOnce(ButtonEvent),
    {
        let now = (self.get_instant)();
        let press_duration = self.down_since.map(|down_since| now - down_since);
        let new_is_down = (self.get_is_down)();
        if self.is_down != new_is_down {
            self.is_down = new_is_down;
            if new_is_down {
                self.down_since = Some(now);
            } else {
                self.down_since = None;
                if let Some(d) = press_duration{
                    if d < LONG_PRESS_DURATION {
                        on_event(ButtonEvent::Press);
                    } else {
                        on_event(ButtonEvent::LongPress);
                    }
                }
            }
        } else if self.is_down && let Some(d) = press_duration && d >= LONG_PRESS_DURATION {
            self.down_since = None;
            on_event(ButtonEvent::LongPress);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;

    use crate::common::Duration;

    use super::*;

    #[test]
    fn button_press() {
        let now = Cell::new(Instant::from_ticks(1_000_000));
        let is_down = Cell::new(false);
        let mut b = Button::new(|| is_down.get(), || now.get());

        b.refresh(|_| panic!());

        is_down.set(true);
        b.refresh(|_| panic!());
        now.set(now.get() + Duration::from_ticks(1_000));
        is_down.set(false);
        b.refresh(|e| assert_eq!(e, ButtonEvent::Press));
    }

    #[test]
    fn button_press_x3() {
        let now = Cell::new(Instant::from_ticks(1_000_000));
        let is_down = Cell::new(false);
        let mut b = Button::new(|| is_down.get(), || now.get());

        b.refresh(|_| panic!());

        is_down.set(true);
        b.refresh(|_| panic!());
        now.set(now.get() + Duration::from_ticks(1_000));
        is_down.set(false);
        b.refresh(|e| assert_eq!(e, ButtonEvent::Press));

        // Very short pause between presses
        now.set(now.get() + Duration::from_ticks(1_000));
        is_down.set(true);
        b.refresh(|_| panic!());
        now.set(now.get() + Duration::from_ticks(1_000));
        is_down.set(false);
        b.refresh(|e| assert_eq!(e, ButtonEvent::Press));

        // Longer pause between presses
        now.set(now.get() + LONG_PRESS_DURATION);
        is_down.set(true);
        b.refresh(|_| panic!());
        now.set(now.get() + Duration::from_ticks(1_000));
        is_down.set(false);
        b.refresh(|e| assert_eq!(e, ButtonEvent::Press));
    }

    #[test]
    fn long_button_press() {
        let now = Cell::new(Instant::from_ticks(1_000_000));
        let is_down = Cell::new(false);
        let mut b = Button::new(|| is_down.get(), || now.get());

        b.refresh(|_| panic!());

        is_down.set(true);
        b.refresh(|_| panic!());
        now.set(now.get() + LONG_PRESS_DURATION);
        b.refresh(|e| assert_eq!(e, ButtonEvent::LongPress));

        // The button can stay depressed for much longer
        now.set(now.get() + LONG_PRESS_DURATION);
        b.refresh(|_| panic!());

        now.set(now.get() + Duration::from_ticks(1_000));
        is_down.set(false);
        b.refresh(|_| panic!());
    }

    #[test]
    fn long_button_press_x2() {
        let now = Cell::new(Instant::from_ticks(1_000_000));
        let is_down = Cell::new(false);
        let mut b = Button::new(|| is_down.get(), || now.get());

        b.refresh(|_| panic!());

        is_down.set(true);
        b.refresh(|_| panic!());
        now.set(now.get() + LONG_PRESS_DURATION);
        b.refresh(|e| assert_eq!(e, ButtonEvent::LongPress));
        now.set(now.get() + Duration::from_ticks(1_000));
        is_down.set(false);
        b.refresh(|_| panic!());

        is_down.set(true);
        b.refresh(|_| panic!());
        now.set(now.get() + LONG_PRESS_DURATION);
        b.refresh(|e| assert_eq!(e, ButtonEvent::LongPress));
        now.set(now.get() + Duration::from_ticks(1_000));
        is_down.set(false);
        b.refresh(|_| panic!());
    }
}
