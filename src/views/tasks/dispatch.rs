use crate::app::{App, Tab};
use crate::views;
use crate::views::tasks::state::WorktreeSelectionOption;
use crate::views::tasks::{FocusPanel, TasksAction, get_active_tasks, get_done_tasks};
use crossterm::event::KeyEvent;
use uuid::Uuid;

pub fn process_event(app: &mut App, key: KeyEvent) {
    let selected_instance_id = get_selected_instance_id(app);
    let default_provider = app.settings.settings.default_provider;
    let vcs_command = &app.settings.settings.vcs_command;
    let action = views::tasks::handle_key_event(
        &mut app.tasks,
        key,
        selected_instance_id,
        default_provider,
        vcs_command,
    );

    handle_action(app, action);
    app.tasks.clamp_focus_selection();
    process_pending_actions(app);
}

fn get_selected_instance_id(app: &App) -> Option<Uuid> {
    match app.tasks.focus_panel {
        FocusPanel::ActiveTasks => {
            let tasks = get_active_tasks(&app.tasks.columns);
            tasks
                .into_iter()
                .nth(app.tasks.focus_active_index)
                .and_then(|task_ref| task_ref.task.instance_id)
        }
        FocusPanel::DoneTasks => {
            let tasks = get_done_tasks(&app.tasks.columns);
            tasks
                .into_iter()
                .nth(app.tasks.focus_done_index)
                .and_then(|task_ref| task_ref.task.instance_id)
        }
    }
}

fn handle_action(app: &mut App, action: TasksAction) {
    match action {
        TasksAction::None => {}
        TasksAction::JumpToInstance(instance_id) => {
            app.active_tab = Tab::Instances;
            app.instances.select_pane_by_id(instance_id);
            app.instances.mode = views::instances::InstanceMode::Focused;
        }
        TasksAction::SendToTerminal(instance_id, data) => {
            if !data.is_empty() {
                app.instances.send_raw_input_to_instance(instance_id, &data);
            }
        }
        TasksAction::ScrollTerminal { instance_id, delta } => {
            handle_scroll_terminal(app, instance_id, delta);
        }
        TasksAction::ScrollTerminalToTop(instance_id) => {
            if let Some(pane) = app.instances.find_pane_mut(instance_id) {
                let max_scrollback = pane.scrollback_len();
                pane.scroll_up(max_scrollback, max_scrollback);
            }
        }
        TasksAction::ScrollTerminalToBottom(instance_id) => {
            if let Some(pane) = app.instances.find_pane_mut(instance_id) {
                pane.scroll_to_bottom();
            }
        }
        TasksAction::CreateTask { title } => {
            if let Some(event_sender) = app.event_sender() {
                let provider = app.settings.settings.default_provider;
                app.tasks
                    .start_classification(title, provider, event_sender);
                let _ = app.save();
            }
        }
        TasksAction::UpdateTask { task_id, new_title } => {
            app.tasks.update_task_title_by_id(task_id, new_title);
            let _ = app.save();
        }
        TasksAction::DeleteTask(task_id) => {
            if let Some(instance_id) = app.tasks.delete_task_by_id(task_id) {
                app.instances.close_pane_by_id(instance_id);
            }
            let _ = app.save();
        }
        TasksAction::OpenInIDE(task_id) => app.open_task_in_ide(task_id),
        TasksAction::SwitchToTerminal(task_id) => app.open_task_in_terminal(task_id),
        TasksAction::RequestChanges { task_id, message } => {
            let vcs_command = &app.settings.settings.vcs_command;
            if let Some(instance_id) = app
                .tasks
                .move_task_to_in_progress_by_id(task_id, vcs_command)
            {
                app.instances.send_input_to_instance(instance_id, &message);
            }
        }
        TasksAction::CommitChanges(task_id) => app.commit_task_changes(task_id),
        TasksAction::MergeBranch { task_id, target } => app.merge_task_branch(task_id, &target),
        TasksAction::WorktreeSelected {
            task_id,
            worktree_option,
            ..
        } => handle_worktree_selected(app, task_id, worktree_option),
        TasksAction::ProviderSelected {
            task_id,
            provider,
            worktree_option,
            remember,
        } => handle_provider_selected(app, task_id, provider, worktree_option, remember),
    }
}

fn handle_scroll_terminal(app: &mut App, instance_id: Uuid, delta: isize) {
    if let Some(pane) = app.instances.find_pane_mut(instance_id) {
        let max_scrollback = pane.scrollback_len();
        if delta > 0 {
            pane.scroll_up(delta.unsigned_abs(), max_scrollback);
        } else {
            pane.scroll_down(delta.unsigned_abs());
        }
    }
}

fn handle_worktree_selected(
    app: &mut App,
    task_id: Uuid,
    worktree_option: WorktreeSelectionOption,
) {
    let detected_providers = &app.settings.detected_providers;
    let should_skip =
        app.settings.settings.skip_provider_selection || detected_providers.len() <= 1;

    if should_skip {
        let provider = if detected_providers.len() == 1 {
            detected_providers[0].provider
        } else {
            app.settings.settings.default_provider
        };
        app.tasks.set_task_provider(task_id, provider);
        let vcs_command = &app.settings.settings.vcs_command;
        app.tasks
            .move_task_to_in_progress_with_worktree(task_id, worktree_option, vcs_command);
        let _ = app.save();
    } else {
        app.tasks.mode = views::tasks::TasksMode::SelectProvider {
            task_id,
            selected_index: 0,
            worktree_option,
            detected_providers: detected_providers.clone(),
        };
    }
}

fn handle_provider_selected(
    app: &mut App,
    task_id: Uuid,
    provider: crate::types::AgentProvider,
    worktree_option: WorktreeSelectionOption,
    remember: bool,
) {
    app.tasks.set_task_provider(task_id, provider);
    if remember {
        app.settings.settings.default_provider = provider;
        app.settings.settings.skip_provider_selection = true;
        let _ = app.save_settings();
    }
    let vcs_command = &app.settings.settings.vcs_command;
    app.tasks
        .move_task_to_in_progress_with_worktree(task_id, worktree_option, vcs_command);
    let _ = app.save();
}

fn process_pending_actions(app: &mut App) {
    if let Some(instance_id) = app.tasks.pending_instance_termination.take() {
        app.instances.close_pane_by_id(instance_id);
    }

    if let Some(worktree_info) = app.tasks.pending_worktree_deletion.take()
        && let Ok(repo_root) = views::worktree::find_repository_root(
            std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .as_path(),
        )
    {
        let vcs_command = &app.settings.settings.vcs_command;
        let _ = views::worktree::delete_worktree(&repo_root, &worktree_info, vcs_command);
    }

    if let Some(task_id) = app.tasks.pending_ide_open.take() {
        app.open_task_in_ide(task_id);
    }

    if let Some(task_id) = app.tasks.pending_terminal_switch.take() {
        app.open_task_in_terminal(task_id);
    }

    if let Some((task_id, change_request)) = app.tasks.pending_change_request.take() {
        let vcs_command = &app.settings.settings.vcs_command;
        if let Some(instance_id) = app
            .tasks
            .move_task_to_in_progress_by_id(task_id, vcs_command)
        {
            app.instances
                .send_input_to_instance(instance_id, &change_request);
        }
    }

    app.sync_task_instances();
}
