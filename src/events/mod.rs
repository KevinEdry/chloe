mod app;
pub mod dispatch;
mod event_loop;
mod hook;

pub use crate::views::instances::TerminalAction;
pub use crate::views::pull_requests::PullRequestAction;
pub use crate::views::roadmap::RoadmapAction;
pub use crate::views::settings::SettingsAction;
pub use crate::views::worktree::WorktreeAction;
pub use app::AppEvent;
pub use event_loop::EventLoop;
pub use hook::{EventListener, EventType, HookEvent, get_socket_path, send_event};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppAction {
    Terminal(TerminalAction),
    Roadmap(RoadmapAction),
    PullRequest(PullRequestAction),
    Worktree(WorktreeAction),
    Settings(SettingsAction),
}
