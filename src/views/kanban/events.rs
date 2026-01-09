use super::{KanbanMode, KanbanState};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_event(state: &mut KanbanState, key: KeyEvent) {
    match &state.mode {
        KanbanMode::Normal => handle_normal_mode(state, key),
        KanbanMode::AddingTask { .. } => handle_text_input_mode(state, key, false),
        KanbanMode::EditingTask { .. } => handle_text_input_mode(state, key, true),
        KanbanMode::ConfirmDelete { task_index } => handle_confirm_delete(state, key, *task_index),
        KanbanMode::ConfirmMoveBack { task_index } => {
            handle_confirm_move_back(state, key, *task_index)
        }
        KanbanMode::ClassifyingTask { .. } => handle_classifying_mode(state, key),
        KanbanMode::ReviewPopup {
            task_index,
            scroll_offset,
            selected_action,
        } => handle_review_popup_mode(state, key, *task_index, *scroll_offset, *selected_action),
        KanbanMode::ReviewRequestChanges { .. } => handle_review_request_changes_mode(state, key),
    }
}

fn handle_normal_mode(state: &mut KanbanState, key: KeyEvent) {
    match key.code {
        // Navigation
        KeyCode::Left | KeyCode::Char('h') => state.previous_column(),
        KeyCode::Right | KeyCode::Char('l') => state.next_column(),
        KeyCode::Up | KeyCode::Char('k') => state.previous_task(),
        KeyCode::Down | KeyCode::Char('j') => state.next_task(),

        // Actions
        KeyCode::Char('a') => {
            state.mode = KanbanMode::AddingTask {
                input: String::new(),
            };
        }
        KeyCode::Char('e') => {
            if let Some(task) = state.get_selected_task() {
                let task_index = state.selected_task.unwrap();
                state.mode = KanbanMode::EditingTask {
                    task_index,
                    input: task.title.clone(),
                };
            }
        }
        KeyCode::Char('d') => {
            if let Some(task_index) = state.selected_task {
                state.mode = KanbanMode::ConfirmDelete { task_index };
            }
        }
        KeyCode::Enter => {
            let is_review_column = state.selected_column == 2;
            let is_in_progress_column = state.selected_column == 1;

            if is_review_column {
                if let Some(task_index) = state.selected_task {
                    state.mode = KanbanMode::ReviewPopup {
                        task_index,
                        scroll_offset: 0,
                        selected_action: super::ReviewAction::ReviewInIDE,
                    };
                }
            } else if !is_in_progress_column {
                state.move_task_next();
            }
        }
        KeyCode::Backspace => {
            state.move_task_previous();
        }

        _ => {}
    }
}

fn handle_text_input_mode(state: &mut KanbanState, key: KeyEvent, is_editing: bool) {
    let (input, task_index) = match &mut state.mode {
        KanbanMode::AddingTask { input } => (input, None),
        KanbanMode::EditingTask { task_index, input } => (input, Some(*task_index)),
        _ => return,
    };

    match key.code {
        KeyCode::Char(c) => {
            input.push(c);
        }
        KeyCode::Backspace => {
            input.pop();
        }
        KeyCode::Enter => {
            let title = input.clone();
            if !title.is_empty() {
                if is_editing {
                    if let Some(i) = task_index {
                        state.start_classification_for_edit(title, i);
                    }
                } else {
                    state.start_classification(title);
                }
            } else {
                state.mode = KanbanMode::Normal;
            }
        }
        KeyCode::Esc => {
            state.mode = KanbanMode::Normal;
        }
        _ => {}
    }
}

fn handle_classifying_mode(state: &mut KanbanState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.cancel_classification();
        }
        _ => {}
    }
}

fn handle_confirm_delete(state: &mut KanbanState, key: KeyEvent, task_index: usize) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            state.delete_task(task_index);
            state.mode = KanbanMode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            state.mode = KanbanMode::Normal;
        }
        _ => {}
    }
}

fn handle_confirm_move_back(state: &mut KanbanState, key: KeyEvent, _task_index: usize) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            state.execute_move_task_previous();
            state.mode = KanbanMode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            state.mode = KanbanMode::Normal;
        }
        _ => {}
    }
}

fn handle_review_popup_mode(
    state: &mut KanbanState,
    key: KeyEvent,
    task_index: usize,
    scroll_offset: usize,
    selected_action: super::ReviewAction,
) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = KanbanMode::Normal;
        }
        KeyCode::Char('j') => {
            state.mode = KanbanMode::ReviewPopup {
                task_index,
                scroll_offset: scroll_offset.saturating_add(1),
                selected_action,
            };
        }
        KeyCode::Char('k') => {
            state.mode = KanbanMode::ReviewPopup {
                task_index,
                scroll_offset: scroll_offset.saturating_sub(1),
                selected_action,
            };
        }
        KeyCode::PageDown => {
            state.mode = KanbanMode::ReviewPopup {
                task_index,
                scroll_offset: scroll_offset.saturating_add(10),
                selected_action,
            };
        }
        KeyCode::PageUp => {
            state.mode = KanbanMode::ReviewPopup {
                task_index,
                scroll_offset: scroll_offset.saturating_sub(10),
                selected_action,
            };
        }
        KeyCode::Left | KeyCode::Char('h') => {
            let actions = super::ReviewAction::all();
            let current_index = actions
                .iter()
                .position(|a| *a == selected_action)
                .unwrap_or(0);
            let new_index = if current_index == 0 {
                actions.len() - 1
            } else {
                current_index - 1
            };
            state.mode = KanbanMode::ReviewPopup {
                task_index,
                scroll_offset,
                selected_action: actions[new_index],
            };
        }
        KeyCode::Right | KeyCode::Char('l') => {
            let actions = super::ReviewAction::all();
            let current_index = actions
                .iter()
                .position(|a| *a == selected_action)
                .unwrap_or(0);
            let new_index = (current_index + 1) % actions.len();
            state.mode = KanbanMode::ReviewPopup {
                task_index,
                scroll_offset,
                selected_action: actions[new_index],
            };
        }
        KeyCode::Tab => {
            let actions = super::ReviewAction::all();
            let current_index = actions
                .iter()
                .position(|a| *a == selected_action)
                .unwrap_or(0);
            let new_index = (current_index + 1) % actions.len();
            state.mode = KanbanMode::ReviewPopup {
                task_index,
                scroll_offset,
                selected_action: actions[new_index],
            };
        }
        KeyCode::Enter => {
            execute_review_action(state, task_index, selected_action);
        }
        _ => {}
    }
}

fn execute_review_action(state: &mut KanbanState, task_index: usize, action: super::ReviewAction) {
    match action {
        super::ReviewAction::ReviewInIDE => {
            state.pending_ide_open = Some(task_index);
            state.mode = KanbanMode::Normal;
        }
        super::ReviewAction::ReviewInTerminal => {
            state.pending_terminal_switch = Some(task_index);
            state.mode = KanbanMode::Normal;
        }
        super::ReviewAction::RequestChanges => {
            state.mode = KanbanMode::ReviewRequestChanges {
                task_index,
                input: String::new(),
            };
        }
        super::ReviewAction::MarkComplete => {
            state.move_task_to_done(task_index);
            state.mode = KanbanMode::Normal;
        }
    }
}

fn handle_review_request_changes_mode(state: &mut KanbanState, key: KeyEvent) {
    let (input, task_index) = match &mut state.mode {
        KanbanMode::ReviewRequestChanges { task_index, input } => (input, *task_index),
        _ => return,
    };

    match key.code {
        KeyCode::Char(c) => {
            input.push(c);
        }
        KeyCode::Backspace => {
            input.pop();
        }
        KeyCode::Enter => {
            let change_request = input.clone();
            if !change_request.is_empty() {
                state.pending_change_request = Some((task_index, change_request));
            }
            state.mode = KanbanMode::Normal;
        }
        KeyCode::Esc => {
            state.mode = KanbanMode::Normal;
        }
        _ => {}
    }
}
