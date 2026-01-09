use super::{InstancePane, InstanceState};
use std::env;
use std::path::PathBuf;

impl InstanceState {
    pub fn create_pane(&mut self, rows: u16, columns: u16) {
        let (actual_rows, actual_columns) = self.calculate_pane_dimensions(rows, columns);
        let working_directory = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let mut pane = InstancePane::new(working_directory.clone(), actual_rows, actual_columns);

        if let Ok(session) =
            super::pty::PtySession::spawn(&working_directory, actual_rows, actual_columns)
        {
            pane.pty_session = Some(session);
        }

        self.panes.push(pane);
        self.selected_pane = self.panes.len() - 1;
    }

    fn calculate_pane_dimensions(&self, default_rows: u16, default_columns: u16) -> (u16, u16) {
        if let Some(area) = self.last_render_area {
            let future_pane_count = self.panes.len() + 1;
            let pane_areas =
                super::layout::calculate_pane_areas(area, self.layout_mode, future_pane_count);

            if let Some(first_pane_area) = pane_areas.first() {
                let inner_area = ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .inner(*first_pane_area);
                return (inner_area.height.max(1), inner_area.width.max(1));
            }
        }

        (default_rows, default_columns)
    }

    pub fn create_pane_for_task(
        &mut self,
        task_title: &str,
        task_description: &str,
        working_directory: Option<PathBuf>,
        rows: u16,
        columns: u16,
    ) -> uuid::Uuid {
        let (actual_rows, actual_columns) = self.calculate_pane_dimensions(rows, columns);
        let working_directory = working_directory
            .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));
        let mut pane = InstancePane::new(working_directory.clone(), actual_rows, actual_columns);

        if let Ok(mut session) =
            super::pty::PtySession::spawn(&working_directory, actual_rows, actual_columns)
        {
            let claude_command = if task_description.is_empty() {
                format!("claude \"{}\"\n", task_title.replace('\"', "\\\""))
            } else {
                format!(
                    "claude \"Work on this task:\n\nTitle: {}\n\nDescription: {}\"\n",
                    task_title.replace('\"', "\\\""),
                    task_description.replace('\"', "\\\"")
                )
            };

            let _ = session.write_input(claude_command.as_bytes());
            let _ = session.write_input(b"clear\n");

            pane.claude_state = super::ClaudeState::Running;
            pane.pty_session = Some(session);
        }

        let pane_id = pane.id;
        self.panes.push(pane);
        self.selected_pane = self.panes.len() - 1;

        pane_id
    }

    pub fn select_pane_by_id(&mut self, instance_id: uuid::Uuid) -> bool {
        for (index, pane) in self.panes.iter().enumerate() {
            if pane.id == instance_id {
                self.selected_pane = index;
                self.mode = super::InstanceMode::Focused;
                return true;
            }
        }
        false
    }

    pub fn close_pane(&mut self) {
        if self.panes.is_empty() {
            return;
        }

        self.panes.remove(self.selected_pane);

        if self.panes.is_empty() {
            self.selected_pane = 0;
        } else if self.selected_pane >= self.panes.len() {
            self.selected_pane = self.panes.len() - 1;
        }
    }

    pub fn close_pane_by_id(&mut self, instance_id: uuid::Uuid) -> bool {
        let index = match self.panes.iter().position(|pane| pane.id == instance_id) {
            Some(i) => i,
            None => return false,
        };

        self.panes.remove(index);

        if self.panes.is_empty() {
            self.selected_pane = 0;
        } else if self.selected_pane >= self.panes.len() {
            self.selected_pane = self.panes.len() - 1;
        } else if self.selected_pane > index {
            self.selected_pane -= 1;
        }

        true
    }

    pub fn next_pane(&mut self) {
        if self.panes.is_empty() {
            return;
        }

        self.selected_pane = (self.selected_pane + 1) % self.panes.len();
    }

    pub fn previous_pane(&mut self) {
        if self.panes.is_empty() {
            return;
        }

        if self.selected_pane == 0 {
            self.selected_pane = self.panes.len() - 1;
        } else {
            self.selected_pane -= 1;
        }
    }

    pub fn poll_pty_output(&mut self) {
        for pane in &mut self.panes {
            if let Some(session) = &mut pane.pty_session {
                let _ = session.read_output();
            }
        }

        for pane in &mut self.panes {
            if pane.pty_session.is_some() {
                Self::capture_output_buffer(pane);
                Self::check_process_exit(pane);
            }
        }
    }

    fn capture_output_buffer(pane: &mut InstancePane) {
        let session = match &pane.pty_session {
            Some(s) => s,
            None => return,
        };

        if let Ok(parser) = session.screen().lock() {
            let screen = parser.screen();
            let mut screen_text = String::new();

            for row in 0..screen.size().0 {
                for col in 0..screen.size().1 {
                    if let Some(cell) = screen.cell(row, col) {
                        screen_text.push_str(&cell.contents());
                    }
                }
                screen_text.push('\n');
            }

            pane.output_buffer = screen_text;
        }
    }

    fn check_process_exit(pane: &mut InstancePane) {
        let process_has_exited = if let Some(session) = &mut pane.pty_session {
            session.check_process_exit()
        } else {
            return;
        };

        if process_has_exited {
            pane.claude_state = super::ClaudeState::Done;
        }
    }

    pub fn send_input_to_instance(&mut self, instance_id: uuid::Uuid, input: &str) -> bool {
        let pane = match self.panes.iter_mut().find(|p| p.id == instance_id) {
            Some(p) => p,
            None => return false,
        };

        if let Some(session) = &mut pane.pty_session {
            let input_with_enter = format!("{}\r", input);
            if session.write_input(input_with_enter.as_bytes()).is_ok() {
                return true;
            }
        }

        false
    }

    pub fn send_raw_input_to_instance(&mut self, instance_id: uuid::Uuid, data: &[u8]) -> bool {
        let pane = match self.panes.iter_mut().find(|p| p.id == instance_id) {
            Some(p) => p,
            None => return false,
        };

        pane.scroll_to_bottom();

        if let Some(session) = &mut pane.pty_session {
            if session.write_input(data).is_ok() {
                return true;
            }
        }

        false
    }
}
