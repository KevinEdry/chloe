mod dialogs;
mod focus_navigation;
mod kanban_navigation;
mod review;
mod terminal;
mod text_input;

use super::state::{TasksMode, TasksState, TasksViewMode};
use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;

pub enum TasksAction {
    None,
    JumpToInstance(Uuid),
    SendToTerminal(Uuid, Vec<u8>),
    CreateTask(String),
    UpdateTask { task_id: Uuid, new_title: String },
    DeleteTask(Uuid),
    StartTask(Uuid),
    CancelClassification,
    OpenInIDE(Uuid),
    SwitchToTerminal(Uuid),
    RequestChanges { task_id: Uuid, message: String },
    MergeBranch(Uuid),
}

pub fn handle_key_event(
    state: &mut TasksState,
    key: KeyEvent,
    selected_instance_id: Option<Uuid>,
) -> TasksAction {
    if state.mode == TasksMode::Normal && key.code == KeyCode::Char('/') {
        state.toggle_view_mode();
        return TasksAction::None;
    }

    match &state.mode {
        TasksMode::Normal => match state.view_mode {
            TasksViewMode::Focus => {
                focus_navigation::handle_focus_normal_mode(state, key, selected_instance_id)
            }
            TasksViewMode::Kanban => {
                kanban_navigation::handle_kanban_normal_mode(state, key);
                TasksAction::None
            }
        },
        TasksMode::TerminalFocused => {
            terminal::handle_terminal_focused_mode(state, key, selected_instance_id)
        }
        TasksMode::AddingTask { .. } => text_input::handle_adding_task_mode(state, key),
        TasksMode::EditingTask { .. } => text_input::handle_editing_task_mode(state, key),
        TasksMode::ConfirmDelete { task_id } => {
            dialogs::handle_confirm_delete_mode(state, key, *task_id)
        }
        TasksMode::ConfirmStartTask { task_id } => {
            dialogs::handle_confirm_start_task_mode(state, key, *task_id)
        }
        TasksMode::ConfirmMoveBack { task_id } => {
            dialogs::handle_confirm_move_back_mode(state, key, *task_id)
        }
        TasksMode::ClassifyingTask { .. } => dialogs::handle_classifying_task_mode(state, key),
        TasksMode::ReviewPopup {
            task_id,
            scroll_offset,
            selected_action,
        } => {
            review::handle_review_popup_mode(state, key, *task_id, *scroll_offset, *selected_action)
        }
        TasksMode::ReviewRequestChanges { task_id, .. } => {
            review::handle_review_request_changes_mode(state, key, *task_id)
        }
    }
}
