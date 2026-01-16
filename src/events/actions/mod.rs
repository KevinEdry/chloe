mod pull_request;
mod roadmap;
mod settings;
mod task;
mod terminal;
mod worktree;

pub use pull_request::PullRequestAction;
pub use roadmap::RoadmapAction;
pub use settings::SettingsAction;
pub use task::TaskAction;
pub use terminal::TerminalAction;
pub use worktree::WorktreeAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppAction {
    Task(TaskAction),
    Terminal(TerminalAction),
    Roadmap(RoadmapAction),
    PullRequest(PullRequestAction),
    Worktree(WorktreeAction),
    Settings(SettingsAction),
}
