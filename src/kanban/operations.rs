use super::{Column, KanbanState, Task};

impl KanbanState {
    /// Add a new task to the selected column
    pub fn add_task(&mut self, title: String, description: String) {
        let task = Task::new(title, description);
        self.selected_column_mut().tasks.push(task);
        // Select the newly added task
        self.selected_task = Some(self.selected_column().tasks.len() - 1);
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
            // Adjust selection
            if column.tasks.is_empty() {
                self.selected_task = None;
            } else if task_idx >= column.tasks.len() {
                self.selected_task = Some(column.tasks.len() - 1);
            }
        }
    }

    /// Move task to the next column
    pub fn move_task_next(&mut self) {
        if self.selected_column < self.columns.len() - 1 {
            if let Some(task_idx) = self.selected_task {
                if let Some(task) = self.columns[self.selected_column].tasks.get(task_idx).cloned() {
                    // Remove from current column
                    self.columns[self.selected_column].tasks.remove(task_idx);
                    // Add to next column
                    self.columns[self.selected_column + 1].tasks.push(task);

                    // Update selection
                    self.selected_column += 1;
                    self.selected_task = Some(self.columns[self.selected_column].tasks.len() - 1);
                }
            }
        }
    }

    /// Move task to the previous column
    pub fn move_task_previous(&mut self) {
        if self.selected_column > 0 {
            if let Some(task_idx) = self.selected_task {
                if let Some(task) = self.columns[self.selected_column].tasks.get(task_idx).cloned() {
                    // Remove from current column
                    self.columns[self.selected_column].tasks.remove(task_idx);
                    // Add to previous column
                    self.columns[self.selected_column - 1].tasks.push(task);

                    // Update selection
                    self.selected_column -= 1;
                    self.selected_task = Some(self.columns[self.selected_column].tasks.len() - 1);
                }
            }
        }
    }

    /// Navigate to the next column
    pub fn next_column(&mut self) {
        if self.selected_column < self.columns.len() - 1 {
            self.selected_column += 1;
            // Select first task in new column if available
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
            // Select first task in new column if available
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
}
