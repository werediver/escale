use crate::{run_loop::*, AppContext, InputPin};

use alloc::boxed::Box;

pub enum InputEvent {
    PinA(bool),
}

pub struct InputScanner {
    pin_a_state: bool,
    pin_a: Box<dyn InputPin>,
}

impl InputScanner {
    pub fn new(pin_a: Box<dyn InputPin>) -> InputScanner {
        InputScanner {
            pin_a_state: false,
            pin_a,
        }
    }
}

impl Task<AppContext> for InputScanner {
    fn run(&mut self, cx: &mut AppContext) -> TaskStatus {
        let pin_a_state_new = self.pin_a.is_low().ok().unwrap();
        if pin_a_state_new != self.pin_a_state {
            cx.mq.push(InputEvent::PinA(pin_a_state_new));
            self.pin_a_state = pin_a_state_new;
        }
        TaskStatus::Pending
    }
}
