use crate::types::AgentProvider;
use crate::views::tasks::state::{MergeTarget, WorktreeSelectionOption};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppAction {
    JumpToInstance(Uuid),

    CreateTask {
        title: String,
    },
    UpdateTask {
        task_id: Uuid,
        new_title: String,
    },
    DeleteTask(Uuid),
    OpenTaskInIde(Uuid),
    OpenTaskInTerminal(Uuid),
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

    SendToTerminal {
        instance_id: Uuid,
        data: Vec<u8>,
    },
    ScrollTerminal {
        instance_id: Uuid,
        delta: isize,
    },
    ScrollTerminalToTop(Uuid),
    ScrollTerminalToBottom(Uuid),

    ConvertRoadmapToTask(usize),
    GenerateRoadmap,

    RefreshPullRequests,
    OpenPullRequestInBrowser,

    OpenWorktreeInIde(usize),
    OpenWorktreeInTerminal(usize),

    SaveSettings,
    SaveState,
}
