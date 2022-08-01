use alloc::boxed::Box;

use stuff::run_loop::{Task, TaskStatus};

use crate::common::{AppContext, AppMessage, InputEvent};

pub struct InputScanner {
    button_a: Button,
    button_b: Button,
}

impl InputScanner {
    pub fn new<'a, F1, F2>(get_is_button_a_down: F1, get_is_button_b_down: F2) -> InputScanner
    where
        F1: Fn() -> bool + 'static,
        F2: Fn() -> bool + 'static,
    {
        InputScanner {
            button_a: Button::new(get_is_button_a_down),
            button_b: Button::new(get_is_button_b_down),
        }
    }
}

impl Task<AppContext> for InputScanner {
    fn run(&mut self, cx: &mut AppContext) -> TaskStatus {
        self.button_a.refresh(|is_down| {
            if is_down {
                cx.mq.push(AppMessage::InputEvent(InputEvent::ButtonADown));
            }
        });
        self.button_b.refresh(|is_down| {
            if is_down {
                cx.mq.push(AppMessage::InputEvent(InputEvent::ButtonBDown));
            }
        });
        TaskStatus::Pending
    }
}

struct Button {
    is_down: bool,
    get_is_down: Box<dyn Fn() -> bool>,
}

impl Button {
    fn new<F>(get_is_down: F) -> Button
    where
        F: Fn() -> bool + 'static,
    {
        Button {
            is_down: get_is_down(),
            get_is_down: Box::new(get_is_down),
        }
    }

    fn refresh<F>(&mut self, on_change: F)
    where
        F: FnOnce(bool) -> (),
    {
        let new_is_down = (self.get_is_down)();
        if self.is_down != new_is_down {
            self.is_down = new_is_down;
            on_change(new_is_down);
        }
    }
}
