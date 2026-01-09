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
}
