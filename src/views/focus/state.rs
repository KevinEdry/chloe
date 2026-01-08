use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusState {
    pub selected_index: usize,
    pub mode: FocusMode,
    pub details_scroll: u16,
}

impl FocusState {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
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
