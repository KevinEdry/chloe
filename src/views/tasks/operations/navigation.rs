use super::queries::{get_active_task_count, get_done_task_count};
use crate::views::tasks::state::{FocusPanel, TasksMode, TasksState};

impl TasksState {
    pub fn next_column(&mut self) {
        if self.kanban_selected_column < self.columns.len() - 1 {
            self.kanban_selected_column += 1;

            if self.columns[self.kanban_selected_column].tasks.is_empty() {
                self.kanban_selected_task = None;
            } else {
                self.kanban_selected_task = Some(0);
            }
        }
    }

    pub fn previous_column(&mut self) {
        if self.kanban_selected_column > 0 {
            self.kanban_selected_column -= 1;

            if self.columns[self.kanban_selected_column].tasks.is_empty() {
                self.kanban_selected_task = None;
            } else {
                self.kanban_selected_task = Some(0);
            }
        }
    }

    pub fn next_task(&mut self) {
        let column = &self.columns[self.kanban_selected_column];
        if column.tasks.is_empty() {
            self.kanban_selected_task = None;
            return;
        }

        self.kanban_selected_task = Some(match self.kanban_selected_task {
            Some(index) if index < column.tasks.len() - 1 => index + 1,
            Some(index) => index,
            None => 0,
        });
    }

    pub fn previous_task(&mut self) {
        let column = &self.columns[self.kanban_selected_column];
        if column.tasks.is_empty() {
            self.kanban_selected_task = None;
            return;
        }

        self.kanban_selected_task = Some(match self.kanban_selected_task {
            Some(index) if index > 0 => index - 1,
            Some(index) => index,
            None => 0,
        });
    }

    pub fn focus_select_next(&mut self) {
        let active_count = get_active_task_count(&self.columns);
        let done_count = get_done_task_count(&self.columns);

        match self.focus_panel {
            FocusPanel::ActiveTasks => {
                if active_count == 0 {
                    return;
                }
                let is_at_last_active_task = self.focus_active_index >= active_count - 1;
                if is_at_last_active_task && done_count > 0 {
                    self.focus_panel = FocusPanel::DoneTasks;
                    self.focus_done_index = 0;
                } else {
                    self.focus_active_index = (self.focus_active_index + 1).min(active_count - 1);
                }
            }
            FocusPanel::DoneTasks => {
                if done_count == 0 {
                    return;
                }
                self.focus_done_index = (self.focus_done_index + 1).min(done_count - 1);
            }
        }
    }

    pub fn focus_select_previous(&mut self) {
        let active_count = get_active_task_count(&self.columns);

        match self.focus_panel {
            FocusPanel::ActiveTasks => {
                self.focus_active_index = self.focus_active_index.saturating_sub(1);
            }
            FocusPanel::DoneTasks => {
                let is_at_first_done_task = self.focus_done_index == 0;
                if is_at_first_done_task && active_count > 0 {
                    self.focus_panel = FocusPanel::ActiveTasks;
                    self.focus_active_index = active_count - 1;
                } else {
                    self.focus_done_index = self.focus_done_index.saturating_sub(1);
                }
            }
        }
    }

    pub fn focus_switch_to_done_panel(&mut self) {
        let done_count = get_done_task_count(&self.columns);
        if done_count == 0 {
            return;
        }
        self.focus_panel = FocusPanel::DoneTasks;
        if self.focus_done_index >= done_count {
            self.focus_done_index = done_count.saturating_sub(1);
        }
    }

    pub fn focus_switch_to_active_panel(&mut self) {
        let active_count = get_active_task_count(&self.columns);
        if active_count == 0 {
            return;
        }
        self.focus_panel = FocusPanel::ActiveTasks;
        if self.focus_active_index >= active_count {
            self.focus_active_index = active_count.saturating_sub(1);
        }
    }

    pub fn enter_terminal_mode(&mut self) {
        self.mode = TasksMode::TerminalFocused;
    }

    pub fn exit_terminal_mode(&mut self) {
        self.mode = TasksMode::Normal;
    }

    pub fn clamp_focus_selection(&mut self) {
        let active_count = get_active_task_count(&self.columns);
        let done_count = get_done_task_count(&self.columns);

        if active_count == 0 {
            self.focus_active_index = 0;
        } else if self.focus_active_index >= active_count {
            self.focus_active_index = active_count - 1;
        }

        if done_count == 0 {
            self.focus_done_index = 0;
        } else if self.focus_done_index >= done_count {
            self.focus_done_index = done_count - 1;
        }

        let focused_panel_empty = match self.focus_panel {
            FocusPanel::ActiveTasks => active_count == 0,
            FocusPanel::DoneTasks => done_count == 0,
        };

        if focused_panel_empty {
            if active_count > 0 {
                self.focus_panel = FocusPanel::ActiveTasks;
            } else if done_count > 0 {
                self.focus_panel = FocusPanel::DoneTasks;
            }
        }
    }
}
