use crate::types::AgentProvider;
use crate::views::tasks::state::{MergeTarget, WorktreeSelectionOption};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskAction {
    Create {
        title: String,
    },
    Update {
        task_id: Uuid,
        new_title: String,
    },
    Delete(Uuid),
    OpenInIde(Uuid),
    OpenInTerminal(Uuid),
    RequestChanges {
        task_id: Uuid,
        message: String,
    },
    CommitChanges(Uuid),
    MergeBranch {
        task_id: Uuid,
        target: MergeTarget,
    },
    WorktreeSelected {
        task_id: Uuid,
        worktree_option: WorktreeSelectionOption,
    },
    ProviderSelected {
        task_id: Uuid,
        provider: AgentProvider,
        worktree_option: WorktreeSelectionOption,
        remember: bool,
    },
}
