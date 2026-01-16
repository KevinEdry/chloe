pub mod actions;
mod app_event;
mod hook_event;

pub use actions::{
    AppAction, PullRequestAction, RoadmapAction, SettingsAction, TaskAction, TerminalAction,
    WorktreeAction,
};
pub use app_event::AppEvent;
pub use hook_event::{EventListener, EventType, HookEvent, get_socket_path, send_event};

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
