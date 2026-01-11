use super::TasksAction;
use crate::views::tasks::state::{MergeTarget, ReviewAction, ReviewPanel, TasksMode, TasksState};
use crate::views::worktree::{find_repository_root, get_current_branch, get_worktree_status};
use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;

const SCROLL_STEP_LINES: usize = 1;
const PAGE_SCROLL_LINES: usize = 10;

#[derive(Clone, Copy)]
pub struct ReviewPopupState {
    pub task_id: Uuid,
    pub diff_scroll_offset: usize,
    pub output_scroll_offset: usize,
    pub selected_file_index: usize,
    pub focused_panel: ReviewPanel,
    pub selected_action: ReviewAction,
}

#[derive(Clone, Copy)]
struct ScrollRequest {
    step: usize,
    is_forward: bool,
}

pub fn handle_review_popup_mode(
    state: &mut TasksState,
    key: KeyEvent,
    popup_state: ReviewPopupState,
) -> TasksAction {
    let mut popup_state = popup_state;

    if matches!(key.code, KeyCode::Esc | KeyCode::Char('q')) {
        state.mode = TasksMode::Normal;
        return TasksAction::None;
    }

    if key.code == KeyCode::Enter {
        return execute_review_action(state, popup_state.task_id, popup_state.selected_action);
    }

    if let Some(panel) = panel_for_key(key.code, popup_state.focused_panel) {
        popup_state.focused_panel = panel;
        return apply_review_popup_state(state, popup_state);
    }

    if let Some(scroll_request) = scroll_request_for_key(key.code) {
        popup_state.apply_scroll_request(scroll_request);
        return apply_review_popup_state(state, popup_state);
    }

    if let Some(action) = action_for_key(key.code, popup_state.selected_action) {
        popup_state.selected_action = action;
        return apply_review_popup_state(state, popup_state);
    }

    TasksAction::None
}

impl ReviewPopupState {
    const fn apply_scroll_request(&mut self, request: ScrollRequest) {
        match self.focused_panel {
            ReviewPanel::FileList => {
                self.selected_file_index =
                    update_offset(self.selected_file_index, request.step, request.is_forward);
            }
            ReviewPanel::DiffContent => {
                self.diff_scroll_offset =
                    update_offset(self.diff_scroll_offset, request.step, request.is_forward);
            }
            ReviewPanel::Output => {
                self.output_scroll_offset =
                    update_offset(self.output_scroll_offset, request.step, request.is_forward);
            }
        }
    }
}

fn apply_review_popup_state(state: &mut TasksState, popup_state: ReviewPopupState) -> TasksAction {
    state.mode = TasksMode::ReviewPopup {
        task_id: popup_state.task_id,
        diff_scroll_offset: popup_state.diff_scroll_offset,
        output_scroll_offset: popup_state.output_scroll_offset,
        selected_file_index: popup_state.selected_file_index,
        focused_panel: popup_state.focused_panel,
        selected_action: popup_state.selected_action,
    };
    TasksAction::None
}

const fn panel_for_key(key: KeyCode, focused_panel: ReviewPanel) -> Option<ReviewPanel> {
    match key {
        KeyCode::Tab => Some(next_panel(focused_panel)),
        KeyCode::BackTab => Some(previous_panel(focused_panel)),
        _ => None,
    }
}

const fn scroll_request_for_key(key: KeyCode) -> Option<ScrollRequest> {
    match key {
        KeyCode::Char('j') | KeyCode::Down => Some(ScrollRequest {
            step: SCROLL_STEP_LINES,
            is_forward: true,
        }),
        KeyCode::Char('k') | KeyCode::Up => Some(ScrollRequest {
            step: SCROLL_STEP_LINES,
            is_forward: false,
        }),
        KeyCode::PageDown => Some(ScrollRequest {
            step: PAGE_SCROLL_LINES,
            is_forward: true,
        }),
        KeyCode::PageUp => Some(ScrollRequest {
            step: PAGE_SCROLL_LINES,
            is_forward: false,
        }),
        _ => None,
    }
}

fn action_for_key(key: KeyCode, selected_action: ReviewAction) -> Option<ReviewAction> {
    match key {
        KeyCode::Left | KeyCode::Char('h') => Some(previous_action(selected_action)),
        KeyCode::Right | KeyCode::Char('l') => Some(next_action(selected_action)),
        _ => None,
    }
}

const fn update_offset(offset: usize, step: usize, is_forward: bool) -> usize {
    if is_forward {
        offset.saturating_add(step)
    } else {
        offset.saturating_sub(step)
    }
}

const fn next_panel(panel: ReviewPanel) -> ReviewPanel {
    match panel {
        ReviewPanel::FileList => ReviewPanel::DiffContent,
        ReviewPanel::DiffContent => ReviewPanel::Output,
        ReviewPanel::Output => ReviewPanel::FileList,
    }
}

const fn previous_panel(panel: ReviewPanel) -> ReviewPanel {
    match panel {
        ReviewPanel::FileList => ReviewPanel::Output,
        ReviewPanel::DiffContent => ReviewPanel::FileList,
        ReviewPanel::Output => ReviewPanel::DiffContent,
    }
}

fn next_action(selected_action: ReviewAction) -> ReviewAction {
    let actions = ReviewAction::all();
    let current_index = actions
        .iter()
        .position(|action| *action == selected_action)
        .unwrap_or(0);
    let new_index = (current_index + 1) % actions.len();
    actions[new_index]
}

fn previous_action(selected_action: ReviewAction) -> ReviewAction {
    let actions = ReviewAction::all();
    let current_index = actions
        .iter()
        .position(|action| *action == selected_action)
        .unwrap_or(0);
    let new_index = if current_index == 0 {
        actions.len() - 1
    } else {
        current_index - 1
    };
    actions[new_index]
}

fn execute_review_action(
    state: &mut TasksState,
    task_id: Uuid,
    action: ReviewAction,
) -> TasksAction {
    let task = state.find_task_by_id(task_id);
    let worktree_info = task.and_then(|task| task.worktree_info.clone());
    let is_clean = worktree_info
        .as_ref()
        .and_then(|info| get_worktree_status(&info.worktree_path).ok())
        .is_none_or(|status| status.is_clean);

    match action {
        ReviewAction::ReviewInIDE => finalize_review_action(state, TasksAction::OpenInIDE(task_id)),
        ReviewAction::ReviewInTerminal => {
            finalize_review_action(state, TasksAction::SwitchToTerminal(task_id))
        }
        ReviewAction::RequestChanges => begin_request_changes(state, task_id),
        ReviewAction::CommitChanges => commit_review_changes(state, task_id, is_clean),
        ReviewAction::MergeAndComplete => {
            begin_merge_confirmation(state, task_id, is_clean, worktree_info)
        }
    }
}

fn finalize_review_action(state: &mut TasksState, action: TasksAction) -> TasksAction {
    state.mode = TasksMode::Normal;
    action
}

fn begin_request_changes(state: &mut TasksState, task_id: Uuid) -> TasksAction {
    state.mode = TasksMode::ReviewRequestChanges {
        task_id,
        input: String::new(),
    };
    TasksAction::None
}

fn commit_review_changes(state: &mut TasksState, task_id: Uuid, is_clean: bool) -> TasksAction {
    if is_clean {
        return TasksAction::None;
    }

    finalize_review_action(state, TasksAction::CommitChanges(task_id))
}

fn begin_merge_confirmation(
    state: &mut TasksState,
    task_id: Uuid,
    is_clean: bool,
    worktree_info: Option<crate::views::worktree::WorktreeInfo>,
) -> TasksAction {
    if !is_clean {
        return TasksAction::None;
    }

    let worktree_branch = worktree_info
        .map(|info| info.branch_name)
        .unwrap_or_default();
    let selected_target = resolve_merge_target();

    state.mode = TasksMode::MergeConfirmation {
        task_id,
        worktree_branch,
        selected_target,
    };
    TasksAction::None
}

fn resolve_merge_target() -> MergeTarget {
    let current_branch = resolve_current_branch();

    if current_branch == "main" || current_branch == "master" {
        MergeTarget::MainBranch
    } else {
        MergeTarget::CurrentBranch(current_branch)
    }
}

fn resolve_current_branch() -> String {
    std::env::current_dir()
        .ok()
        .and_then(|dir| find_repository_root(&dir).ok())
        .and_then(|root| get_current_branch(&root).ok())
        .unwrap_or_else(|| "main".to_string())
}

pub fn handle_review_request_changes_mode(
    state: &mut TasksState,
    key: KeyEvent,
    task_id: Uuid,
) -> TasksAction {
    let TasksMode::ReviewRequestChanges { input, .. } = &mut state.mode else {
        return TasksAction::None;
    };

    match key.code {
        KeyCode::Char(character) => {
            input.push(character);
            TasksAction::None
        }
        KeyCode::Backspace => {
            input.pop();
            TasksAction::None
        }
        KeyCode::Enter => {
            let change_request = input.clone();
            state.mode = TasksMode::Normal;
            if change_request.is_empty() {
                TasksAction::None
            } else {
                TasksAction::RequestChanges {
                    task_id,
                    message: change_request,
                }
            }
        }
        KeyCode::Esc => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        _ => TasksAction::None,
    }
}

pub fn handle_merge_confirmation_mode(
    state: &mut TasksState,
    key: KeyEvent,
    task_id: Uuid,
    worktree_branch: String,
    selected_target: MergeTarget,
) -> TasksAction {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        KeyCode::Up | KeyCode::Down | KeyCode::Char('k' | 'j') => {
            let current_branch = std::env::current_dir()
                .ok()
                .and_then(|dir| find_repository_root(&dir).ok())
                .and_then(|root| get_current_branch(&root).ok())
                .unwrap_or_else(|| "main".to_string());

            let new_target = match &selected_target {
                MergeTarget::CurrentBranch(_) => MergeTarget::MainBranch,
                MergeTarget::MainBranch => {
                    if current_branch == "main" || current_branch == "master" {
                        MergeTarget::MainBranch
                    } else {
                        MergeTarget::CurrentBranch(current_branch)
                    }
                }
            };

            state.mode = TasksMode::MergeConfirmation {
                task_id,
                worktree_branch,
                selected_target: new_target,
            };
            TasksAction::None
        }
        KeyCode::Enter => {
            state.mode = TasksMode::Normal;
            TasksAction::MergeBranch {
                task_id,
                target: selected_target,
            }
        }
        _ => TasksAction::None,
    }
}
