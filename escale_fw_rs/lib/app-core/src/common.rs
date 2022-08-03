use stuff::mq::MessageQueue;

use crate::button::ButtonEvent;

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