use super::state::{PullRequestsMode, PullRequestsState};
use crate::events::{AppAction, EventHandler, EventResult, PullRequestAction};
use crossterm::event::{KeyCode, KeyEvent};

impl EventHandler for PullRequestsState {
    fn handle_key(&mut self, key: KeyEvent) -> EventResult {
        match self.mode {
            PullRequestsMode::Normal => self.handle_normal_mode_event(key),
            PullRequestsMode::Viewing => self.handle_viewing_mode_event(key),
        }
    }
}

impl PullRequestsState {
    fn handle_normal_mode_event(&mut self, key: KeyEvent) -> EventResult {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next();
                EventResult::Consumed
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_previous();
                EventResult::Consumed
            }
            KeyCode::Char('g') => {
                self.select_first();
                EventResult::Consumed
            }
            KeyCode::Char('G') => {
                self.select_last();
                EventResult::Consumed
            }
            KeyCode::Char('r' | 'R') => {
                self.mark_needs_refresh();
                EventResult::Action(AppAction::PullRequest(PullRequestAction::Refresh))
            }
            KeyCode::Enter | KeyCode::Char('o') => {
                EventResult::Action(AppAction::PullRequest(PullRequestAction::OpenInBrowser))
            }
            _ => EventResult::Ignored,
        }
    }

    const fn handle_viewing_mode_event(&mut self, key: KeyEvent) -> EventResult {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.mode = PullRequestsMode::Normal;
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }
}
