use crate::instance::InstanceState;
use crate::kanban::{KanbanState, TaskType};
use crate::roadmap::RoadmapState;
use crate::types::Config;
use crate::worktree::WorktreeTabState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tab {
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
    #[serde(skip)]
    pub config: Config,
    #[serde(skip)]
    pub showing_exit_confirmation: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: Tab::Kanban,
            kanban: KanbanState::new(),
            instances: InstanceState::new(),
            roadmap: RoadmapState::new(),
            worktree: WorktreeTabState::new(),
            config: Config::default(),
            showing_exit_confirmation: false,
        }
    }

    /// Load state from disk, or create new if it doesn't exist
    pub fn load_or_default() -> Self {
        match crate::persistence::storage::load_state() {
            Ok(mut app) => {
                app.config = Config::default();

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

    /// Save the current state to disk
    pub fn save(&self) -> crate::types::Result<()> {
        crate::persistence::storage::save_state(self)
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
    }

    pub fn next_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Kanban => Tab::Instances,
            Tab::Instances => Tab::Roadmap,
            Tab::Roadmap => Tab::Worktree,
            Tab::Worktree => Tab::Kanban,
        };
    }

    /// Auto-create instances for tasks in "In Progress" that don't have one
    pub fn sync_task_instances(&mut self) {
        if self.kanban.columns.len() < 2 {
            return;
        }

        let in_progress_column = &self.kanban.columns[1];
        let tasks_needing_instances: Vec<(
            uuid::Uuid,
            String,
            String,
            Option<std::path::PathBuf>,
        )> = in_progress_column
            .tasks
            .iter()
            .filter(|task| task.instance_id.is_none())
            .map(|task| {
                let worktree_path = task
                    .worktree_info
                    .as_ref()
                    .map(|info| info.worktree_path.clone());
                (task.id, task.title.clone(), task.description.clone(), worktree_path)
            })
            .collect();

        for (task_id, task_title, task_description, working_directory) in tasks_needing_instances {
            let instance_id = self.instances.create_pane_for_task(
                &task_title,
                &task_description,
                working_directory,
                24,
                80,
            );
            self.kanban.link_task_to_instance(task_id, instance_id);
        }
    }

    /// Jump to a task's instance in the Instances tab
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
                    24,
                    80,
                );
                self.kanban.link_task_to_instance(task_id, instance_id);
                self.active_tab = Tab::Instances;
                return true;
            }
        }
        false
    }

    /// Get the Claude state for an instance by its ID
    pub fn get_instance_claude_state(
        &self,
        instance_id: uuid::Uuid,
    ) -> Option<crate::instance::ClaudeState> {
        self.instances
            .panes
            .iter()
            .find(|pane| pane.id == instance_id)
            .map(|pane| pane.claude_state)
    }

    /// Auto-transition tasks from In Progress to Review when Claude Code completes
    pub fn auto_transition_completed_tasks(&mut self) {
        let completed_instances: Vec<uuid::Uuid> = self
            .instances
            .panes
            .iter()
            .filter(|pane| pane.claude_state == crate::instance::ClaudeState::Done)
            .map(|pane| pane.id)
            .collect();

        for instance_id in completed_instances {
            self.kanban.move_task_to_review_by_instance(instance_id);
        }
    }

    /// Get the output buffer for an instance by its ID
    pub fn get_instance_output(&self, instance_id: uuid::Uuid) -> Option<&str> {
        self.instances
            .panes
            .iter()
            .find(|pane| pane.id == instance_id)
            .map(|pane| pane.output_buffer.as_str())
    }

    /// Open the project in the default IDE for a task in Review column
    pub fn open_task_in_ide(&self, task_idx: usize) {
        let review_column_index = 2;
        if let Some(task) = self
            .kanban
            .columns
            .get(review_column_index)
            .and_then(|col| col.tasks.get(task_idx))
        {
            if let Some(instance_id) = task.instance_id {
                if let Some(pane) = self.instances.panes.iter().find(|p| p.id == instance_id) {
                    let working_dir = &pane.working_directory;
                    let _ = std::process::Command::new("open").arg(working_dir).spawn();
                }
            }
        }
    }

    /// Switch to instances tab and focus the instance for a task in Review column
    pub fn switch_to_task_instance(&mut self, task_idx: usize) -> bool {
        let review_column_index = 2;
        if let Some(task) = self
            .kanban
            .columns
            .get(review_column_index)
            .and_then(|col| col.tasks.get(task_idx))
        {
            if let Some(instance_id) = task.instance_id {
                self.active_tab = Tab::Instances;
                return self.instances.select_pane_by_id(instance_id);
            }
        }
        false
    }

    /// Convert a roadmap item to a kanban task in Planning column
    pub fn convert_roadmap_item_to_task(&mut self, item_index: usize) {
        if let Some(item) = self.roadmap.items.get(item_index) {
            let title = item.title.clone();
            let description = item.description.clone();
            self.kanban
                .add_task_to_planning(title, description, TaskType::Task);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
