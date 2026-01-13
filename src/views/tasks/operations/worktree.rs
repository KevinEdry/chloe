use crate::views::settings::VcsCommand;
use crate::views::tasks::state::{Task, TasksMode, TasksState, WorktreeSelectionOption};
use crate::views::worktree::WorktreeInfo;
use crate::views::worktree::operations::list_worktrees;
use uuid::Uuid;

impl TasksState {
    pub fn begin_add_task(&mut self) {
        let prompt = Self::select_prompt();
        self.mode = TasksMode::AddingTask {
            input: String::new(),
            prompt,
        };
    }

    fn load_worktree_selection_options(vcs_command: &VcsCommand) -> Vec<WorktreeSelectionOption> {
        let mut options = vec![WorktreeSelectionOption::AutoCreate];

        let Ok(current_directory) = std::env::current_dir() else {
            return options;
        };

        let Ok(repository_root) = crate::views::worktree::find_repository_root(&current_directory)
        else {
            return options;
        };

        let Ok(worktrees) = list_worktrees(&repository_root, vcs_command) else {
            return options;
        };

        for worktree in worktrees {
            options.push(WorktreeSelectionOption::Existing {
                branch_name: worktree.branch_name,
                worktree_path: worktree.path,
            });
        }

        options
    }

    pub fn begin_worktree_selection_for_task(&mut self, task_id: Uuid, vcs_command: &VcsCommand) {
        let Some(task) = self.find_task_by_id(task_id) else {
            return;
        };

        let options = Self::load_worktree_selection_options(vcs_command);
        self.mode = TasksMode::SelectWorktree {
            task_id,
            task_title: task.title.clone(),
            selected_index: 0,
            options,
        };
    }

    fn select_prompt() -> String {
        const PROMPTS: [&str; 20] = [
            "What should we build today?",
            "Lets build something awesome, what are you thinking about?",
            "Whats on your mind to create right now?",
            "What should we tackle together today?",
            "Got an idea you want to bring to life?",
            "Whats the next cool thing we should build?",
            "What problem do you want to solve today?",
            "What should we prototype right now?",
            "Whats the next feature youre excited about?",
            "What do you want to ship next?",
            "What should we experiment with today?",
            "Whats the next big idea to explore?",
            "What should we design and build together?",
            "Whats a task you want to knock out?",
            "What are we crafting today?",
            "Whats your next project idea?",
            "What should we build to make things better?",
            "What are you thinking about building next?",
            "What challenge do you want to solve today?",
            "What should we create right now?",
        ];
        const PROMPT_COUNT: u128 = PROMPTS.len() as u128;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();

        let prompt_index = (timestamp % PROMPT_COUNT) as usize;
        PROMPTS[prompt_index].to_string()
    }

    #[must_use]
    pub fn create_worktree_for_new_task(
        task_title: &str,
        task_id: &Uuid,
        vcs_command: &VcsCommand,
    ) -> Option<WorktreeInfo> {
        let Ok(current_directory) = std::env::current_dir() else {
            return None;
        };

        let Ok(repository_root) = crate::views::worktree::find_repository_root(&current_directory)
        else {
            return None;
        };

        crate::views::worktree::create_worktree(&repository_root, task_title, task_id, vcs_command)
            .ok()
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

    pub fn try_cleanup_worktree(task: &Task, vcs_command: &VcsCommand) {
        let Some(worktree_info) = &task.worktree_info else {
            return;
        };

        let was_auto_created = worktree_info.auto_created;
        if !was_auto_created {
            return;
        };

        let Ok(current_directory) = std::env::current_dir() else {
            return;
        };

        let Ok(repository_root) = crate::views::worktree::find_repository_root(&current_directory)
        else {
            return;
        };

        let _ =
            crate::views::worktree::delete_worktree(&repository_root, worktree_info, vcs_command);
    }
}
