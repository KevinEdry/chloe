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

    /// Start AI classification of user input
    pub fn start_classification(&mut self, raw_input: String) {
        let request = ClassificationRequest::spawn(raw_input.clone());
        self.classification_request = Some(request);
        self.mode = super::KanbanMode::ClassifyingTask { raw_input };
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
                        if let super::KanbanMode::ClassifyingTask { raw_input } = &self.mode {
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

        self.add_task_to_planning(classified.title, classified.description, task_type);
        self.mode = super::KanbanMode::Normal;
    }

    /// Fallback to manual entry if classification fails
    pub fn fallback_to_manual(&mut self, raw_input: String) {
        self.add_task_to_planning(raw_input, String::new(), TaskType::Task);
        self.mode = super::KanbanMode::Normal;
    }

    /// Edit an existing task
    pub fn edit_task(&mut self, task_idx: usize, title: String, description: String) {
        if let Some(task) = self.selected_column_mut().tasks.get_mut(task_idx) {
            task.title = title;
            task.description = description;
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

        let task = match self.columns[self.selected_column]
            .tasks
            .get(task_index)
            .cloned()
        {
            Some(t) => t,
            None => return,
        };

        self.columns[self.selected_column].tasks.remove(task_index);
        self.columns[self.selected_column + 1].tasks.push(task);

        self.selected_column += 1;
        self.selected_task = Some(self.columns[self.selected_column].tasks.len() - 1);
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

        self.columns[self.selected_column].tasks.remove(task_index);

        let mut task_without_instance = task;
        task_without_instance.instance_id = None;

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
}
