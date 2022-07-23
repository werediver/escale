use core::cell::RefCell;

use alloc::{format, rc::Rc, string::String};

use crate::{
    common::{Duration, Instant},
    input_scanner::InputEvent,
    run_loop::{MessageProcessingStatus, Task, TaskStatus},
    AppContext, Terminal,
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

    fn render(&mut self, cx: &mut AppContext) -> core::fmt::Result {
        let mut terminal = self.terminal.borrow_mut();
        terminal.set_position(0, 0)?;
        terminal.write_fmt(format_args!("\nWEIGHT: {:<8}\n", cx.state.weight))?;
        terminal.write_fmt(format_args!(
            "\n  TIME:{}\n",
            Self::format_duration(
                self.stopwatch
                    .as_ref()
                    .map_or_else(|| Duration::from_ticks(0), |w| w.read())
            ),
        ))
    }

    fn format_duration(d: Duration) -> String {
        format!(
            "{:2}:{:02}.{:02}",
            d.to_minutes(),
            d.to_secs() % 60,
            d.to_millis() % 1000 / 10
        )
    }

    fn handle_input(&mut self, e: &InputEvent) -> MessageProcessingStatus {
        match e {
            InputEvent::ButtonADown => {
                // Tare
                MessageProcessingStatus::Processed
            }
            InputEvent::ButtonBDown => {
                // Start or stop the stopwatch
                if let Some(stopwatch) = self.stopwatch.as_mut() && stopwatch.is_running() {
                    stopwatch.stop();
                } else {
                    self.stopwatch = Some(Stopwatch::new(self.get_instant));
                }
                MessageProcessingStatus::Processed
            }
        }
    }
}

impl Task<AppContext> for Dashboard {
    fn run(&mut self, cx: &mut AppContext) -> TaskStatus {
        self.render(cx).unwrap();
        cx.mq.process(|m| self.handle_input(m));
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
