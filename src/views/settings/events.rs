use super::state::{SettingsMode, SettingsState};
use crossterm::event::{KeyCode, KeyEvent};

pub enum SettingsAction {
    None,
    SaveSettings,
}

pub fn handle_key_event(state: &mut SettingsState, key: KeyEvent) -> SettingsAction {
    match state.mode {
        SettingsMode::Normal => handle_normal_mode(state, key),
        SettingsMode::EditingShell | SettingsMode::EditingAutoSave => {
            handle_editing_mode(state, key)
        }
    }
}

fn handle_normal_mode(state: &mut SettingsState, key: KeyEvent) -> SettingsAction {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.select_next();
            SettingsAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.select_previous();
            SettingsAction::None
        }
        KeyCode::Char('g') => {
            state.select_first();
            SettingsAction::None
        }
        KeyCode::Char('G') => {
            state.select_last();
            SettingsAction::None
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            state.start_editing();
            SettingsAction::SaveSettings
        }
        _ => SettingsAction::None,
    }
}

fn handle_editing_mode(state: &mut SettingsState, key: KeyEvent) -> SettingsAction {
    match key.code {
        KeyCode::Enter => {
            state.confirm_edit();
            SettingsAction::SaveSettings
        }
        KeyCode::Esc => {
            state.cancel_edit();
            SettingsAction::None
        }
        KeyCode::Backspace => {
            state.handle_edit_backspace();
            SettingsAction::None
        }
        KeyCode::Char(character) => {
            state.handle_edit_input(character);
            SettingsAction::None
        }
        _ => SettingsAction::None,
    }
}
