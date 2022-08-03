use alloc::{format, rc::Rc, string::String};
use core::cell::RefCell;
use libm::fabsf;

use crate::{
    button::ButtonEvent,
    common::{AppContext, AppMessage, Duration, InputEvent, Instant},
    terminal::Terminal,
};
use stuff::{
    mq::MessageProcessingStatus,
    run_loop::{Task, TaskStatus},
};

pub struct Dashboard {
    terminal: Rc<RefCell<dyn Terminal>>,
    get_instant: fn() -> Instant,
    stopwatch: Option<Stopwatch>,
}

impl Dashboard {
    pub fn new(terminal: Rc<RefCell<dyn Terminal>>, get_instant: fn() -> Instant) -> Self {
        Self {
            terminal,
            get_instant,
            stopwatch: None,
        }
    }

    fn handle_input(
        &mut self,
        e: &AppMessage,
        push: &mut dyn FnMut(AppMessage) -> (),
    ) -> MessageProcessingStatus {
        if let AppMessage::InputEvent(e) = e {
            match e {
                InputEvent::ButtonA(e) => {
                    match e {
                        ButtonEvent::Press => push(AppMessage::Tare),
                        ButtonEvent::LongPress => push(AppMessage::Calibrate),
                    }
                    MessageProcessingStatus::Processed
                }
                InputEvent::ButtonB(e) => match e {
                    ButtonEvent::Press => {
                        if let Some(stopwatch) = self.stopwatch.as_mut() && stopwatch.is_running() {
                            stopwatch.stop();
                        } else {
                            self.stopwatch = Some(Stopwatch::new(self.get_instant));
                        }
                        MessageProcessingStatus::Processed
                    }
                    ButtonEvent::LongPress => MessageProcessingStatus::Ignored,
                },
            }
        } else {
            MessageProcessingStatus::Ignored
        }
    }

    fn render(&mut self, cx: &mut AppContext) -> core::fmt::Result {
        let mut terminal = self.terminal.borrow_mut();
        terminal.set_position(0, 0)?;
        terminal.write_fmt(format_args!(
            "\nWEIGHT: {:<8.2}\n",
            Self::clamp_inf(cx.state.weight, 9999.0)
        ))?;
        terminal.write_fmt(format_args!(
            "\n  TIME:{}\n",
            Self::format_duration(
                self.stopwatch
                    .as_ref()
                    .map_or_else(|| Duration::from_ticks(0), |w| w.read())
            ),
        ))
    }

    fn clamp_inf(x: f32, abs_max: f32) -> f32 {
        if fabsf(x) > abs_max {
            if x >= 0.0 {
                f32::INFINITY
            } else {
                f32::NEG_INFINITY
            }
        } else {
            x
        }
    }

    fn format_duration(d: Duration) -> String {
        format!(
            "{:2}:{:02}.{:02}",
            d.to_minutes(),
            d.to_secs() % 60,
            d.to_millis() % 1000 / 10
        )
    }
}

impl Task<AppContext> for Dashboard {
    fn run(&mut self, cx: &mut AppContext) -> TaskStatus {
        cx.mq.process(|m, push| self.handle_input(m, push));
        self.render(cx).unwrap();
        TaskStatus::Pending
    }
}

struct Stopwatch {
    start: Instant,
    end: Option<Instant>,
    get_instant: fn() -> Instant,
}

impl Stopwatch {
    fn new(get_instant: fn() -> Instant) -> Self {
        Self {
            start: get_instant(),
            end: None,
            get_instant: get_instant,
        }
    }

    fn stop(&mut self) {
        if self.end.is_none() {
            self.end = Some((self.get_instant)());
        }
    }

    fn is_running(&self) -> bool {
        self.end.is_none()
    }

    fn read(&self) -> Duration {
        let end = self.end.unwrap_or_else(|| (self.get_instant)());
        end - self.start
    }
}
