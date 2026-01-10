use super::state::{InstancePane, InstanceState, PaneNode, SplitDirection};
use super::{layout, pty};
use ratatui::layout::Rect;
use std::env;
use std::path::PathBuf;
use uuid::Uuid;

impl InstanceState {
    pub fn create_pane(&mut self, rows: u16, columns: u16) {
        let working_directory = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let (actual_rows, actual_columns) = self.calculate_pane_dimensions(rows, columns);

        let mut pane = InstancePane::new(working_directory.clone(), actual_rows, actual_columns);

        if let Ok(session) = pty::PtySession::spawn(&working_directory, actual_rows, actual_columns)
        {
            pane.pty_session = Some(session);
        }

        let pane_id = pane.id;

        if self.root.is_none() {
            self.root = Some(PaneNode::Leaf(pane));
            self.selected_pane_id = Some(pane_id);
            return;
        }

        self.split_biggest_pane_with_new_pane(pane);
    }

    fn split_biggest_pane_with_new_pane(&mut self, new_pane: InstancePane) {
        let Some(target_id) = layout::find_biggest_pane_id(&self.pane_areas) else {
            return;
        };

        let Some(target_area) = self.get_pane_area(target_id) else {
            return;
        };

        let Some(direction) = layout::choose_split_direction(target_area) else {
            return;
        };

        let new_pane_id = new_pane.id;

        if let Some(root) = self.root.take() {
            self.root = Some(split_node_at_pane(
                root,
                target_id,
                new_pane,
                direction,
                layout::default_split_ratio(),
            ));
            self.selected_pane_id = Some(new_pane_id);
        }
    }

    fn calculate_pane_dimensions(&self, default_rows: u16, default_columns: u16) -> (u16, u16) {
        let Some(area) = self.last_render_area else {
            return (default_rows, default_columns);
        };

        let inner_area = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .inner(area);

        let rows = inner_area.height.max(1);
        let columns = inner_area.width.max(1);

        (rows, columns)
    }

    pub fn create_pane_for_task(
        &mut self,
        task_title: &str,
        task_description: &str,
        working_directory: Option<PathBuf>,
        rows: u16,
        columns: u16,
    ) -> Uuid {
        let working_directory = working_directory
            .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));
        let (actual_rows, actual_columns) = self.calculate_pane_dimensions(rows, columns);

        let mut pane = InstancePane::new(working_directory.clone(), actual_rows, actual_columns);

        if let Ok(session) =
            pty::PtySession::spawn(&working_directory, actual_rows, actual_columns)
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

        if self.root.is_none() {
            self.root = Some(PaneNode::Leaf(pane));
            self.selected_pane_id = Some(pane_id);
        } else {
            self.split_biggest_pane_with_new_pane(pane);
        }

        pane_id
    }

    pub fn select_pane_by_id(&mut self, instance_id: Uuid) -> bool {
        if self.find_pane(instance_id).is_some() {
            self.selected_pane_id = Some(instance_id);
            self.mode = super::InstanceMode::Focused;
            return true;
        }
        false
    }

    pub fn close_pane(&mut self) {
        let Some(selected_id) = self.selected_pane_id else {
            return;
        };

        self.close_pane_by_id(selected_id);
    }

    pub fn close_pane_by_id(&mut self, instance_id: Uuid) -> bool {
        let Some(root) = self.root.take() else {
            return false;
        };

        let pane_ids: Vec<Uuid> = Self::collect_pane_ids_from_node(&root);

        if !pane_ids.contains(&instance_id) {
            self.root = Some(root);
            return false;
        }

        let new_root = remove_pane_from_tree(root, instance_id);
        self.root = new_root;

        if self.selected_pane_id == Some(instance_id) {
            self.selected_pane_id = self.root.as_ref().map(PaneNode::first_pane_id);
        }

        true
    }

    fn collect_pane_ids_from_node(node: &PaneNode) -> Vec<Uuid> {
        node.collect_panes().iter().map(|p| p.id).collect()
    }

    pub fn next_pane(&mut self) {
        let pane_ids = self.collect_all_pane_ids();
        if pane_ids.is_empty() {
            return;
        }

        let current_index = self
            .selected_pane_id
            .and_then(|id| pane_ids.iter().position(|&pid| pid == id))
            .unwrap_or(0);

        let next_index = (current_index + 1) % pane_ids.len();
        self.selected_pane_id = Some(pane_ids[next_index]);
    }

    pub fn previous_pane(&mut self) {
        let pane_ids = self.collect_all_pane_ids();
        if pane_ids.is_empty() {
            return;
        }

        let current_index = self
            .selected_pane_id
            .and_then(|id| pane_ids.iter().position(|&pid| pid == id))
            .unwrap_or(0);

        let prev_index = if current_index == 0 {
            pane_ids.len() - 1
        } else {
            current_index - 1
        };

        self.selected_pane_id = Some(pane_ids[prev_index]);
    }

    fn collect_all_pane_ids(&self) -> Vec<Uuid> {
        self.collect_panes().iter().map(|p| p.id).collect()
    }

    pub fn poll_pty_output(&mut self) {
        let Some(root) = &self.root else {
            return;
        };

        let pane_ids: Vec<Uuid> = root.collect_panes().iter().map(|p| p.id).collect();

        for pane_id in &pane_ids {
            if let Some(pane) = self.find_pane_mut(*pane_id)
                && let Some(session) = &pane.pty_session
            {
                session.read_output();
            }
        }

        for pane_id in pane_ids {
            if let Some(pane) = self.find_pane_mut(pane_id) {
                Self::check_process_exit(pane);
            }
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

    pub fn send_input_to_instance(&mut self, instance_id: Uuid, input: &str) -> bool {
        let Some(pane) = self.find_pane_mut(instance_id) else {
            return false;
        };

        if let Some(session) = &mut pane.pty_session {
            let input_with_newline = format!("{input}\n");
            if session.write_input(input_with_newline.as_bytes()).is_ok() {
                return true;
            }
        }

        false
    }

    pub fn send_raw_input_to_instance(&mut self, instance_id: Uuid, data: &[u8]) -> bool {
        let Some(pane) = self.find_pane_mut(instance_id) else {
            return false;
        };

        pane.scroll_to_bottom();

        if let Some(session) = &mut pane.pty_session
            && session.write_input(data).is_ok()
        {
            return true;
        }

        false
    }

    pub fn navigate_to_pane_in_direction(&mut self, direction: NavigationDirection) {
        let Some(selected_id) = self.selected_pane_id else {
            return;
        };

        let Some(current_area) = self.get_pane_area(selected_id) else {
            return;
        };

        let target_pane_id =
            find_nearest_pane_in_direction(&self.pane_areas, selected_id, current_area, direction);

        if let Some(pane_id) = target_pane_id {
            self.selected_pane_id = Some(pane_id);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NavigationDirection {
    Left,
    Right,
    Up,
    Down,
}

fn split_node_at_pane(
    node: PaneNode,
    target_id: Uuid,
    new_pane: InstancePane,
    direction: SplitDirection,
    ratio: f32,
) -> PaneNode {
    match node {
        PaneNode::Leaf(pane) => {
            if pane.id == target_id {
                PaneNode::Split {
                    direction,
                    ratio,
                    first: Box::new(PaneNode::Leaf(pane)),
                    second: Box::new(PaneNode::Leaf(new_pane)),
                }
            } else {
                PaneNode::Leaf(pane)
            }
        }
        PaneNode::Split {
            direction: split_dir,
            ratio: split_ratio,
            first,
            second,
        } => {
            let first_contains = contains_pane(&first, target_id);

            if first_contains {
                PaneNode::Split {
                    direction: split_dir,
                    ratio: split_ratio,
                    first: Box::new(split_node_at_pane(
                        *first, target_id, new_pane, direction, ratio,
                    )),
                    second,
                }
            } else {
                PaneNode::Split {
                    direction: split_dir,
                    ratio: split_ratio,
                    first,
                    second: Box::new(split_node_at_pane(
                        *second, target_id, new_pane, direction, ratio,
                    )),
                }
            }
        }
    }
}

fn contains_pane(node: &PaneNode, target_id: Uuid) -> bool {
    match node {
        PaneNode::Leaf(pane) => pane.id == target_id,
        PaneNode::Split { first, second, .. } => {
            contains_pane(first, target_id) || contains_pane(second, target_id)
        }
    }
}

fn remove_pane_from_tree(node: PaneNode, target_id: Uuid) -> Option<PaneNode> {
    match node {
        PaneNode::Leaf(pane) => {
            if pane.id == target_id {
                None
            } else {
                Some(PaneNode::Leaf(pane))
            }
        }
        PaneNode::Split {
            direction,
            ratio,
            first,
            second,
        } => {
            let first_contains = contains_pane(&first, target_id);
            let second_contains = contains_pane(&second, target_id);

            if !first_contains && !second_contains {
                return Some(PaneNode::Split {
                    direction,
                    ratio,
                    first,
                    second,
                });
            }

            if first_contains {
                let new_first = remove_pane_from_tree(*first, target_id);
                match new_first {
                    None => Some(*second),
                    Some(f) => Some(PaneNode::Split {
                        direction,
                        ratio,
                        first: Box::new(f),
                        second,
                    }),
                }
            } else {
                let new_second = remove_pane_from_tree(*second, target_id);
                match new_second {
                    None => Some(*first),
                    Some(s) => Some(PaneNode::Split {
                        direction,
                        ratio,
                        first,
                        second: Box::new(s),
                    }),
                }
            }
        }
    }
}

fn find_nearest_pane_in_direction(
    pane_areas: &[(Uuid, Rect)],
    current_id: Uuid,
    current_area: Rect,
    direction: NavigationDirection,
) -> Option<Uuid> {
    let current_center_x = current_area.x + current_area.width / 2;
    let current_center_y = current_area.y + current_area.height / 2;

    let mut best_candidate: Option<(Uuid, i32)> = None;

    for (pane_id, area) in pane_areas {
        if *pane_id == current_id {
            continue;
        }

        let pane_center_x = area.x + area.width / 2;
        let pane_center_y = area.y + area.height / 2;

        let is_valid_direction = match direction {
            NavigationDirection::Left => pane_center_x < current_center_x,
            NavigationDirection::Right => pane_center_x > current_center_x,
            NavigationDirection::Up => pane_center_y < current_center_y,
            NavigationDirection::Down => pane_center_y > current_center_y,
        };

        if !is_valid_direction {
            continue;
        }

        let dx = (pane_center_x as i32) - (current_center_x as i32);
        let dy = (pane_center_y as i32) - (current_center_y as i32);
        let distance = dx.abs() + dy.abs();

        let is_better = match best_candidate {
            None => true,
            Some((_, best_distance)) => distance < best_distance,
        };

        if is_better {
            best_candidate = Some((*pane_id, distance));
        }
    }

    best_candidate.map(|(id, _)| id)
}
