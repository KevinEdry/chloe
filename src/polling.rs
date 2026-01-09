use crate::app::{App, Tab};
use crate::events::EventListener;
use crate::views;
use crate::views::tasks::{FocusPanel, TasksAction, TasksMode, get_active_tasks, get_done_tasks};
use crossterm::event::KeyEvent;

pub fn poll_background_tasks(app: &mut App, event_listener: &EventListener) {
    if app.active_tab == Tab::Tasks {
        let was_classifying = matches!(app.tasks.mode, TasksMode::ClassifyingTask { .. });
        app.tasks.poll_classification();
        let is_still_classifying = matches!(app.tasks.mode, TasksMode::ClassifyingTask { .. });

        if was_classifying && !is_still_classifying {
            app.tasks.mode = TasksMode::Normal;
        }
    }

    if app.active_tab == Tab::Roadmap {
        app.roadmap.poll_generation();
        if app.roadmap.mode == views::roadmap::RoadmapMode::Generating {
            app.roadmap.advance_spinner();
        }
    }

    app.instances.poll_pty_output();

    if app.active_tab == Tab::Worktree {
        app.worktree.poll_worktrees();
    }

    for event in event_listener.poll_events() {
        app.process_hook_event(&event);
    }

    app.auto_transition_completed_tasks();
}

pub fn process_tasks_pending_actions(app: &mut App) {
    if let Some(instance_id) = app.tasks.pending_instance_termination.take() {
        app.instances.close_pane_by_id(instance_id);
    }

    if let Some(worktree_info) = app.tasks.pending_worktree_deletion.take() {
        if let Ok(repo_root) = views::worktree::find_repository_root(
            std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .as_path(),
        ) {
            let _ = views::worktree::delete_worktree(&repo_root, &worktree_info);
        }
    }

    if let Some(task_id) = app.tasks.pending_ide_open.take() {
        app.open_task_in_ide(task_id);
    }

    if let Some(task_id) = app.tasks.pending_terminal_switch.take() {
        app.open_task_in_terminal(task_id);
    }

    if let Some((task_id, change_request)) = app.tasks.pending_change_request.take() {
        if let Some(instance_id) = app.tasks.move_task_to_in_progress_by_id(task_id) {
            app.instances
                .send_input_to_instance(instance_id, &change_request);
        }
    }

    app.sync_task_instances();
}

pub fn process_roadmap_action(app: &mut App, action: views::roadmap::events::RoadmapAction) {
    match action {
        views::roadmap::events::RoadmapAction::ConvertToTask(item_index) => {
            app.convert_roadmap_item_to_task(item_index);
            app.active_tab = Tab::Tasks;
        }
        views::roadmap::events::RoadmapAction::SaveState => {
            let _ = app.save();
        }
        views::roadmap::events::RoadmapAction::GenerateRoadmap => {
            if let Ok(current_dir) = std::env::current_dir() {
                app.roadmap
                    .start_generation(current_dir.to_string_lossy().to_string());
            }
        }
        views::roadmap::events::RoadmapAction::None => {}
    }
}

pub fn process_worktree_pending_actions(app: &mut App) {
    if let Some(worktree_index) = app.worktree.pending_ide_open.take() {
        app.open_worktree_in_ide(worktree_index);
    }

    if let Some(worktree_index) = app.worktree.pending_terminal_open.take() {
        app.open_worktree_in_terminal(worktree_index);
    }
}

pub fn process_tasks_event(app: &mut App, key: KeyEvent) {
    let selected_instance_id = match app.tasks.focus_panel {
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
    };

    let action = views::tasks::handle_key_event(&mut app.tasks, key, selected_instance_id);

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
        TasksAction::CreateTask(title) => {
            app.tasks.mode = TasksMode::ClassifyingTask {
                raw_input: title.clone(),
                edit_task_id: None,
            };
            app.tasks.start_classification(title);
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
        TasksAction::StartTask(task_id) => {
            app.tasks.move_task_to_in_progress_by_id(task_id);
            app.sync_task_instances();
            let _ = app.save();
        }
        TasksAction::CancelClassification => {
            app.tasks.cancel_classification();
        }
        TasksAction::OpenInIDE(task_id) => {
            app.open_task_in_ide(task_id);
        }
        TasksAction::SwitchToTerminal(task_id) => {
            app.open_task_in_terminal(task_id);
        }
        TasksAction::RequestChanges { task_id, message } => {
            if let Some(instance_id) = app.tasks.move_task_to_in_progress_by_id(task_id) {
                app.instances.send_input_to_instance(instance_id, &message);
            }
        }
        TasksAction::MergeBranch(task_id) => {
            app.merge_task_branch(task_id);
        }
    }

    app.tasks.clamp_focus_selection();

    process_tasks_pending_actions(app);
}
