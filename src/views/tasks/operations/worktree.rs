use crate::views::tasks::state::{Task, TasksState};

impl TasksState {
    pub fn try_create_worktree_for_task(&mut self, task_index: usize) {
        self.try_create_worktree_for_task_in_column(self.kanban_selected_column, task_index);
    }

    pub fn try_create_worktree_for_task_in_column(
        &mut self,
        column_index: usize,
        task_index: usize,
    ) {
        let task = match self
            .columns
            .get_mut(column_index)
            .and_then(|column| column.tasks.get_mut(task_index))
        {
            Some(task) => task,
            None => return,
        };

        let already_has_worktree = task.worktree_info.is_some();
        if already_has_worktree {
            return;
        }

        let current_directory = match std::env::current_dir() {
            Ok(directory) => directory,
            Err(_) => return,
        };

        let repository_root = match crate::views::worktree::find_repository_root(&current_directory)
        {
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

    pub fn try_merge_worktree(&self, task: &Task) -> Option<crate::views::worktree::MergeResult> {
        let worktree_info = task.worktree_info.as_ref()?;

        let was_auto_created = worktree_info.auto_created;
        if !was_auto_created {
            return None;
        }

        let current_directory = std::env::current_dir().ok()?;
        let repository_root =
            crate::views::worktree::find_repository_root(&current_directory).ok()?;

        crate::views::worktree::merge_worktree_to_main(&repository_root, worktree_info).ok()
    }

    pub fn try_cleanup_worktree(&self, task: &Task) {
        let worktree_info = match &task.worktree_info {
            Some(info) => info,
            None => return,
        };

        let was_auto_created = worktree_info.auto_created;
        if !was_auto_created {
            return;
        }

        let current_directory = match std::env::current_dir() {
            Ok(directory) => directory,
            Err(_) => return,
        };

        let repository_root = match crate::views::worktree::find_repository_root(&current_directory)
        {
            Ok(root) => root,
            Err(_) => return,
        };

        let _ = crate::views::worktree::delete_worktree(&repository_root, worktree_info);
    }
}
