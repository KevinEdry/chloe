use super::state::{FocusMode, FocusState};
use crate::views::kanban::{Column, Task};

pub struct TaskReference<'a> {
    pub task: &'a Task,
    pub column_name: &'a str,
    pub column_index: usize,
}

impl FocusState {
    pub fn select_next(&mut self, total_tasks: usize) {
        if total_tasks == 0 {
            return;
        }
        self.selected_index = (self.selected_index + 1).min(total_tasks - 1);
    }

    pub fn select_previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn enter_terminal_mode(&mut self) {
        self.mode = FocusMode::TerminalFocused;
    }

    pub fn exit_terminal_mode(&mut self) {
        self.mode = FocusMode::Normal;
    }

    pub fn clamp_selection(&mut self, total_tasks: usize) {
        if total_tasks == 0 {
            self.selected_index = 0;
        } else if self.selected_index >= total_tasks {
            self.selected_index = total_tasks - 1;
        }
    }
}

pub fn get_ordered_tasks(columns: &[Column]) -> Vec<TaskReference<'_>> {
    let mut tasks = Vec::new();

    let active_column_indices = [0, 1, 2];
    for &column_index in &active_column_indices {
        if let Some(column) = columns.get(column_index) {
            for task in &column.tasks {
                tasks.push(TaskReference {
                    task,
                    column_name: &column.name,
                    column_index,
                });
            }
        }
    }

    let done_column_index = 3;
    if let Some(column) = columns.get(done_column_index) {
        for task in &column.tasks {
            tasks.push(TaskReference {
                task,
                column_name: &column.name,
                column_index: done_column_index,
            });
        }
    }

    tasks
}

pub fn get_active_task_count(columns: &[Column]) -> usize {
    columns
        .iter()
        .take(3)
        .map(|column| column.tasks.len())
        .sum()
}
