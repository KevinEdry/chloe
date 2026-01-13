use super::state::{SettingsFocus, SettingsMode, SettingsSection, SettingsState};
use crate::views::tasks::dialogs::{get_option_count, get_selection_result};
use crossterm::event::{KeyCode, KeyEvent};

const IDE_OPTIONS_COUNT: usize = 3;
const TERMINAL_OPTIONS_COUNT: usize = 2;
const VCS_OPTIONS_COUNT: usize = 2;

pub enum SettingsAction {
    None,
    SaveSettings,
}

pub fn handle_key_event(state: &mut SettingsState, key: KeyEvent) -> SettingsAction {
    match state.mode {
        SettingsMode::Normal => handle_normal_mode(state, key),
        SettingsMode::EditingShell { .. } | SettingsMode::EditingAutoSave { .. } => {
            handle_editing_mode(state, key)
        }
        SettingsMode::SelectingProvider { .. } => handle_provider_selection_mode(state, key),
        SettingsMode::SelectingIde { .. } => handle_ide_selection_mode(state, key),
        SettingsMode::SelectingTerminal { .. } => handle_terminal_selection_mode(state, key),
        SettingsMode::SelectingVcs { .. } => handle_vcs_selection_mode(state, key),
    }
}

fn handle_normal_mode(state: &mut SettingsState, key: KeyEvent) -> SettingsAction {
    match key.code {
        KeyCode::Tab | KeyCode::BackTab => {
            state.toggle_focus();
            SettingsAction::None
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state.navigate_down();
            SettingsAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.navigate_up();
            SettingsAction::None
        }
        KeyCode::Char('h') | KeyCode::Left => {
            if state.focus == SettingsFocus::Content {
                state.focus = SettingsFocus::Sidebar;
            }
            SettingsAction::None
        }
        KeyCode::Char('l') | KeyCode::Right => {
            state.enter_content();
            SettingsAction::None
        }
        KeyCode::Char('g') => {
            state.selected_section = 0;
            state.selected_item_in_section = 0;
            SettingsAction::None
        }
        KeyCode::Char('G') => {
            state.selected_section = SettingsSection::count() - 1;
            state.selected_item_in_section = 0;
            SettingsAction::None
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if state.focus == SettingsFocus::Sidebar {
                state.focus = SettingsFocus::Content;
                SettingsAction::None
            } else {
                state.start_editing();
                SettingsAction::SaveSettings
            }
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

fn handle_provider_selection_mode(state: &mut SettingsState, key: KeyEvent) -> SettingsAction {
    let SettingsMode::SelectingProvider { selected_index } = state.mode else {
        return SettingsAction::None;
    };

    let option_count = get_option_count(&state.detected_providers);

    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = SettingsMode::Normal;
            SettingsAction::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.mode = SettingsMode::SelectingProvider {
                selected_index: selected_index.saturating_sub(1),
            };
            SettingsAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.mode = SettingsMode::SelectingProvider {
                selected_index: (selected_index + 1).min(option_count - 1),
            };
            SettingsAction::None
        }
        KeyCode::Enter => {
            let result = get_selection_result(
                selected_index,
                &state.detected_providers,
                state.settings.default_provider,
            );

            state.mode = SettingsMode::Normal;

            if let Some(selection) = result {
                state.settings.default_provider = selection.provider();
                if selection.should_remember() {
                    state.settings.skip_provider_selection = true;
                }
                SettingsAction::SaveSettings
            } else {
                SettingsAction::None
            }
        }
        _ => SettingsAction::None,
    }
}

fn handle_ide_selection_mode(state: &mut SettingsState, key: KeyEvent) -> SettingsAction {
    let SettingsMode::SelectingIde { selected_index } = state.mode else {
        return SettingsAction::None;
    };

    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = SettingsMode::Normal;
            SettingsAction::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.mode = SettingsMode::SelectingIde {
                selected_index: selected_index.saturating_sub(1),
            };
            SettingsAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.mode = SettingsMode::SelectingIde {
                selected_index: (selected_index + 1).min(IDE_OPTIONS_COUNT - 1),
            };
            SettingsAction::None
        }
        KeyCode::Enter => {
            state.select_ide(selected_index);
            SettingsAction::SaveSettings
        }
        _ => SettingsAction::None,
    }
}

fn handle_terminal_selection_mode(state: &mut SettingsState, key: KeyEvent) -> SettingsAction {
    let SettingsMode::SelectingTerminal { selected_index } = state.mode else {
        return SettingsAction::None;
    };

    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = SettingsMode::Normal;
            SettingsAction::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.mode = SettingsMode::SelectingTerminal {
                selected_index: selected_index.saturating_sub(1),
            };
            SettingsAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.mode = SettingsMode::SelectingTerminal {
                selected_index: (selected_index + 1).min(TERMINAL_OPTIONS_COUNT - 1),
            };
            SettingsAction::None
        }
        KeyCode::Enter => {
            state.select_terminal(selected_index);
            SettingsAction::SaveSettings
        }
        _ => SettingsAction::None,
    }
}

fn handle_vcs_selection_mode(state: &mut SettingsState, key: KeyEvent) -> SettingsAction {
    let SettingsMode::SelectingVcs { selected_index } = state.mode else {
        return SettingsAction::None;
    };

    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = SettingsMode::Normal;
            SettingsAction::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.mode = SettingsMode::SelectingVcs {
                selected_index: selected_index.saturating_sub(1),
            };
            SettingsAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.mode = SettingsMode::SelectingVcs {
                selected_index: (selected_index + 1).min(VCS_OPTIONS_COUNT - 1),
            };
            SettingsAction::None
        }
        KeyCode::Enter => {
            state.select_vcs(selected_index);
            SettingsAction::SaveSettings
        }
        _ => SettingsAction::None,
    }
}
