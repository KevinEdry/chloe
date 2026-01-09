pub mod operations;
pub mod state;
pub mod tab_events;
pub mod tab_state;
pub mod view;

pub use operations::{
    MergeResult, check_merge_conflicts, create_worktree, delete_worktree, find_repository_root,
    generate_branch_name, get_default_branch, is_git_repository, list_worktrees,
    merge_worktree_to_main,
};
pub use state::{Worktree, WorktreeInfo};
pub use tab_state::WorktreeTabState;
