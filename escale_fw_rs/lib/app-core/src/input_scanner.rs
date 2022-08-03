use stuff::run_loop::{Task, TaskStatus};

use crate::{
    button::{Button, ButtonEvent},
    common::{AppContext, AppMessage, InputEvent, Instant},
};

pub struct InputScanner<'a> {
    button_a: Button<'a>,
    button_b: Button<'a>,
}

impl<'a> InputScanner<'a> {
    pub fn new<F1, F2, F3>(
        get_is_button_a_down: F1,
        get_is_button_b_down: F2,
        get_instant: F3,
    ) -> InputScanner<'static>
    where
        F1: Fn() -> bool + 'static,
        F2: Fn() -> bool + 'static,
        F3: Fn() -> Instant + Copy + 'static,
    {
        InputScanner {
            button_a: Button::new(get_is_button_a_down, get_instant),
            button_b: Button::new(get_is_button_b_down, get_instant),
        }
    }
}

impl<'a> Task<AppContext> for InputScanner<'a> {
    fn run(&mut self, cx: &mut AppContext) -> TaskStatus {
        self.button_a
            .refresh(|e| cx.mq.push(AppMessage::InputEvent(InputEvent::ButtonA(e))));
        self.button_b
            .refresh(|e| cx.mq.push(AppMessage::InputEvent(InputEvent::ButtonB(e))));
        TaskStatus::Pending
    }
}
