mod dialogs;
mod focus_navigation;
mod kanban_navigation;
mod provider_selection;
mod review;
mod terminal;
mod text_input;
mod worktree_selection;

use super::state::{MergeTarget, TasksMode, TasksState, TasksViewMode, WorktreeSelectionOption};
use crate::types::AgentProvider;
use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;

pub enum TasksAction {
    None,
    JumpToInstance(Uuid),
    SendToTerminal(Uuid, Vec<u8>),
    ScrollTerminal { instance_id: Uuid, delta: isize },
    ScrollTerminalToTop(Uuid),
    ScrollTerminalToBottom(Uuid),
    CreateTask { title: String },
    UpdateTask { task_id: Uuid, new_title: String },
    DeleteTask(Uuid),
    OpenInIDE(Uuid),
    SwitchToTerminal(Uuid),
    RequestChanges { task_id: Uuid, message: String },
    CommitChanges(Uuid),
    MergeBranch { task_id: Uuid, target: MergeTarget },
    WorktreeSelected {
        task_id: Uuid,
        task_title: String,
        worktree_option: WorktreeSelectionOption,
    },
    ProviderSelected {
        task_id: Uuid,
        provider: AgentProvider,
        worktree_option: WorktreeSelectionOption,
        remember: bool,
    },
}

pub fn handle_key_event(
    state: &mut TasksState,
    key: KeyEvent,
    selected_instance_id: Option<Uuid>,
) -> TasksAction {
    if state.error_message.is_some() {
        state.error_message = None;
        return TasksAction::None;
    }

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
        TasksMode::TerminalScroll => {
            terminal::handle_terminal_scroll_mode(state, key, selected_instance_id)
        }
        TasksMode::AddingTask { .. } => text_input::handle_adding_task_mode(state, key),
        TasksMode::SelectWorktree { .. } => {
            worktree_selection::handle_worktree_selection_mode(state, key)
        }
        TasksMode::SelectProvider { .. } => {
            provider_selection::handle_provider_selection_mode(state, key)
        }
        TasksMode::EditingTask { .. } => text_input::handle_editing_task_mode(state, key),
        TasksMode::ConfirmDelete { task_id } => {
            dialogs::handle_confirm_delete_mode(state, key, *task_id)
        }

        TasksMode::ConfirmMoveBack { task_id } => {
            dialogs::handle_confirm_move_back_mode(state, key, *task_id)
        }
        TasksMode::ReviewPopup {
            task_id,
            diff_scroll_offset,
            output_scroll_offset,
            selected_file_index,
            focused_panel,
            selected_action,
        } => {
            let popup_state = review::ReviewPopupState {
                task_id: *task_id,
                diff_scroll_offset: *diff_scroll_offset,
                output_scroll_offset: *output_scroll_offset,
                selected_file_index: *selected_file_index,
                focused_panel: *focused_panel,
                selected_action: *selected_action,
            };
            review::handle_review_popup_mode(state, key, popup_state)
        }
        TasksMode::ReviewRequestChanges { task_id, .. } => {
            review::handle_review_request_changes_mode(state, key, *task_id)
        }
        TasksMode::MergeConfirmation {
            task_id,
            worktree_branch,
            selected_target,
        } => review::handle_merge_confirmation_mode(
            state,
            key,
            *task_id,
            worktree_branch.clone(),
            selected_target.clone(),
        ),
    }
}
