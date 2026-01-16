mod app_action;
mod app_event;

pub use app_action::AppAction;
pub use app_event::AppEvent;

use crossterm::event::KeyEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventResult {
    Consumed,
    Ignored,
    Action(AppAction),
    Quit,
}

impl EventResult {
    #[must_use]
    pub const fn is_quit(&self) -> bool {
        matches!(self, Self::Quit)
    }
}

pub trait EventHandler {
    fn handle_key(&mut self, key: KeyEvent) -> EventResult;
}
