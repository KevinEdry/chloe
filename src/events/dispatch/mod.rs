pub mod tasks;

use crate::app::{App, Tab};
use crate::events::{
    AppAction, AppEvent, EventHandler, EventResult, PullRequestAction, RoadmapAction,
    SettingsAction, TerminalAction, WorktreeAction,
};
use crate::views;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    if handle_exit_confirmation(app, key) {
        return true;
    }

    if app.showing_exit_confirmation {
        return false;
    }

    let result = dispatch_key_event(app, key);

    if result.is_quit() {
        app.showing_exit_confirmation = true;
    }

    false
}

const fn handle_exit_confirmation(app: &mut App, key: KeyEvent) -> bool {
    if !app.showing_exit_confirmation {
        return false;
    }

    match key.code {
        KeyCode::Char('y' | 'Y') => true,
        KeyCode::Char('n' | 'N') | KeyCode::Esc => {
            app.showing_exit_confirmation = false;
            false
        }
        _ => false,
    }
}

fn is_terminal_focused(app: &App) -> bool {
    let instances_focused = app.active_tab == Tab::Instances
        && matches!(
            app.instances.mode,
            views::instances::InstanceMode::Focused | views::instances::InstanceMode::Scroll
        );
    let tasks_focused = app.active_tab == Tab::Tasks && app.tasks.is_terminal_focused();
    instances_focused || tasks_focused
}

fn is_typing_mode(app: &App) -> bool {
    app.active_tab == Tab::Tasks && app.tasks.is_typing_mode()
}

fn dispatch_key_event(app: &mut App, key: KeyEvent) -> EventResult {
    let terminal_focused = is_terminal_focused(app);
    let typing_mode = is_typing_mode(app);
    let can_handle_global = !terminal_focused && !typing_mode;

    if let Some(result) = handle_global_key(app, key, can_handle_global) {
        return result;
    }

    dispatch_to_active_tab(app, key)
}

fn handle_global_key(app: &mut App, key: KeyEvent, can_handle: bool) -> Option<EventResult> {
    if !can_handle {
        return None;
    }

    match key.code {
        KeyCode::Char('q' | 'Q') => Some(EventResult::Quit),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(EventResult::Quit)
        }
        KeyCode::Tab => {
            app.next_tab();
            Some(EventResult::Consumed)
        }
        KeyCode::BackTab => {
            app.previous_tab();
            Some(EventResult::Consumed)
        }
        KeyCode::Char('1') => {
            app.switch_tab(Tab::Tasks);
            Some(EventResult::Consumed)
        }
        KeyCode::Char('2') => {
            app.switch_tab(Tab::Instances);
            Some(EventResult::Consumed)
        }
        KeyCode::Char('3') => {
            app.switch_tab(Tab::Roadmap);
            Some(EventResult::Consumed)
        }
        KeyCode::Char('4') => {
            app.switch_tab(Tab::Worktree);
            Some(EventResult::Consumed)
        }
        KeyCode::Char('5') => {
            app.switch_tab(Tab::PullRequests);
            Some(EventResult::Consumed)
        }
        KeyCode::Char('6') => {
            app.switch_tab(Tab::Settings);
            Some(EventResult::Consumed)
        }
        _ => None,
    }
}

fn dispatch_to_active_tab(app: &mut App, key: KeyEvent) -> EventResult {
    match app.active_tab {
        Tab::Tasks => dispatch_tasks_event(app, key),
        Tab::Instances => dispatch_instances_event(app, key),
        Tab::Roadmap => dispatch_roadmap_event(app, key),
        Tab::Worktree => dispatch_worktree_event(app, key),
        Tab::PullRequests => dispatch_pull_requests_event(app, key),
        Tab::Settings => dispatch_settings_event(app, key),
    }
}

fn dispatch_tasks_event(app: &mut App, key: KeyEvent) -> EventResult {
    let is_jump_to_instance =
        app.tasks.is_normal_mode() && matches!(key.code, KeyCode::Char('t' | 'T'));

    if is_jump_to_instance {
        app.jump_to_task_instance();
        return EventResult::Consumed;
    }

    tasks::process_event(app, key);
    EventResult::Consumed
}

fn dispatch_instances_event(app: &mut App, key: KeyEvent) -> EventResult {
    let result = app.instances.handle_key(key);

    if let EventResult::Action(action) = &result {
        process_instances_action(app, action);
        return EventResult::Consumed;
    }

    result
}

fn process_instances_action(app: &mut App, action: &AppAction) {
    if let AppAction::Terminal(TerminalAction::SendInput { instance_id, data }) = action
        && let Some(pane) = app.instances.find_pane_mut(*instance_id)
        && let Some(session) = &mut pane.pty_session
    {
        let _ = session.write_input(data);
    }
}

fn dispatch_roadmap_event(app: &mut App, key: KeyEvent) -> EventResult {
    let result = app.roadmap.handle_key(key);

    if let EventResult::Action(action) = &result {
        process_roadmap_action(app, action);
        return EventResult::Consumed;
    }

    result
}

fn process_roadmap_action(app: &mut App, action: &AppAction) {
    match action {
        AppAction::Roadmap(RoadmapAction::ConvertToTask(index)) => {
            app.convert_roadmap_item_to_task(*index);
            app.active_tab = Tab::Tasks;
        }
        AppAction::Roadmap(RoadmapAction::Generate) => {
            if let Ok(current_directory) = std::env::current_dir()
                && let Some(event_sender) = app.event_sender()
            {
                app.roadmap.start_generation(
                    current_directory.to_string_lossy().to_string(),
                    event_sender,
                );
            }
        }
        AppAction::Settings(SettingsAction::SaveState) => {
            let _ = app.save();
        }
        _ => {}
    }
}

fn dispatch_worktree_event(app: &mut App, key: KeyEvent) -> EventResult {
    let vcs_command = &app.settings.settings.vcs_command.clone();
    app.worktree.handle_key_event(key, vcs_command);

    let result = app.worktree.handle_key(key);

    if let EventResult::Action(action) = &result {
        process_worktree_action(app, action);
    }

    if let Some(worktree_index) = app.worktree.pending_ide_open.take() {
        app.open_worktree_in_ide(worktree_index);
    }

    if let Some(worktree_index) = app.worktree.pending_terminal_open.take() {
        app.open_worktree_in_terminal(worktree_index);
    }

    EventResult::Consumed
}

fn process_worktree_action(app: &App, action: &AppAction) {
    match action {
        AppAction::Worktree(WorktreeAction::OpenInIde(index)) => {
            app.open_worktree_in_ide(*index);
        }
        AppAction::Worktree(WorktreeAction::OpenInTerminal(index)) => {
            app.open_worktree_in_terminal(*index);
        }
        _ => {}
    }
}

fn dispatch_pull_requests_event(app: &mut App, key: KeyEvent) -> EventResult {
    let result = app.pull_requests.handle_key(key);

    if let EventResult::Action(action) = &result {
        process_pull_requests_action(app, action);
        return EventResult::Consumed;
    }

    result
}

fn process_pull_requests_action(app: &mut App, action: &AppAction) {
    match action {
        AppAction::PullRequest(PullRequestAction::Refresh) => {
            views::pull_requests::refresh(&mut app.pull_requests);
        }
        AppAction::PullRequest(PullRequestAction::OpenInBrowser) => {
            if let Some(pull_request) = app.pull_requests.get_selected_pull_request() {
                let url = pull_request.url.clone();
                let _ = views::pull_requests::open_url_in_browser(&url);
            }
        }
        _ => {}
    }
}

fn dispatch_settings_event(app: &mut App, key: KeyEvent) -> EventResult {
    let result = app.settings.handle_key(key);

    if matches!(
        &result,
        EventResult::Action(AppAction::Settings(SettingsAction::Save))
    ) {
        let _ = app.save_settings();
        return EventResult::Consumed;
    }

    result
}

pub fn handle_app_event(app: &mut App, event: AppEvent) {
    match event {
        AppEvent::PtyOutput { pane_id, data } => {
            app.instances.process_pty_output(pane_id, &data);
        }
        AppEvent::PtyExit { pane_id } => {
            app.instances.handle_pty_exit(pane_id);
        }
        AppEvent::ClassificationCompleted { task_id, result } => {
            app.tasks.handle_classification_completed(task_id, result);
        }
        AppEvent::RoadmapGenerationCompleted { result } => {
            app.roadmap.handle_generation_completed(result);
        }
        AppEvent::HookReceived(hook_event) => {
            app.process_hook_event(&hook_event);
        }
    }
}

pub fn handle_tick(app: &mut App) {
    if app.active_tab == Tab::Tasks && app.tasks.has_pending_classifications() {
        app.tasks.advance_spinner();
    }

    if app.active_tab == Tab::Roadmap && app.roadmap.mode == views::roadmap::RoadmapMode::Generating
    {
        app.roadmap.advance_spinner();
    }

    if app.active_tab == Tab::Worktree {
        let vcs_command = &app.settings.settings.vcs_command;
        app.worktree.poll_worktrees(vcs_command);
    }

    if app.active_tab == Tab::PullRequests && app.pull_requests.should_refresh() {
        views::pull_requests::refresh(&mut app.pull_requests);
    }

    app.auto_transition_completed_tasks();
}
