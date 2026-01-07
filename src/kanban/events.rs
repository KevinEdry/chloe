use super::{KanbanMode, KanbanState};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_event(state: &mut KanbanState, key: KeyEvent) {
    match &state.mode {
        KanbanMode::Normal => handle_normal_mode(state, key),
        KanbanMode::AddingTask { .. } => handle_text_input_mode(state, key, false),
        KanbanMode::EditingTask { .. } => handle_text_input_mode(state, key, true),
        KanbanMode::ConfirmDelete { task_idx } => handle_confirm_delete(state, key, *task_idx),
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
                let task_idx = state.selected_task.unwrap();
                state.mode = KanbanMode::EditingTask {
                    task_idx,
                    input: task.title.clone(),
                };
            }
        }
        KeyCode::Char('d') => {
            if let Some(task_idx) = state.selected_task {
                state.mode = KanbanMode::ConfirmDelete { task_idx };
            }
        }
        KeyCode::Enter => {
            state.move_task_next();
        }
        KeyCode::Backspace => {
            state.move_task_previous();
        }

        _ => {}
    }
}

fn handle_text_input_mode(state: &mut KanbanState, key: KeyEvent, is_editing: bool) {
    let (input, task_idx) = match &mut state.mode {
        KanbanMode::AddingTask { input } => (input, None),
        KanbanMode::EditingTask { task_idx, input } => (input, Some(*task_idx)),
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
                    if let Some(idx) = task_idx {
                        state.edit_task(idx, title, String::new());
                    }
                } else {
                    state.add_task(title, String::new());
                }
            }
            state.mode = KanbanMode::Normal;
        }
        KeyCode::Esc => {
            state.mode = KanbanMode::Normal;
        }
        _ => {}
    }
}

fn handle_confirm_delete(state: &mut KanbanState, key: KeyEvent, task_idx: usize) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            state.delete_task(task_idx);
            state.mode = KanbanMode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            state.mode = KanbanMode::Normal;
        }
        _ => {}
    }
}
