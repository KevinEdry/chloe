pub mod operations;
pub mod state;
pub mod tab_events;
pub mod tab_state;
pub mod tab_ui;

pub use operations::{
    MergeResult, create_worktree, delete_worktree, find_repository_root, generate_branch_name,
    is_git_repository, list_worktrees, merge_worktree_to_main,
};
pub use state::{Worktree, WorktreeInfo};
pub use tab_state::WorktreeTabState;
