use super::state::{FocusMode, FocusPanel, FocusState};
use crate::views::kanban::{Column, Task};

pub struct TaskReference<'a> {
    pub task: &'a Task,
    pub column_name: &'a str,
    pub column_index: usize,
}

impl FocusState {
    pub fn select_next(&mut self, active_count: usize, done_count: usize) {
        match self.focused_panel {
            FocusPanel::ActiveTasks => {
                if active_count == 0 {
                    return;
                }
                self.active_selected_index = (self.active_selected_index + 1).min(active_count - 1);
            }
            FocusPanel::DoneTasks => {
                if done_count == 0 {
                    return;
                }
                self.done_selected_index = (self.done_selected_index + 1).min(done_count - 1);
            }
        }
    }

    pub fn select_previous(&mut self) {
        match self.focused_panel {
            FocusPanel::ActiveTasks => {
                self.active_selected_index = self.active_selected_index.saturating_sub(1);
            }
            FocusPanel::DoneTasks => {
                self.done_selected_index = self.done_selected_index.saturating_sub(1);
            }
        }
    }

    pub fn switch_to_done_panel(&mut self, done_count: usize) {
        if done_count == 0 {
            return;
        }
        self.focused_panel = FocusPanel::DoneTasks;
        if self.done_selected_index >= done_count {
            self.done_selected_index = done_count.saturating_sub(1);
        }
    }

    pub fn switch_to_active_panel(&mut self, active_count: usize) {
        if active_count == 0 {
            return;
        }
        self.focused_panel = FocusPanel::ActiveTasks;
        if self.active_selected_index >= active_count {
            self.active_selected_index = active_count.saturating_sub(1);
        }
    }

    pub fn enter_terminal_mode(&mut self) {
        self.mode = FocusMode::TerminalFocused;
    }

    pub fn exit_terminal_mode(&mut self) {
        self.mode = FocusMode::Normal;
    }

    pub fn clamp_selection(&mut self, active_count: usize, done_count: usize) {
        if active_count == 0 {
            self.active_selected_index = 0;
        } else if self.active_selected_index >= active_count {
            self.active_selected_index = active_count - 1;
        }

        if done_count == 0 {
            self.done_selected_index = 0;
        } else if self.done_selected_index >= done_count {
            self.done_selected_index = done_count - 1;
        }

        let focused_panel_empty = match self.focused_panel {
            FocusPanel::ActiveTasks => active_count == 0,
            FocusPanel::DoneTasks => done_count == 0,
        };

        if focused_panel_empty {
            if active_count > 0 {
                self.focused_panel = FocusPanel::ActiveTasks;
            } else if done_count > 0 {
                self.focused_panel = FocusPanel::DoneTasks;
            }
        }
    }
}

pub fn get_active_task_count(columns: &[Column]) -> usize {
    columns
        .iter()
        .take(3)
        .map(|column| column.tasks.len())
        .sum()
}

pub fn get_done_task_count(columns: &[Column]) -> usize {
    columns.get(3).map(|column| column.tasks.len()).unwrap_or(0)
}

pub fn get_active_tasks(columns: &[Column]) -> Vec<TaskReference<'_>> {
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

    tasks
}

pub fn get_done_tasks(columns: &[Column]) -> Vec<TaskReference<'_>> {
    let mut tasks = Vec::new();
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
