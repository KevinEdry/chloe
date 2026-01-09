use super::ai_classifier::{ClassificationRequest, ClassifiedTask};
use super::{KanbanState, Task, TaskType};

impl KanbanState {
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

    pub fn start_classification(&mut self, raw_input: String) {
        self.start_classification_internal(raw_input, None);
    }

    pub fn start_classification_for_edit(&mut self, raw_input: String, task_index: usize) {
        self.start_classification_internal(raw_input, Some(task_index));
    }

    fn start_classification_internal(&mut self, raw_input: String, edit_task_index: Option<usize>) {
        let request = ClassificationRequest::spawn(raw_input.clone());
        self.classification_request = Some(request);
        self.mode = super::KanbanMode::ClassifyingTask {
            raw_input,
            edit_task_index,
        };
    }

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

    pub fn cancel_classification(&mut self) {
        self.classification_request = None;
        self.mode = super::KanbanMode::Normal;
    }

    pub fn apply_classification(&mut self, classified: ClassifiedTask) {
        let task_type = match classified.task_type.to_lowercase().as_str() {
            "feature" => TaskType::Feature,
            "bug" => TaskType::Bug,
            "chore" => TaskType::Chore,
            _ => TaskType::Task,
        };

        if let super::KanbanMode::ClassifyingTask {
            edit_task_index, ..
        } = self.mode
        {
            if let Some(task_index) = edit_task_index {
                self.edit_task_with_classification(
                    task_index,
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

    pub fn fallback_to_manual(&mut self, raw_input: String) {
        if let super::KanbanMode::ClassifyingTask {
            edit_task_index, ..
        } = self.mode
        {
            if let Some(task_index) = edit_task_index {
                self.edit_task(task_index, raw_input, String::new());
            } else {
                self.add_task_to_planning(raw_input, String::new(), TaskType::Task);
            }
        }
        self.mode = super::KanbanMode::Normal;
    }

    pub fn edit_task(&mut self, task_index: usize, title: String, description: String) {
        if let Some(task) = self.selected_column_mut().tasks.get_mut(task_index) {
            task.title = title;
            task.description = description;
        }
    }

    fn edit_task_with_classification(
        &mut self,
        task_index: usize,
        title: String,
        description: String,
        task_type: TaskType,
    ) {
        if let Some(task) = self.selected_column_mut().tasks.get_mut(task_index) {
            task.title = title;
            task.description = description;
            task.task_type = task_type;
        }
    }

    pub fn delete_task(&mut self, task_index: usize) {
        let column = self.selected_column_mut();
        if task_index < column.tasks.len() {
            column.tasks.remove(task_index);

            if column.tasks.is_empty() {
                self.selected_task = None;
            } else if task_index >= column.tasks.len() {
                self.selected_task = Some(column.tasks.len() - 1);
            }
        }
    }

    pub fn move_task_next(&mut self) {
        let can_move_next = self.selected_column < self.columns.len() - 1;
        if !can_move_next {
            return;
        }

        let task_index = match self.selected_task {
            Some(i) => i,
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

        let repository_root = match crate::views::worktree::find_repository_root(&current_dir) {
            Ok(root) => root,
            Err(_) => return,
        };

        let worktree_info = match crate::views::worktree::create_worktree(
            &repository_root,
            &task.title,
            &task.id,
        ) {
            Ok(info) => info,
            Err(_) => return,
        };

        task.worktree_info = Some(worktree_info);
    }

    pub fn move_task_previous(&mut self) {
        let can_move_previous = self.selected_column > 0;
        if !can_move_previous {
            return;
        }

        let task_index = match self.selected_task {
            Some(i) => i,
            None => return,
        };

        let task = match self.columns[self.selected_column].tasks.get(task_index) {
            Some(t) => t,
            None => return,
        };

        let is_in_progress_column = self.selected_column == 1;
        let has_instance = task.instance_id.is_some();

        if is_in_progress_column && has_instance {
            self.mode = super::KanbanMode::ConfirmMoveBack { task_index };
            return;
        }

        self.execute_move_task_previous();
    }

    pub fn execute_move_task_previous(&mut self) {
        let can_move_previous = self.selected_column > 0;
        if !can_move_previous {
            return;
        }

        let task_index = match self.selected_task {
            Some(i) => i,
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

    pub fn next_task(&mut self) {
        let column = &self.columns[self.selected_column];
        if column.tasks.is_empty() {
            self.selected_task = None;
            return;
        }

        self.selected_task = Some(match self.selected_task {
            Some(i) if i < column.tasks.len() - 1 => i + 1,
            Some(i) => i,
            None => 0,
        });
    }

    pub fn previous_task(&mut self) {
        let column = &self.columns[self.selected_column];
        if column.tasks.is_empty() {
            self.selected_task = None;
            return;
        }

        self.selected_task = Some(match self.selected_task {
            Some(i) if i > 0 => i - 1,
            Some(i) => i,
            None => 0,
        });
    }

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
            Some(i) => i,
            None => return false,
        };

        let task = in_progress_column.tasks.remove(task_index);
        self.columns[review_column_index].tasks.push(task);

        true
    }

    pub fn move_task_to_done(&mut self, task_index: usize) {
        let review_column_index = 2;
        let done_column_index = 3;

        if self.columns.len() <= done_column_index {
            return;
        }

        if task_index >= self.columns[review_column_index].tasks.len() {
            return;
        }

        let task = &self.columns[review_column_index].tasks[task_index];

        let has_worktree_to_merge = task.worktree_info.is_some();
        if has_worktree_to_merge {
            let merge_result = self.try_merge_worktree(task);

            match merge_result {
                Some(crate::views::worktree::MergeResult::Success) => {}
                Some(crate::views::worktree::MergeResult::Conflicts { conflicted_files }) => {
                    let conflict_message = format!(
                        "Merge conflicts detected in the following files:\n{}\n\nPlease resolve these conflicts and commit the changes.",
                        conflicted_files.join("\n")
                    );
                    self.pending_change_request = Some((task_index, conflict_message));
                    return;
                }
                None => {}
            }
        }

        let task = &self.columns[review_column_index].tasks[task_index];
        self.try_cleanup_worktree(task);

        let task = self.columns[review_column_index].tasks.remove(task_index);
        self.columns[done_column_index].tasks.push(task);

        let review_tasks_remaining = self.columns[review_column_index].tasks.len();
        if review_tasks_remaining == 0 {
            self.selected_task = None;
        } else if task_index >= review_tasks_remaining {
            self.selected_task = Some(review_tasks_remaining - 1);
        }
    }

    fn try_merge_worktree(
        &self,
        task: &super::state::Task,
    ) -> Option<crate::views::worktree::MergeResult> {
        let worktree_info = task.worktree_info.as_ref()?;

        let was_auto_created = worktree_info.auto_created;
        if !was_auto_created {
            return None;
        }

        let current_dir = std::env::current_dir().ok()?;
        let repository_root = crate::views::worktree::find_repository_root(&current_dir).ok()?;

        crate::views::worktree::merge_worktree_to_main(&repository_root, worktree_info).ok()
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

        let repository_root = match crate::views::worktree::find_repository_root(&current_dir) {
            Ok(root) => root,
            Err(_) => return,
        };

        let _ = crate::views::worktree::delete_worktree(&repository_root, worktree_info);
    }

    pub fn move_task_to_in_progress(&mut self, task_index: usize) -> Option<uuid::Uuid> {
        let review_column_index = 2;
        let in_progress_column_index = 1;

        if self.columns.len() <= review_column_index {
            return None;
        }

        if task_index >= self.columns[review_column_index].tasks.len() {
            return None;
        }

        let task = self.columns[review_column_index].tasks.remove(task_index);
        let instance_id = task.instance_id;
        self.columns[in_progress_column_index].tasks.push(task);

        let review_tasks_remaining = self.columns[review_column_index].tasks.len();
        if review_tasks_remaining == 0 {
            self.selected_task = None;
        } else if task_index >= review_tasks_remaining {
            self.selected_task = Some(review_tasks_remaining - 1);
        }

        instance_id
    }

    pub fn move_task_to_in_progress_by_id(&mut self, task_id: uuid::Uuid) -> bool {
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

        let (source_column_index, task_index) = match task_location {
            Some(location) => location,
            None => return false,
        };

        let is_already_in_progress = source_column_index == in_progress_column_index;
        if is_already_in_progress {
            return true;
        }

        let task = self.columns[source_column_index].tasks.remove(task_index);
        self.columns[in_progress_column_index].tasks.push(task);

        true
    }

    pub fn update_task_title_by_id(&mut self, task_id: uuid::Uuid, new_title: String) -> bool {
        for column in &mut self.columns {
            for task in &mut column.tasks {
                if task.id == task_id {
                    task.title = new_title;
                    return true;
                }
            }
        }
        false
    }

    pub fn delete_task_by_id(&mut self, task_id: uuid::Uuid) -> Option<uuid::Uuid> {
        for column in &mut self.columns {
            if let Some(position) = column.tasks.iter().position(|task| task.id == task_id) {
                let task = column.tasks.remove(position);
                return task.instance_id;
            }
        }
        None
    }
}
