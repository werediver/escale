use stuff::{
    mq::MessageQueue,
    run_loop::{FnTask, Task},
};

use crate::{button::ButtonEvent, dashboard::Dashboard, input_scanner::InputScanner};

pub type Instant = fugit::Instant<u64, 1, 1_000_000>;
pub type Duration = fugit::Duration<u64, 1, 1_000_000>;

#[derive(Default)]
pub struct AppContext {
    pub mq: MessageQueue<AppMessage>,
    pub state: AppState,
}

pub enum AppMessage {
    InputEvent(InputEvent),
    Tare,
    Calibrate,
}

pub enum InputEvent {
    ButtonA(ButtonEvent),
    ButtonB(ButtonEvent),
}

#[derive(Default)]
pub struct AppState {
    pub weight: f32,
}

pub enum AppTask<'a> {
    InputScanner(InputScanner<'a>),
    Dashboard(Dashboard),
    Fn(FnTask<'a, AppContext>),
}

impl<'a> AsMut<dyn Task<AppContext> + 'a> for AppTask<'a> {
    fn as_mut(&mut self) -> &mut (dyn Task<AppContext> + 'a) {
        match self {
            AppTask::InputScanner(task) => task,
            AppTask::Dashboard(task) => task,
            AppTask::Fn(task) => task,
        }
    }
}
