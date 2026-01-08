use super::ai_classifier::{ClassificationRequest, ClassifiedTask};
use super::{KanbanState, Task, TaskType};

impl KanbanState {
    /// Add a new task to the Planning column (index 0)
    pub fn add_task_to_planning(
        &mut self,
        title: String,
        description: String,
        task_type: TaskType,
    ) {
        let task = Task::new(title, description, task_type);
        self.columns[0].tasks.push(task);
        self.selected_column = 0;
        self.selected_task = Some(self.columns[0].tasks.len() - 1);
    }

    /// Start AI classification of user input for a new task
    pub fn start_classification(&mut self, raw_input: String) {
        self.start_classification_internal(raw_input, None);
    }

    /// Start AI classification of user input for editing an existing task
    pub fn start_classification_for_edit(&mut self, raw_input: String, task_idx: usize) {
        self.start_classification_internal(raw_input, Some(task_idx));
    }

    /// Internal classification starter
    fn start_classification_internal(&mut self, raw_input: String, edit_task_idx: Option<usize>) {
        let request = ClassificationRequest::spawn(raw_input.clone());
        self.classification_request = Some(request);
        self.mode = super::KanbanMode::ClassifyingTask {
            raw_input,
            edit_task_idx,
        };
    }

    /// Poll classification result (called every event cycle)
    pub fn poll_classification(&mut self) {
        if let Some(request) = &self.classification_request {
            if let Some(result) = request.try_recv() {
                self.classification_request = None;

                match result {
                    Ok(classified) => {
                        self.apply_classification(classified);
                    }
                    Err(_) => {
                        if let super::KanbanMode::ClassifyingTask { raw_input, .. } = &self.mode {
                            self.fallback_to_manual(raw_input.clone());
                        }
                    }
                }
            }
        }
    }

    /// Cancel ongoing classification
    pub fn cancel_classification(&mut self) {
        self.classification_request = None;
        self.mode = super::KanbanMode::Normal;
    }

    /// Apply classification result
    pub fn apply_classification(&mut self, classified: ClassifiedTask) {
        let task_type = match classified.task_type.to_lowercase().as_str() {
            "feature" => TaskType::Feature,
            "bug" => TaskType::Bug,
            "chore" => TaskType::Chore,
            _ => TaskType::Task,
        };

        if let super::KanbanMode::ClassifyingTask { edit_task_idx, .. } = self.mode {
            if let Some(task_idx) = edit_task_idx {
                self.edit_task_with_classification(
                    task_idx,
                    classified.title,
                    classified.description,
                    task_type,
                );
            } else {
                self.add_task_to_planning(classified.title, classified.description, task_type);
            }
        }

        self.mode = super::KanbanMode::Normal;
    }

    /// Fallback to manual entry if classification fails
    pub fn fallback_to_manual(&mut self, raw_input: String) {
        if let super::KanbanMode::ClassifyingTask { edit_task_idx, .. } = self.mode {
            if let Some(task_idx) = edit_task_idx {
                self.edit_task(task_idx, raw_input, String::new());
            } else {
                self.add_task_to_planning(raw_input, String::new(), TaskType::Task);
            }
        }
        self.mode = super::KanbanMode::Normal;
    }

    /// Edit an existing task
    pub fn edit_task(&mut self, task_idx: usize, title: String, description: String) {
        if let Some(task) = self.selected_column_mut().tasks.get_mut(task_idx) {
            task.title = title;
            task.description = description;
        }
    }

    /// Edit an existing task with AI classification results
    fn edit_task_with_classification(
        &mut self,
        task_idx: usize,
        title: String,
        description: String,
        task_type: TaskType,
    ) {
        if let Some(task) = self.selected_column_mut().tasks.get_mut(task_idx) {
            task.title = title;
            task.description = description;
            task.task_type = task_type;
        }
    }

    /// Delete the selected task
    pub fn delete_task(&mut self, task_idx: usize) {
        let column = self.selected_column_mut();
        if task_idx < column.tasks.len() {
            column.tasks.remove(task_idx);

            if column.tasks.is_empty() {
                self.selected_task = None;
            } else if task_idx >= column.tasks.len() {
                self.selected_task = Some(column.tasks.len() - 1);
            }
        }
    }

    /// Move task to the next column
    pub fn move_task_next(&mut self) {
        let can_move_next = self.selected_column < self.columns.len() - 1;
        if !can_move_next {
            return;
        }

        let task_index = match self.selected_task {
            Some(idx) => idx,
            None => return,
        };

        let is_entering_in_progress = self.selected_column == 0 && self.selected_column + 1 == 1;
        if is_entering_in_progress {
            self.try_create_worktree_for_task(task_index);
        }

        let task = match self.columns[self.selected_column]
            .tasks
            .get(task_index)
            .cloned()
        {
            Some(t) => t,
            None => return,
        };

        if is_entering_in_progress {
            self.pending_instance_creation = Some(task.id);
        }

        self.columns[self.selected_column].tasks.remove(task_index);
        self.columns[self.selected_column + 1].tasks.push(task);

        self.selected_column += 1;
        self.selected_task = Some(self.columns[self.selected_column].tasks.len() - 1);
    }

    fn try_create_worktree_for_task(&mut self, task_index: usize) {
        let task = match self.columns[self.selected_column].tasks.get_mut(task_index) {
            Some(t) => t,
            None => return,
        };

        let already_has_worktree = task.worktree_info.is_some();
        if already_has_worktree {
            return;
        }

        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return,
        };

        let repository_root = match crate::worktree::find_repository_root(&current_dir) {
            Ok(root) => root,
            Err(_) => return,
        };

        let worktree_info =
            match crate::worktree::create_worktree(&repository_root, &task.title, &task.id) {
                Ok(info) => info,
                Err(_) => return,
            };

        task.worktree_info = Some(worktree_info);
    }

    /// Move task to the previous column
    pub fn move_task_previous(&mut self) {
        let can_move_previous = self.selected_column > 0;
        if !can_move_previous {
            return;
        }

        let task_index = match self.selected_task {
            Some(idx) => idx,
            None => return,
        };

        let task = match self.columns[self.selected_column].tasks.get(task_index) {
            Some(t) => t,
            None => return,
        };

        let is_in_progress_column = self.selected_column == 1;
        let has_instance = task.instance_id.is_some();

        if is_in_progress_column && has_instance {
            self.mode = super::KanbanMode::ConfirmMoveBack {
                task_idx: task_index,
            };
            return;
        }

        self.execute_move_task_previous();
    }

    /// Execute the actual task movement (called after confirmation if needed)
    pub fn execute_move_task_previous(&mut self) {
        let can_move_previous = self.selected_column > 0;
        if !can_move_previous {
            return;
        }

        let task_index = match self.selected_task {
            Some(idx) => idx,
            None => return,
        };

        let task = match self.columns[self.selected_column]
            .tasks
            .get(task_index)
            .cloned()
        {
            Some(t) => t,
            None => return,
        };

        if let Some(instance_id) = task.instance_id {
            self.pending_instance_termination = Some(instance_id);
        }

        if let Some(worktree_info) = task.worktree_info.clone() {
            self.pending_worktree_deletion = Some(worktree_info);
        }

        self.columns[self.selected_column].tasks.remove(task_index);

        let mut task_without_instance = task;
        task_without_instance.instance_id = None;
        task_without_instance.worktree_info = None;

        self.columns[self.selected_column - 1]
            .tasks
            .push(task_without_instance);

        self.selected_column -= 1;
        self.selected_task = Some(self.columns[self.selected_column].tasks.len() - 1);
    }

    /// Navigate to the next column
    pub fn next_column(&mut self) {
        if self.selected_column < self.columns.len() - 1 {
            self.selected_column += 1;

            if !self.columns[self.selected_column].tasks.is_empty() {
                self.selected_task = Some(0);
            } else {
                self.selected_task = None;
            }
        }
    }

    /// Navigate to the previous column
    pub fn previous_column(&mut self) {
        if self.selected_column > 0 {
            self.selected_column -= 1;

            if !self.columns[self.selected_column].tasks.is_empty() {
                self.selected_task = Some(0);
            } else {
                self.selected_task = None;
            }
        }
    }

    /// Navigate to the next task in the current column
    pub fn next_task(&mut self) {
        let column = &self.columns[self.selected_column];
        if column.tasks.is_empty() {
            self.selected_task = None;
            return;
        }

        self.selected_task = Some(match self.selected_task {
            Some(idx) if idx < column.tasks.len() - 1 => idx + 1,
            Some(idx) => idx,
            None => 0,
        });
    }

    /// Navigate to the previous task in the current column
    pub fn previous_task(&mut self) {
        let column = &self.columns[self.selected_column];
        if column.tasks.is_empty() {
            self.selected_task = None;
            return;
        }

        self.selected_task = Some(match self.selected_task {
            Some(idx) if idx > 0 => idx - 1,
            Some(idx) => idx,
            None => 0,
        });
    }

    /// Toggle pause state of the selected task in In Progress column
    /// Returns the instance_id and desired pause state if the task has an instance
    pub fn toggle_pause(&mut self) -> Option<(uuid::Uuid, bool)> {
        let is_in_progress_column = self.selected_column == 1;
        if !is_in_progress_column {
            return None;
        }

        let task_index = self.selected_task?;
        let task = self.columns[self.selected_column]
            .tasks
            .get_mut(task_index)?;
        let instance_id = task.instance_id?;

        task.is_paused = !task.is_paused;
        Some((instance_id, task.is_paused))
    }

    /// Move a task from In Progress to Review by instance_id
    /// Returns true if the task was found and moved
    pub fn move_task_to_review_by_instance(&mut self, instance_id: uuid::Uuid) -> bool {
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

        let task_index = match task_index {
            Some(idx) => idx,
            None => return false,
        };

        let task = in_progress_column.tasks.remove(task_index);
        self.columns[review_column_index].tasks.push(task);

        true
    }

    /// Move a task from Review to Done by task index in Review column
    /// This will attempt to merge the worktree branch to main first
    /// If there are conflicts, the task will be moved back to In Progress
    pub fn move_task_to_done(&mut self, task_idx: usize) {
        let review_column_index = 2;
        let done_column_index = 3;

        if self.columns.len() <= done_column_index {
            return;
        }

        if task_idx >= self.columns[review_column_index].tasks.len() {
            return;
        }

        let task = &self.columns[review_column_index].tasks[task_idx];

        let has_worktree_to_merge = task.worktree_info.is_some();
        if has_worktree_to_merge {
            let merge_result = self.try_merge_worktree(task);

            match merge_result {
                Some(crate::worktree::MergeResult::Success) => {}
                Some(crate::worktree::MergeResult::Conflicts { conflicted_files }) => {
                    let conflict_message = format!(
                        "Merge conflicts detected in the following files:\n{}\n\nPlease resolve these conflicts and commit the changes.",
                        conflicted_files.join("\n")
                    );
                    self.pending_change_request = Some((task_idx, conflict_message));
                    return;
                }
                None => {}
            }
        }

        let task = &self.columns[review_column_index].tasks[task_idx];
        self.try_cleanup_worktree(task);

        let task = self.columns[review_column_index].tasks.remove(task_idx);
        self.columns[done_column_index].tasks.push(task);

        let review_tasks_remaining = self.columns[review_column_index].tasks.len();
        if review_tasks_remaining == 0 {
            self.selected_task = None;
        } else if task_idx >= review_tasks_remaining {
            self.selected_task = Some(review_tasks_remaining - 1);
        }
    }

    fn try_merge_worktree(
        &self,
        task: &super::state::Task,
    ) -> Option<crate::worktree::MergeResult> {
        let worktree_info = task.worktree_info.as_ref()?;

        let was_auto_created = worktree_info.auto_created;
        if !was_auto_created {
            return None;
        }

        let current_dir = std::env::current_dir().ok()?;
        let repository_root = crate::worktree::find_repository_root(&current_dir).ok()?;

        crate::worktree::merge_worktree_to_main(&repository_root, worktree_info).ok()
    }

    fn try_cleanup_worktree(&self, task: &super::state::Task) {
        let worktree_info = match &task.worktree_info {
            Some(info) => info,
            None => return,
        };

        let was_auto_created = worktree_info.auto_created;
        if !was_auto_created {
            return;
        }

        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return,
        };

        let repository_root = match crate::worktree::find_repository_root(&current_dir) {
            Ok(root) => root,
            Err(_) => return,
        };

        let _ = crate::worktree::delete_worktree(&repository_root, worktree_info);
    }

    /// Move a task from Review back to In Progress by task index in Review column
    /// Returns the instance_id if the task has one
    pub fn move_task_to_in_progress(&mut self, task_idx: usize) -> Option<uuid::Uuid> {
        let review_column_index = 2;
        let in_progress_column_index = 1;

        if self.columns.len() <= review_column_index {
            return None;
        }

        if task_idx >= self.columns[review_column_index].tasks.len() {
            return None;
        }

        let task = self.columns[review_column_index].tasks.remove(task_idx);
        let instance_id = task.instance_id;
        self.columns[in_progress_column_index].tasks.push(task);

        let review_tasks_remaining = self.columns[review_column_index].tasks.len();
        if review_tasks_remaining == 0 {
            self.selected_task = None;
        } else if task_idx >= review_tasks_remaining {
            self.selected_task = Some(review_tasks_remaining - 1);
        }

        instance_id
    }
}
