use crate::app::{App, Tab};
use crate::events::EventListener;
use crate::views;
use crate::views::focus::FocusPanel;
use crate::views::focus::operations::{
    get_active_task_count, get_active_tasks, get_done_task_count, get_done_tasks,
};
use crossterm::event::KeyEvent;

pub fn poll_background_tasks(app: &mut App, event_listener: &EventListener) {
    if app.active_tab == Tab::Kanban {
        app.kanban.poll_classification();
    }

    if app.active_tab == Tab::Focus {
        let was_classifying =
            matches!(app.focus.mode, views::focus::FocusMode::ClassifyingTask { .. });
        app.kanban.poll_classification();
        let is_still_classifying =
            matches!(app.kanban.mode, views::kanban::KanbanMode::ClassifyingTask { .. });

        if was_classifying && !is_still_classifying {
            app.focus.mode = views::focus::FocusMode::Normal;
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

pub fn process_kanban_pending_actions(app: &mut App) {
    if let Some(instance_id) = app.kanban.pending_instance_termination.take() {
        app.instances.close_pane_by_id(instance_id);
    }

    if let Some(worktree_info) = app.kanban.pending_worktree_deletion.take() {
        if let Ok(repo_root) = views::worktree::find_repository_root(
            std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .as_path(),
        ) {
            let _ = views::worktree::delete_worktree(&repo_root, &worktree_info);
        }
    }

    if let Some(task_index) = app.kanban.pending_ide_open.take() {
        app.open_task_in_ide(task_index);
    }

    if let Some(task_index) = app.kanban.pending_terminal_switch.take() {
        app.open_task_in_terminal(task_index);
    }

    if let Some((task_index, change_request)) = app.kanban.pending_change_request.take() {
        if let Some(instance_id) = app.kanban.move_task_to_in_progress(task_index) {
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
            app.active_tab = Tab::Kanban;
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

pub fn process_focus_event(app: &mut App, key: KeyEvent) {
    let selected_instance_id = match app.focus.focused_panel {
        FocusPanel::ActiveTasks => {
            let tasks = get_active_tasks(&app.kanban.columns);
            tasks
                .into_iter()
                .nth(app.focus.active_selected_index)
                .and_then(|task_ref| task_ref.task.instance_id)
        }
        FocusPanel::DoneTasks => {
            let tasks = get_done_tasks(&app.kanban.columns);
            tasks
                .into_iter()
                .nth(app.focus.done_selected_index)
                .and_then(|task_ref| task_ref.task.instance_id)
        }
    };

    let action = views::focus::handle_key_event(
        &mut app.focus,
        key,
        &app.kanban.columns,
        selected_instance_id,
    );

    match action {
        views::focus::FocusAction::None => {}
        views::focus::FocusAction::JumpToInstance(instance_id) => {
            app.active_tab = Tab::Instances;
            app.instances.select_pane_by_id(instance_id);
            app.instances.mode = views::instances::InstanceMode::Focused;
        }
        views::focus::FocusAction::SendToTerminal(instance_id, data) => {
            if !data.is_empty() {
                app.instances.send_raw_input_to_instance(instance_id, &data);
            }
        }
        views::focus::FocusAction::CreateTask(title) => {
            app.focus.mode = views::focus::FocusMode::ClassifyingTask {
                raw_input: title.clone(),
            };
            app.kanban.start_classification(title);
        }
        views::focus::FocusAction::UpdateTask { task_id, new_title } => {
            app.kanban.update_task_title_by_id(task_id, new_title);
            let _ = app.save();
        }
        views::focus::FocusAction::DeleteTask(task_id) => {
            if let Some(instance_id) = app.kanban.delete_task_by_id(task_id) {
                app.instances.close_pane_by_id(instance_id);
            }
            let _ = app.save();
        }
        views::focus::FocusAction::StartTask(task_id) => {
            app.kanban.move_task_to_in_progress_by_id(task_id);
            app.sync_task_instances();
            let _ = app.save();
        }
        views::focus::FocusAction::CancelClassification => {
            app.kanban.cancel_classification();
        }
        views::focus::FocusAction::OpenInIDE(task_id) => {
            if let Some(task_index) = app.kanban.find_task_index_by_id(task_id) {
                app.open_task_in_ide(task_index);
            }
        }
        views::focus::FocusAction::SwitchToTerminal(task_id) => {
            if let Some(task_index) = app.kanban.find_task_index_by_id(task_id) {
                app.open_task_in_terminal(task_index);
            }
        }
        views::focus::FocusAction::RequestChanges { task_id, message } => {
            if let Some(task_index) = app.kanban.find_task_index_by_id(task_id) {
                if let Some(instance_id) = app.kanban.move_task_to_in_progress(task_index) {
                    app.instances
                        .send_input_to_instance(instance_id, &message);
                }
            }
        }
        views::focus::FocusAction::MergeBranch(task_id) => {
            app.merge_task_branch(task_id);
        }
    }

    let active_count = get_active_task_count(&app.kanban.columns);
    let done_count = get_done_task_count(&app.kanban.columns);
    app.focus.clamp_selection(active_count, done_count);
}
