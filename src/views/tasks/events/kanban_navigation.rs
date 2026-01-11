use crate::views::tasks::state::{ReviewAction, ReviewPanel, TasksMode, TasksState};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_kanban_normal_mode(state: &mut TasksState, key: KeyEvent) {
    match key.code {
        KeyCode::Left | KeyCode::Char('h') => state.previous_column(),
        KeyCode::Right | KeyCode::Char('l') => state.next_column(),
        KeyCode::Up | KeyCode::Char('k') => state.previous_task(),
        KeyCode::Down | KeyCode::Char('j') => state.next_task(),
        KeyCode::Char('a') => {
            state.mode = TasksMode::AddingTask {
                input: String::new(),
            };
        }
        KeyCode::Char('e') => {
            if let Some(task) = state.get_kanban_selected_task() {
                let task_id = task.id;
                let title = task.title.clone();
                state.mode = TasksMode::EditingTask {
                    task_id,
                    input: title,
                };
            }
        }
        KeyCode::Char('d') => {
            if let Some(task) = state.get_kanban_selected_task() {
                state.mode = TasksMode::ConfirmDelete { task_id: task.id };
            }
        }
        KeyCode::Enter => {
            let is_review_column = state.kanban_selected_column == 2;
            let is_in_progress_column = state.kanban_selected_column == 1;

            if is_review_column {
                if let Some(task) = state.get_kanban_selected_task() {
                    state.mode = TasksMode::ReviewPopup {
                        task_id: task.id,
                        diff_scroll_offset: 0,
                        output_scroll_offset: 0,
                        selected_file_index: 0,
                        focused_panel: ReviewPanel::FileList,
                        selected_action: ReviewAction::ReviewInIDE,
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
