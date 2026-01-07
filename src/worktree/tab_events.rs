use super::tab_state::{WorktreeMode, WorktreeTabState};
use crossterm::event::{KeyCode, KeyEvent};

impl WorktreeTabState {
    pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        match self.mode {
            WorktreeMode::Normal => self.handle_normal_mode(key),
            WorktreeMode::ConfirmDelete { worktree_index } => {
                self.handle_confirm_delete_mode(key, worktree_index)
            }
        }
    }

    fn handle_normal_mode(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next();
                true
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_previous();
                true
            }
            KeyCode::Char('r') => {
                self.refresh_worktrees();
                true
            }
            KeyCode::Char('d') => {
                if self.selected_index.is_some() {
                    self.mode = WorktreeMode::ConfirmDelete {
                        worktree_index: self.selected_index.unwrap(),
                    };
                }
                true
            }
            _ => false,
        }
    }

    fn handle_confirm_delete_mode(&mut self, key: KeyEvent, worktree_index: usize) -> bool {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.delete_worktree_at_index(worktree_index);
                self.mode = WorktreeMode::Normal;
                true
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.mode = WorktreeMode::Normal;
                true
            }
            _ => false,
        }
    }

    fn select_next(&mut self) {
        if self.worktrees.is_empty() {
            self.selected_index = None;
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(idx) if idx < self.worktrees.len() - 1 => idx + 1,
            Some(idx) => idx,
            None => 0,
        });
    }

    fn select_previous(&mut self) {
        if self.worktrees.is_empty() {
            self.selected_index = None;
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(idx) if idx > 0 => idx - 1,
            Some(idx) => idx,
            None => 0,
        });
    }

    fn refresh_worktrees(&mut self) {
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(error) => {
                self.error_message = Some(format!("Failed to get current directory: {error}"));
                return;
            }
        };

        let repository_root = match super::operations::find_repository_root(&current_dir) {
            Ok(root) => root,
            Err(error) => {
                self.error_message = Some(format!("Not a git repository: {error}"));
                self.worktrees.clear();
                self.selected_index = None;
                return;
            }
        };

        match super::operations::list_worktrees(&repository_root) {
            Ok(worktrees) => {
                self.worktrees = worktrees;
                self.error_message = None;

                if self.worktrees.is_empty() {
                    self.selected_index = None;
                } else if self.selected_index.is_none() {
                    self.selected_index = Some(0);
                } else if let Some(idx) = self.selected_index {
                    if idx >= self.worktrees.len() {
                        self.selected_index = Some(self.worktrees.len() - 1);
                    }
                }
            }
            Err(error) => {
                self.error_message = Some(format!("Failed to list worktrees: {error}"));
            }
        }
    }

    fn delete_worktree_at_index(&mut self, index: usize) {
        let worktree = match self.worktrees.get(index) {
            Some(wt) => wt,
            None => return,
        };

        let worktree_info = super::WorktreeInfo {
            branch_name: worktree.branch_name.clone(),
            worktree_path: worktree.path.clone(),
            auto_created: true,
        };

        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(error) => {
                self.error_message = Some(format!("Failed to get current directory: {error}"));
                return;
            }
        };

        let repository_root = match super::operations::find_repository_root(&current_dir) {
            Ok(root) => root,
            Err(error) => {
                self.error_message = Some(format!("Not a git repository: {error}"));
                return;
            }
        };

        match super::operations::delete_worktree(&repository_root, &worktree_info) {
            Ok(()) => {
                self.error_message = None;
                self.refresh_worktrees();
            }
            Err(error) => {
                self.error_message = Some(format!("Failed to delete worktree: {error}"));
            }
        }
    }
}
