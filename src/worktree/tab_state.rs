use super::state::Worktree;
use serde::{Deserialize, Serialize};
use std::time::Instant;

pub const REFRESH_INTERVAL_SECONDS: u64 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeTabState {
    pub worktrees: Vec<Worktree>,
    pub selected_index: Option<usize>,
    pub mode: WorktreeMode,
    #[serde(skip)]
    pub error_message: Option<String>,
    #[serde(skip)]
    pub(super) last_refresh: Option<Instant>,
    #[serde(skip)]
    pub(super) needs_initial_refresh: bool,
    #[serde(skip)]
    pub pending_ide_open: Option<usize>,
    #[serde(skip)]
    pub pending_terminal_open: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorktreeMode {
    Normal,
    ConfirmDelete { worktree_index: usize },
}

impl WorktreeTabState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            worktrees: Vec::new(),
            selected_index: None,
            mode: WorktreeMode::Normal,
            error_message: None,
            last_refresh: None,
            needs_initial_refresh: true,
            pending_ide_open: None,
            pending_terminal_open: None,
        }
    }

    #[must_use]
    pub fn get_selected_worktree(&self) -> Option<&Worktree> {
        self.selected_index.and_then(|idx| self.worktrees.get(idx))
    }

    pub fn mark_needs_refresh(&mut self) {
        self.needs_initial_refresh = true;
    }
}

impl Default for WorktreeTabState {
    fn default() -> Self {
        Self::new()
    }
}
