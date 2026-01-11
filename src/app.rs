use crate::types::{AgentProvider, ProviderConfig};
use crate::views::instances::InstanceState;
use crate::views::pull_requests::PullRequestsState;
use crate::views::roadmap::RoadmapState;
use crate::views::settings::SettingsState;
use crate::views::tasks::{TaskType, TasksState};
use crate::views::worktree::WorktreeTabState;
use serde::{Deserialize, Serialize};

const DEFAULT_PTY_ROWS: u16 = 24;
const DEFAULT_PTY_COLUMNS: u16 = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tab {
    Tasks,
    Instances,
    Roadmap,
    Worktree,
    PullRequests,
    Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub active_tab: Tab,
    pub tasks: TasksState,
    pub instances: InstanceState,
    pub roadmap: RoadmapState,
    pub worktree: WorktreeTabState,
    pub pull_requests: PullRequestsState,
    #[serde(skip)]
    pub settings: SettingsState,
    #[serde(skip)]
    pub showing_exit_confirmation: bool,
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_tab: Tab::Tasks,
            tasks: TasksState::new(),
            instances: InstanceState::new(),
            roadmap: RoadmapState::new(),
            worktree: WorktreeTabState::new(),
            pull_requests: PullRequestsState::new(),
            settings: SettingsState::new(),
            showing_exit_confirmation: false,
        }
    }

    #[must_use]
    pub fn load_or_default() -> Self {
        let settings = crate::persistence::storage::load_settings().unwrap_or_default();

        if let Ok(mut app) = crate::persistence::storage::load_state() {
            let active_instance_ids: Vec<uuid::Uuid> = app
                .instances
                .collect_panes()
                .iter()
                .map(|pane| pane.id)
                .collect();
            for column in &mut app.tasks.columns {
                for task in &mut column.tasks {
                    if let Some(instance_id) = task.instance_id
                        && !active_instance_ids.contains(&instance_id)
                    {
                        task.instance_id = None;
                    }
                }
            }

            app.roadmap.sort_items_by_priority();
            app.settings = SettingsState::with_settings(settings);
            app
        } else {
            Self {
                settings: SettingsState::with_settings(settings),
                ..Default::default()
            }
        }
    }

    pub fn save(&self) -> crate::types::Result<()> {
        crate::persistence::storage::save_state(self)
    }

    pub fn save_settings(&self) -> crate::types::Result<()> {
        crate::persistence::storage::save_settings(&self.settings.settings)
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.active_tab = tab;

        if tab == Tab::Worktree {
            self.worktree.mark_needs_refresh();
        }

        if tab == Tab::PullRequests {
            self.pull_requests.mark_needs_refresh();
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Tasks => Tab::Instances,
            Tab::Instances => Tab::Roadmap,
            Tab::Roadmap => Tab::Worktree,
            Tab::Worktree => Tab::PullRequests,
            Tab::PullRequests => Tab::Settings,
            Tab::Settings => Tab::Tasks,
        };

        if self.active_tab == Tab::Worktree {
            self.worktree.mark_needs_refresh();
        }

        if self.active_tab == Tab::PullRequests {
            self.pull_requests.mark_needs_refresh();
        }
    }

    pub fn previous_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Tasks => Tab::Settings,
            Tab::Instances => Tab::Tasks,
            Tab::Roadmap => Tab::Instances,
            Tab::Worktree => Tab::Roadmap,
            Tab::PullRequests => Tab::Worktree,
            Tab::Settings => Tab::PullRequests,
        };

        if self.active_tab == Tab::Worktree {
            self.worktree.mark_needs_refresh();
        }

        if self.active_tab == Tab::PullRequests {
            self.pull_requests.mark_needs_refresh();
        }
    }

    pub fn sync_task_instances(&mut self) {
        if self.tasks.columns.len() < 2 {
            return;
        }

        let in_progress_column = &self.tasks.columns[1];
        let tasks_needing_instances: Vec<(uuid::Uuid, String, String, Option<std::path::PathBuf>)> =
            in_progress_column
                .tasks
                .iter()
                .filter(|task| task.instance_id.is_none())
                .map(|task| {
                    let worktree_path = task
                        .worktree_info
                        .as_ref()
                        .map(|info| info.worktree_path.clone());
                    (
                        task.id,
                        task.title.clone(),
                        task.description.clone(),
                        worktree_path,
                    )
                })
                .collect();

        for (task_id, task_title, task_description, working_directory) in tasks_needing_instances {
            let instance_id = self.instances.create_pane_for_task(
                &task_title,
                &task_description,
                working_directory,
                DEFAULT_PTY_ROWS,
                DEFAULT_PTY_COLUMNS,
            );
            self.tasks.link_task_to_instance(task_id, instance_id);
        }
    }

    pub fn jump_to_task_instance(&mut self) -> bool {
        if let Some(task) = self.tasks.get_kanban_selected_task() {
            let task_id = task.id;
            let task_title = task.title.clone();
            let task_description = task.description.clone();
            let working_directory = task
                .worktree_info
                .as_ref()
                .map(|info| info.worktree_path.clone());

            if let Some(instance_id) = task.instance_id {
                self.active_tab = Tab::Instances;
                return self.instances.select_pane_by_id(instance_id);
            }

            let instance_id = self.instances.create_pane_for_task(
                &task_title,
                &task_description,
                working_directory,
                DEFAULT_PTY_ROWS,
                DEFAULT_PTY_COLUMNS,
            );
            self.tasks.link_task_to_instance(task_id, instance_id);
            self.active_tab = Tab::Instances;
            self.instances.mode = crate::views::instances::InstanceMode::Focused;
            return true;
        }
        false
    }

    #[must_use]
    pub fn get_instance_claude_state(
        &self,
        instance_id: uuid::Uuid,
    ) -> Option<crate::views::instances::ClaudeState> {
        self.instances
            .find_pane(instance_id)
            .map(|pane| pane.claude_state)
    }

    pub fn auto_transition_completed_tasks(&mut self) {
        let completed_instances: Vec<uuid::Uuid> = self
            .instances
            .collect_panes()
            .iter()
            .filter(|pane| pane.claude_state == crate::views::instances::ClaudeState::Done)
            .map(|pane| pane.id)
            .collect();

        for instance_id in completed_instances {
            self.tasks.move_task_to_review_by_instance(instance_id);
        }
    }

    pub fn process_hook_event(&mut self, event: &crate::events::HookEvent) {
        let task_id = event.worktree_id;

        let instance_id = self
            .tasks
            .columns
            .iter()
            .flat_map(|column| &column.tasks)
            .find(|task| task.id == task_id)
            .and_then(|task| task.instance_id);

        let Some(instance_id) = instance_id else {
            return;
        };

        let Some(pane) = self.instances.find_pane_mut(instance_id) else {
            return;
        };

        match event.event_type() {
            crate::events::EventType::Start => {
                pane.claude_state = crate::views::instances::ClaudeState::Running;
                self.tasks.move_task_to_in_progress_by_id(task_id);
            }
            crate::events::EventType::End => {
                pane.claude_state = crate::views::instances::ClaudeState::Done;
            }
            crate::events::EventType::Permission => {
                pane.claude_state = crate::views::instances::ClaudeState::NeedsPermissions;
            }
            crate::events::EventType::Unknown(_) => {}
        }
    }

    pub fn open_task_in_ide(&self, task_id: uuid::Uuid) {
        let Some(task) = self.tasks.find_task_by_id(task_id) else {
            return;
        };

        let path_to_open = if let Some(worktree_info) = &task.worktree_info {
            worktree_info.worktree_path.clone()
        } else if let Some(instance_id) = task.instance_id {
            if let Some(pane) = self.instances.find_pane(instance_id) {
                pane.working_directory.clone()
            } else {
                return;
            }
        } else {
            return;
        };

        let ide_command = self.settings.settings.ide_command.command_name();
        let _ = std::process::Command::new(ide_command)
            .arg(&path_to_open)
            .spawn();
    }

    pub fn open_task_in_terminal(&self, task_id: uuid::Uuid) {
        let Some(task) = self.tasks.find_task_by_id(task_id) else {
            return;
        };

        let path_to_open = if let Some(worktree_info) = &task.worktree_info {
            worktree_info.worktree_path.clone()
        } else if let Some(instance_id) = task.instance_id {
            if let Some(pane) = self.instances.find_pane(instance_id) {
                pane.working_directory.clone()
            } else {
                return;
            }
        } else {
            return;
        };

        let _ = self
            .settings
            .settings
            .terminal_command
            .open_at_path(&path_to_open);
    }

    pub fn convert_roadmap_item_to_task(&mut self, item_index: usize) {
        if let Some(item) = self.roadmap.items.get(item_index) {
            let title = item.title.clone();
            let description = item.description.clone();
            self.tasks
                .add_task_to_planning(title, description, TaskType::Task);
        }
    }

    pub fn open_worktree_in_ide(&self, worktree_index: usize) {
        if let Some(worktree) = self.worktree.worktrees.get(worktree_index) {
            let ide_command = self.settings.settings.ide_command.command_name();
            let _ = std::process::Command::new(ide_command)
                .arg(&worktree.path)
                .spawn();
        }
    }

    pub fn open_worktree_in_terminal(&self, worktree_index: usize) {
        if let Some(worktree) = self.worktree.worktrees.get(worktree_index) {
            let _ = self
                .settings
                .settings
                .terminal_command
                .open_at_path(&worktree.path);
        }
    }

    pub fn commit_task_changes(&mut self, task_id: uuid::Uuid) {
        let Some(task) = self.tasks.find_task_by_id(task_id) else {
            return;
        };

        let Some(instance_id) = task.instance_id else {
            self.tasks.error_message =
                Some("No Claude Code instance associated with this task.".to_string());
            return;
        };

        let commit_prompt = "Please commit the current changes. Review what's been modified and create appropriate atomic commits with clear, descriptive messages.

Before committing, check if this repository has commit message standards or conventions defined (e.g., in CONTRIBUTING.md, README.md, or commit config files like commitlint.config.js, .commitlintrc, committed.toml). If standards exist, follow them. If not, use sensible defaults.

Do not push to remote.";

        self.instances
            .send_input_to_instance(instance_id, commit_prompt);
    }

    pub fn merge_task_branch(
        &mut self,
        task_id: uuid::Uuid,
        target: &crate::views::tasks::state::MergeTarget,
    ) {
        let Some(task) = self.tasks.find_task_by_id(task_id) else {
            return;
        };

        let Some(worktree_info) = &task.worktree_info else {
            self.tasks.error_message =
                Some("No worktree associated with this task. Nothing to merge.".to_string());
            return;
        };
        let worktree_info = worktree_info.clone();

        let Ok(current_directory) = std::env::current_dir() else {
            self.tasks.error_message = Some("Failed to get current directory.".to_string());
            return;
        };

        let Ok(repository_root) = crate::views::worktree::find_repository_root(&current_directory)
        else {
            self.tasks.error_message = Some("Could not find git repository root.".to_string());
            return;
        };

        let has_conflicts =
            crate::views::worktree::check_merge_conflicts(&repository_root, &worktree_info)
                .ok()
                .flatten()
                .is_some();

        if has_conflicts {
            self.resolve_task_conflicts(task_id);
            return;
        }

        let target_branch = target.branch_name();
        let merge_result =
            crate::views::worktree::merge_worktree(&repository_root, &worktree_info, target_branch);

        match merge_result {
            Ok(crate::views::worktree::MergeResult::Success) => {
                let _ = crate::views::worktree::delete_worktree(&repository_root, &worktree_info);
                self.tasks.move_task_to_done_by_id(task_id);
                let _ = self.save();
            }
            Ok(crate::views::worktree::MergeResult::Conflicts { conflicted_files }) => {
                let conflict_message = format!(
                    "Please resolve merge conflicts in the following files:\n{}\n\nThen commit the resolution.",
                    conflicted_files.join("\n")
                );
                if let Some(task_index) = self.tasks.find_task_index_by_id(task_id)
                    && let Some(instance_id) = self.tasks.move_task_to_in_progress(task_index)
                {
                    self.instances
                        .send_input_to_instance(instance_id, &conflict_message);
                }
            }
            Err(error) => {
                self.tasks.error_message = Some(format!("Merge failed: {error}"));
            }
        }
    }

    pub fn resolve_task_conflicts(&mut self, task_id: uuid::Uuid) {
        let Some(task) = self.tasks.find_task_by_id(task_id) else {
            return;
        };

        let Some(worktree_info) = &task.worktree_info else {
            return;
        };
        let worktree_info = worktree_info.clone();

        let Ok(current_directory) = std::env::current_dir() else {
            return;
        };

        let Ok(repository_root) = crate::views::worktree::find_repository_root(&current_directory)
        else {
            return;
        };

        let conflicts =
            crate::views::worktree::check_merge_conflicts(&repository_root, &worktree_info)
                .ok()
                .flatten()
                .unwrap_or_default();

        let default_branch = crate::views::worktree::get_default_branch(&repository_root)
            .unwrap_or_else(|_| "main".to_string());

        let conflict_message = if conflicts.is_empty() {
            format!(
                "Please merge branch '{}' into '{}' and resolve any conflicts that arise.",
                worktree_info.branch_name, default_branch
            )
        } else {
            format!(
                "Please resolve the following merge conflicts when merging '{}' into '{}':\n{}\n\nResolve the conflicts and commit the changes.",
                worktree_info.branch_name,
                default_branch,
                conflicts.join("\n")
            )
        };

        if let Some(task_index) = self.tasks.find_task_index_by_id(task_id)
            && let Some(instance_id) = self.tasks.move_task_to_in_progress(task_index)
        {
            self.instances
                .send_input_to_instance(instance_id, &conflict_message);
        }
    }

    #[must_use]
    pub fn get_provider_config(&self, provider: AgentProvider) -> ProviderConfig {
        self.settings.settings.provider_registry.get_config(provider)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
