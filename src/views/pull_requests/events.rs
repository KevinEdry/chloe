use super::state::{PullRequestsMode, PullRequestsState};
use crossterm::event::{KeyCode, KeyEvent};

pub enum PullRequestsAction {
    None,
    Refresh,
    OpenInBrowser,
}

pub fn handle_key_event(state: &mut PullRequestsState, key: KeyEvent) -> PullRequestsAction {
    match state.mode {
        PullRequestsMode::Normal => handle_normal_mode(state, key),
        PullRequestsMode::Viewing => handle_viewing_mode(state, key),
    }
}

fn handle_normal_mode(state: &mut PullRequestsState, key: KeyEvent) -> PullRequestsAction {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.select_next();
            PullRequestsAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.select_previous();
            PullRequestsAction::None
        }
        KeyCode::Char('g') => {
            state.select_first();
            PullRequestsAction::None
        }
        KeyCode::Char('G') => {
            state.select_last();
            PullRequestsAction::None
        }
        KeyCode::Char('r' | 'R') => {
            state.mark_needs_refresh();
            PullRequestsAction::Refresh
        }
        KeyCode::Enter | KeyCode::Char('o') => PullRequestsAction::OpenInBrowser,
        _ => PullRequestsAction::None,
    }
}

const fn handle_viewing_mode(state: &mut PullRequestsState, key: KeyEvent) -> PullRequestsAction {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = PullRequestsMode::Normal;
            PullRequestsAction::None
        }
        _ => PullRequestsAction::None,
    }
}
