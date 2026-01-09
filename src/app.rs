use crate::types::Config;
use crate::views::focus::FocusState;
use crate::views::instances::InstanceState;
use crate::views::kanban::{KanbanState, TaskType};
use crate::views::roadmap::RoadmapState;
use crate::views::worktree::WorktreeTabState;
use serde::{Deserialize, Serialize};

const DEFAULT_PTY_ROWS: u16 = 24;
const DEFAULT_PTY_COLUMNS: u16 = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tab {
    Focus,
    Kanban,
    Instances,
    Roadmap,
    Worktree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub active_tab: Tab,
    pub kanban: KanbanState,
    pub instances: InstanceState,
    pub roadmap: RoadmapState,
    pub worktree: WorktreeTabState,
    pub focus: FocusState,
    pub config: Config,
    #[serde(skip)]
    pub showing_exit_confirmation: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: Tab::Focus,
            kanban: KanbanState::new(),
            instances: InstanceState::new(),
            roadmap: RoadmapState::new(),
            worktree: WorktreeTabState::new(),
            focus: FocusState::new(),
            config: Config::default(),
            showing_exit_confirmation: false,
        }
    }

    pub fn load_or_default() -> Self {
        match crate::persistence::storage::load_state() {
            Ok(mut app) => {
                // Preserve config from state.json to maintain user's IDE/terminal preferences

                let active_instance_ids: Vec<uuid::Uuid> =
                    app.instances.panes.iter().map(|p| p.id).collect();
                for column in &mut app.kanban.columns {
                    for task in &mut column.tasks {
                        if let Some(instance_id) = task.instance_id {
                            if !active_instance_ids.contains(&instance_id) {
                                task.instance_id = None;
                            }
                        }
                    }
                }

                app.roadmap.sort_items_by_priority();
                app
            }
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> crate::types::Result<()> {
        crate::persistence::storage::save_state(self)
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.active_tab = tab;

        if tab == Tab::Worktree {
            self.worktree.mark_needs_refresh();
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Focus => Tab::Kanban,
            Tab::Kanban => Tab::Instances,
            Tab::Instances => Tab::Roadmap,
            Tab::Roadmap => Tab::Worktree,
            Tab::Worktree => Tab::Focus,
        };

        if self.active_tab == Tab::Worktree {
            self.worktree.mark_needs_refresh();
        }
    }

    pub fn previous_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Focus => Tab::Worktree,
            Tab::Kanban => Tab::Focus,
            Tab::Instances => Tab::Kanban,
            Tab::Roadmap => Tab::Instances,
            Tab::Worktree => Tab::Roadmap,
        };

        if self.active_tab == Tab::Worktree {
            self.worktree.mark_needs_refresh();
        }
    }

    pub fn sync_task_instances(&mut self) {
        if self.kanban.columns.len() < 2 {
            return;
        }

        let in_progress_column = &self.kanban.columns[1];
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
            self.kanban.link_task_to_instance(task_id, instance_id);
        }
    }

    pub fn jump_to_task_instance(&mut self) -> bool {
        if let Some(task) = self.kanban.get_selected_task() {
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
            } else {
                let instance_id = self.instances.create_pane_for_task(
                    &task_title,
                    &task_description,
                    working_directory,
                    DEFAULT_PTY_ROWS,
                    DEFAULT_PTY_COLUMNS,
                );
                self.kanban.link_task_to_instance(task_id, instance_id);
                self.active_tab = Tab::Instances;
                self.instances.mode = crate::views::instances::InstanceMode::Focused;
                return true;
            }
        }
        false
    }

    pub fn get_instance_claude_state(
        &self,
        instance_id: uuid::Uuid,
    ) -> Option<crate::views::instances::ClaudeState> {
        self.instances
            .panes
            .iter()
            .find(|pane| pane.id == instance_id)
            .map(|pane| pane.claude_state)
    }

    pub fn auto_transition_completed_tasks(&mut self) {
        let completed_instances: Vec<uuid::Uuid> = self
            .instances
            .panes
            .iter()
            .filter(|pane| pane.claude_state == crate::views::instances::ClaudeState::Done)
            .map(|pane| pane.id)
            .collect();

        for instance_id in completed_instances {
            self.kanban.move_task_to_review_by_instance(instance_id);
        }
    }

    pub fn process_hook_event(&mut self, event: &crate::events::HookEvent) {
        let task_id = event.worktree_id;

        let instance_id = self
            .kanban
            .columns
            .iter()
            .flat_map(|col| &col.tasks)
            .find(|task| task.id == task_id)
            .and_then(|task| task.instance_id);

        let Some(instance_id) = instance_id else {
            return;
        };

        let Some(pane) = self
            .instances
            .panes
            .iter_mut()
            .find(|p| p.id == instance_id)
        else {
            return;
        };

        match event.event_type() {
            crate::events::EventType::Start => {
                pane.claude_state = crate::views::instances::ClaudeState::Running;
                self.kanban.move_task_to_in_progress_by_id(task_id);
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

    pub fn get_instance_output(&self, instance_id: uuid::Uuid) -> Option<&str> {
        self.instances
            .panes
            .iter()
            .find(|pane| pane.id == instance_id)
            .map(|pane| pane.output_buffer.as_str())
    }

    pub fn open_task_in_ide(&self, task_index: usize) {
        let review_column_index = 2;
        if let Some(task) = self
            .kanban
            .columns
            .get(review_column_index)
            .and_then(|col| col.tasks.get(task_index))
        {
            let path_to_open = if let Some(worktree_info) = &task.worktree_info {
                &worktree_info.worktree_path
            } else if let Some(instance_id) = task.instance_id {
                if let Some(pane) = self.instances.panes.iter().find(|p| p.id == instance_id) {
                    &pane.working_directory
                } else {
                    return;
                }
            } else {
                return;
            };

            let ide_command = self.config.ide_command.command_name();
            let _ = std::process::Command::new(ide_command)
                .arg(path_to_open)
                .spawn();
        }
    }

    pub fn open_task_in_terminal(&self, task_index: usize) {
        let review_column_index = 2;
        if let Some(task) = self
            .kanban
            .columns
            .get(review_column_index)
            .and_then(|col| col.tasks.get(task_index))
        {
            let path_to_open = if let Some(worktree_info) = &task.worktree_info {
                &worktree_info.worktree_path
            } else if let Some(instance_id) = task.instance_id {
                if let Some(pane) = self.instances.panes.iter().find(|p| p.id == instance_id) {
                    &pane.working_directory
                } else {
                    return;
                }
            } else {
                return;
            };

            let _ = self.config.terminal_command.open_at_path(path_to_open);
        }
    }

    pub fn convert_roadmap_item_to_task(&mut self, item_index: usize) {
        if let Some(item) = self.roadmap.items.get(item_index) {
            let title = item.title.clone();
            let description = item.description.clone();
            self.kanban
                .add_task_to_planning(title, description, TaskType::Task);
        }
    }

    pub fn open_worktree_in_ide(&self, worktree_index: usize) {
        if let Some(worktree) = self.worktree.worktrees.get(worktree_index) {
            let ide_command = self.config.ide_command.command_name();
            let _ = std::process::Command::new(ide_command)
                .arg(&worktree.path)
                .spawn();
        }
    }

    pub fn open_worktree_in_terminal(&self, worktree_index: usize) {
        if let Some(worktree) = self.worktree.worktrees.get(worktree_index) {
            let _ = self.config.terminal_command.open_at_path(&worktree.path);
        }
    }

    pub fn merge_task_branch(&mut self, task_id: uuid::Uuid) {
        let task = match self.kanban.find_task_by_id(task_id) {
            Some(t) => t,
            None => return,
        };

        let worktree_info = match &task.worktree_info {
            Some(info) => info.clone(),
            None => {
                self.kanban.move_task_to_done_by_id(task_id);
                let _ = self.save();
                return;
            }
        };

        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return,
        };

        let repository_root = match crate::views::worktree::find_repository_root(&current_dir) {
            Ok(root) => root,
            Err(_) => return,
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

        let merge_result =
            crate::views::worktree::merge_worktree_to_main(&repository_root, &worktree_info);

        match merge_result {
            Ok(crate::views::worktree::MergeResult::Success) => {
                let _ = crate::views::worktree::delete_worktree(&repository_root, &worktree_info);
                self.kanban.move_task_to_done_by_id(task_id);
                let _ = self.save();
            }
            Ok(crate::views::worktree::MergeResult::Conflicts { conflicted_files }) => {
                let conflict_message = format!(
                    "Please resolve merge conflicts in the following files:\n{}\n\nThen commit the resolution.",
                    conflicted_files.join("\n")
                );
                if let Some(task_index) = self.kanban.find_task_index_by_id(task_id) {
                    if let Some(instance_id) = self.kanban.move_task_to_in_progress(task_index) {
                        self.instances
                            .send_input_to_instance(instance_id, &conflict_message);
                    }
                }
            }
            Err(_) => {}
        }
    }

    pub fn resolve_task_conflicts(&mut self, task_id: uuid::Uuid) {
        let task = match self.kanban.find_task_by_id(task_id) {
            Some(t) => t,
            None => return,
        };

        let worktree_info = match &task.worktree_info {
            Some(info) => info.clone(),
            None => return,
        };

        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return,
        };

        let repository_root = match crate::views::worktree::find_repository_root(&current_dir) {
            Ok(root) => root,
            Err(_) => return,
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

        if let Some(task_index) = self.kanban.find_task_index_by_id(task_id) {
            if let Some(instance_id) = self.kanban.move_task_to_in_progress(task_index) {
                self.instances
                    .send_input_to_instance(instance_id, &conflict_message);
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
