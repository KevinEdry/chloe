use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FocusPanel {
    #[default]
    ActiveTasks,
    DoneTasks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusState {
    pub active_selected_index: usize,
    pub done_selected_index: usize,
    pub focused_panel: FocusPanel,
    pub mode: FocusMode,
    pub details_scroll: u16,
}

impl FocusState {
    pub fn new() -> Self {
        Self {
            active_selected_index: 0,
            done_selected_index: 0,
            focused_panel: FocusPanel::default(),
            mode: FocusMode::Normal,
            details_scroll: 0,
        }
    }
}

impl Default for FocusState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusMode {
    Normal,
    TerminalFocused,
    AddingTask { input: String },
    EditingTask { task_id: Uuid, input: String },
    ConfirmDelete { task_id: Uuid },
    ConfirmStartTask { task_id: Uuid },
    ClassifyingTask { raw_input: String },
    ReviewPopup {
        task_id: Uuid,
        scroll_offset: usize,
        selected_action: FocusReviewAction,
    },
    ReviewRequestChanges {
        task_id: Uuid,
        input: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusReviewAction {
    ReviewInIDE,
    ReviewInTerminal,
    RequestChanges,
    MergeToBranch,
}

impl FocusReviewAction {
    pub const fn all() -> [Self; 4] {
        [
            Self::ReviewInIDE,
            Self::ReviewInTerminal,
            Self::RequestChanges,
            Self::MergeToBranch,
        ]
    }

    pub fn label(&self, branch_name: Option<&str>, has_conflicts: bool) -> String {
        match self {
            Self::ReviewInIDE => "Review in IDE".to_string(),
            Self::ReviewInTerminal => "Review in Terminal".to_string(),
            Self::RequestChanges => "Request Changes".to_string(),
            Self::MergeToBranch => {
                if has_conflicts {
                    "Resolve Conflicts".to_string()
                } else {
                    match branch_name {
                        Some(name) => format!("Merge to {}", name),
                        None => "Mark Complete".to_string(),
                    }
                }
            }
        }
    }
}
