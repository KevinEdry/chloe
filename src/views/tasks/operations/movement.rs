use crate::views::settings::VcsCommand;
use crate::views::tasks::state::{TasksMode, TasksState};
use uuid::Uuid;

use crate::views::tasks::state::WorktreeSelectionOption;
use crate::views::worktree::WorktreeInfo;

impl TasksState {
    pub fn move_task_next(&mut self, vcs_command: &VcsCommand) {
        let can_move_next = self.kanban_selected_column < self.columns.len() - 1;
        if !can_move_next {
            return;
        }

        let Some(task_index) = self.kanban_selected_task else {
            return;
        };

        let is_entering_in_progress =
            self.kanban_selected_column == 0 && self.kanban_selected_column + 1 == 1;

        if is_entering_in_progress {
            let task = self.columns[self.kanban_selected_column]
                .tasks
                .get(task_index);

            let is_task_classifying = task.is_some_and(|task| task.is_classifying);
            if is_task_classifying {
                return;
            }

            let has_worktree = task.is_some_and(|task| task.worktree_info.is_some());
            if !has_worktree {
                if let Some(task) = task {
                    self.begin_worktree_selection_for_task(task.id, vcs_command);
                }
                return;
            }
        }

        let done_column_index = 3;
        let is_entering_done = self.kanban_selected_column + 1 == done_column_index;

        let Some(mut task) = self.columns[self.kanban_selected_column]
            .tasks
            .get(task_index)
            .cloned()
        else {
            return;
        };

        if is_entering_in_progress {
            self.pending_instance_creation = Some(task.id);
        }

        if is_entering_done && let Some(instance_id) = task.instance_id.take() {
            self.pending_instance_termination = Some(instance_id);
        }

        self.columns[self.kanban_selected_column]
            .tasks
            .remove(task_index);
        self.columns[self.kanban_selected_column + 1]
            .tasks
            .push(task);

        self.kanban_selected_column += 1;
        self.kanban_selected_task = Some(self.columns[self.kanban_selected_column].tasks.len() - 1);
    }

    pub fn move_task_previous(&mut self) {
        let can_move_previous = self.kanban_selected_column > 0;
        if !can_move_previous {
            return;
        }

        let Some(task_index) = self.kanban_selected_task else {
            return;
        };

        let Some(task) = self.columns[self.kanban_selected_column]
            .tasks
            .get(task_index)
        else {
            return;
        };

        let is_in_progress_column = self.kanban_selected_column == 1;
        let has_instance = task.instance_id.is_some();

        if is_in_progress_column && has_instance {
            self.mode = TasksMode::ConfirmMoveBack { task_id: task.id };
            return;
        }

        self.execute_move_task_previous();
    }

    pub fn execute_move_task_previous(&mut self) {
        let can_move_previous = self.kanban_selected_column > 0;
        if !can_move_previous {
            return;
        }

        let Some(task_index) = self.kanban_selected_task else {
            return;
        };

        let Some(task) = self.columns[self.kanban_selected_column]
            .tasks
            .get(task_index)
            .cloned()
        else {
            return;
        };

        if let Some(instance_id) = task.instance_id {
            self.pending_instance_termination = Some(instance_id);
        }

        if let Some(worktree_info) = task.worktree_info.clone() {
            let should_delete_worktree = worktree_info.auto_created;
            if should_delete_worktree {
                self.pending_worktree_deletion = Some(worktree_info);
            }
        }

        self.columns[self.kanban_selected_column]
            .tasks
            .remove(task_index);

        let mut task_without_instance = task;
        task_without_instance.instance_id = None;
        task_without_instance.worktree_info = None;

        self.columns[self.kanban_selected_column - 1]
            .tasks
            .push(task_without_instance);

        self.kanban_selected_column -= 1;
        self.kanban_selected_task = Some(self.columns[self.kanban_selected_column].tasks.len() - 1);
    }

    pub fn move_task_to_review_by_instance(&mut self, instance_id: Uuid) -> bool {
        let in_progress_column_index = 1;
        let review_column_index = 2;

        if self.columns.len() <= review_column_index {
            return false;
        }

        let in_progress_column = &mut self.columns[in_progress_column_index];
        let task_index = in_progress_column
            .tasks
            .iter()
            .position(|task| task.instance_id == Some(instance_id));

        let Some(task_index) = task_index else {
            return false;
        };

        let task = in_progress_column.tasks.remove(task_index);
        self.columns[review_column_index].tasks.push(task);

        true
    }

    pub fn move_task_to_done_by_id(&mut self, task_id: Uuid, vcs_command: &VcsCommand) -> bool {
        let review_column_index = 2;
        let done_column_index = 3;

        let Some(task_index) = self
            .columns
            .get(review_column_index)
            .and_then(|column| column.tasks.iter().position(|task| task.id == task_id))
        else {
            return false;
        };

        if self.columns.len() <= done_column_index {
            return false;
        }

        let task = &self.columns[review_column_index].tasks[task_index];

        let has_worktree_to_merge = task.worktree_info.is_some();
        if has_worktree_to_merge {
            let merge_result = Self::try_merge_worktree(task);

            match merge_result {
                Some(crate::views::worktree::MergeResult::Success) | None => {}
                Some(crate::views::worktree::MergeResult::Conflicts { conflicted_files }) => {
                    let conflict_message = format!(
                        "Merge conflicts detected in the following files:\n{}\n\nPlease resolve these conflicts and commit the changes.",
                        conflicted_files.join("\n")
                    );
                    self.pending_change_request = Some((task_id, conflict_message));
                    return false;
                }
            }
        }

        let task = &self.columns[review_column_index].tasks[task_index];
        Self::try_cleanup_worktree(task, vcs_command);

        let mut task = self.columns[review_column_index].tasks.remove(task_index);

        if let Some(instance_id) = task.instance_id.take() {
            self.pending_instance_termination = Some(instance_id);
        }

        self.columns[done_column_index].tasks.push(task);

        true
    }

    pub fn move_task_to_in_progress(
        &mut self,
        task_index: usize,
        vcs_command: &VcsCommand,
    ) -> Option<Uuid> {
        let review_column_index = 2;
        let in_progress_column_index = 1;

        if self.columns.len() <= review_column_index {
            return None;
        }

        if task_index >= self.columns[review_column_index].tasks.len() {
            return None;
        }

        let task = self.columns[review_column_index].tasks.get(task_index);

        let is_task_classifying = task.is_some_and(|task| task.is_classifying);
        if is_task_classifying {
            return None;
        }

        let has_worktree = task.is_some_and(|task| task.worktree_info.is_some());
        if !has_worktree {
            if let Some(task) = task {
                self.begin_worktree_selection_for_task(task.id, vcs_command);
            }
            return None;
        }

        let task = self.columns[review_column_index].tasks.remove(task_index);
        let instance_id = task.instance_id;
        self.columns[in_progress_column_index].tasks.push(task);

        let review_tasks_remaining = self.columns[review_column_index].tasks.len();
        if review_tasks_remaining == 0 {
            self.kanban_selected_task = None;
        } else if task_index >= review_tasks_remaining {
            self.kanban_selected_task = Some(review_tasks_remaining - 1);
        }

        instance_id
    }

    pub fn move_task_to_in_progress_by_id(
        &mut self,
        task_id: Uuid,
        vcs_command: &VcsCommand,
    ) -> Option<Uuid> {
        let in_progress_column_index = 1;

        let task_location = self
            .columns
            .iter()
            .enumerate()
            .find_map(|(column_index, column)| {
                column
                    .tasks
                    .iter()
                    .position(|task| task.id == task_id)
                    .map(|task_index| (column_index, task_index))
            });

        let (source_column_index, task_index) = task_location?;

        let is_already_in_progress = source_column_index == in_progress_column_index;
        if is_already_in_progress {
            let task = self.columns[in_progress_column_index]
                .tasks
                .iter()
                .find(|task| task.id == task_id);

            let has_worktree = task.is_some_and(|task| task.worktree_info.is_some());
            if !has_worktree {
                if let Some(task) = task {
                    self.begin_worktree_selection_for_task(task.id, vcs_command);
                }
                return None;
            }

            let instance_id = task.and_then(|task| task.instance_id);
            return instance_id;
        }

        let task = self.columns[source_column_index].tasks.get(task_index);

        let is_task_classifying = task.is_some_and(|task| task.is_classifying);
        if is_task_classifying {
            return None;
        }

        let has_worktree = task.is_some_and(|task| task.worktree_info.is_some());
        if !has_worktree {
            if let Some(task) = task {
                self.begin_worktree_selection_for_task(task.id, vcs_command);
            }
            return None;
        }

        let task = self.columns[source_column_index].tasks.remove(task_index);
        let instance_id = task.instance_id;
        self.pending_instance_creation = Some(task.id);
        self.columns[in_progress_column_index].tasks.push(task);

        instance_id
    }

    pub fn move_task_to_in_progress_with_worktree(
        &mut self,
        task_id: Uuid,
        worktree_option: WorktreeSelectionOption,
        vcs_command: &VcsCommand,
    ) -> Option<Uuid> {
        let in_progress_column_index = 1;

        let task_location = self
            .columns
            .iter()
            .enumerate()
            .find_map(|(column_index, column)| {
                column
                    .tasks
                    .iter()
                    .position(|task| task.id == task_id)
                    .map(|task_index| (column_index, task_index))
            });

        let (source_column_index, task_index) = task_location?;

        let task = self.columns[source_column_index].tasks.get(task_index);
        let is_task_classifying = task.is_some_and(|task| task.is_classifying);
        if is_task_classifying {
            return None;
        }

        let task_title = task.map(|task| task.title.clone())?;
        let worktree_info = match worktree_option {
            WorktreeSelectionOption::AutoCreate => {
                Self::create_worktree_for_new_task(&task_title, &task_id, vcs_command)
            }
            WorktreeSelectionOption::Existing {
                branch_name,
                worktree_path,
            } => Some(WorktreeInfo::new_existing(branch_name, worktree_path)),
        };

        let Some(worktree_info) = worktree_info else {
            self.error_message = Some("Failed to create worktree.".to_string());
            return None;
        };

        let mut task = self.columns[source_column_index].tasks.remove(task_index);
        task.worktree_info = Some(worktree_info);

        if source_column_index == in_progress_column_index {
            let instance_id = task.instance_id;
            self.columns[in_progress_column_index].tasks.push(task);
            return instance_id;
        }

        let instance_id = task.instance_id;
        self.pending_instance_creation = Some(task.id);
        self.columns[in_progress_column_index].tasks.push(task);

        if source_column_index == 2 {
            let review_tasks_remaining = self.columns[2].tasks.len();
            if review_tasks_remaining == 0 {
                self.kanban_selected_task = None;
            } else if task_index >= review_tasks_remaining {
                self.kanban_selected_task = Some(review_tasks_remaining - 1);
            }
        }

        instance_id
    }
}
