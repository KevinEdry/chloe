use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Metadata about a git worktree associated with a task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorktreeInfo {
    /// The branch name (e.g., "chloe/implement-worktree-support")
    pub branch_name: String,
    /// Absolute path to the worktree directory
    pub worktree_path: PathBuf,
    /// Whether this worktree was auto-created by Chloe
    pub auto_created: bool,
}

impl WorktreeInfo {
    #[must_use]
    pub fn new(branch_name: String, worktree_path: PathBuf) -> Self {
        Self {
            branch_name,
            worktree_path,
            auto_created: true,
        }
    }
}

/// Represents a git worktree that exists in the repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    pub path: PathBuf,
    pub branch_name: String,
    pub is_bare: bool,
    pub is_detached: bool,
}
