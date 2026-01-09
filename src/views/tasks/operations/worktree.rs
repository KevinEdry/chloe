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
        let Some(task) = self
            .columns
            .get_mut(column_index)
            .and_then(|column| column.tasks.get_mut(task_index))
        else {
            return;
        };

        let already_has_worktree = task.worktree_info.is_some();
        if already_has_worktree {
            return;
        }

        let Ok(current_directory) = std::env::current_dir() else {
            return;
        };

        let Ok(repository_root) = crate::views::worktree::find_repository_root(&current_directory)
        else {
            return;
        };

        let Ok(worktree_info) =
            crate::views::worktree::create_worktree(&repository_root, &task.title, &task.id)
        else {
            return;
        };

        task.worktree_info = Some(worktree_info);
    }

    #[must_use]
    pub fn try_merge_worktree(task: &Task) -> Option<crate::views::worktree::MergeResult> {
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

    pub fn try_cleanup_worktree(task: &Task) {
        let Some(worktree_info) = &task.worktree_info else {
            return;
        };

        let was_auto_created = worktree_info.auto_created;
        if !was_auto_created {
            return;
        }

        let Ok(current_directory) = std::env::current_dir() else {
            return;
        };

        let Ok(repository_root) = crate::views::worktree::find_repository_root(&current_directory)
        else {
            return;
        };

        let _ = crate::views::worktree::delete_worktree(&repository_root, worktree_info);
    }
}
