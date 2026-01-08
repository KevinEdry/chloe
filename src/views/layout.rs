use super::{focus, footer, instances, kanban, roadmap, tab_bar, worktree};
use crate::app::{App, Tab};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

const TAB_BAR_HEIGHT: u16 = 3;
const FOOTER_HEIGHT: u16 = 3;

struct AppLayout {
    tab_bar: Rect,
    content: Rect,
    footer: Rect,
}

fn calculate_layout(area: Rect) -> AppLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(TAB_BAR_HEIGHT),
            Constraint::Min(0),
            Constraint::Length(FOOTER_HEIGHT),
        ])
        .split(area);

    AppLayout {
        tab_bar: chunks[0],
        content: chunks[1],
        footer: chunks[2],
    }
}

pub fn render(frame: &mut Frame, app: &mut App) {
    let layout = calculate_layout(frame.area());

    tab_bar::render(frame, app, layout.tab_bar);

    match app.active_tab {
        Tab::Kanban => kanban::view::render(frame, app, layout.content),
        Tab::Instances => instances::view::render(frame, &mut app.instances, layout.content),
        Tab::Roadmap => roadmap::view::render(frame, app, layout.content),
        Tab::Worktree => worktree::view::render(frame, layout.content, &app.worktree),
        Tab::Focus => focus::view::render(frame, app, layout.content),
    }

    let status_content = match app.active_tab {
        Tab::Kanban => kanban::view::get_status_bar_content(&app.kanban, layout.footer.width),
        Tab::Instances => {
            instances::view::get_status_bar_content(&app.instances, layout.footer.width)
        }
        Tab::Roadmap => roadmap::view::get_status_bar_content(&app.roadmap, layout.footer.width),
        Tab::Worktree => {
            worktree::view::get_status_bar_content(&app.worktree, layout.footer.width)
        }
        Tab::Focus => focus::view::get_status_bar_content(app, layout.footer.width),
    };

    footer::render_footer(frame, layout.footer, status_content);

    if app.showing_exit_confirmation {
        kanban::dialogs::render_exit_confirmation_dialog(frame, frame.area());
    }
}
