pub mod widgets;

use crate::app::{App, Tab};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Tabs as RatTabs},
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Min(0),    // Content
        ])
        .split(f.area());

    // Render tab bar
    let tab_titles = vec!["Kanban", "Instances", "Roadmap", "Worktree"];
    let tabs = RatTabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Chloe"))
        .select(match app.active_tab {
            Tab::Kanban => 0,
            Tab::Instances => 1,
            Tab::Roadmap => 2,
            Tab::Worktree => 3,
        })
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, chunks[0]);

    // Render active tab content
    match app.active_tab {
        Tab::Kanban => crate::kanban::ui::render(f, app, chunks[1]),
        Tab::Instances => crate::instance::ui::render(f, &mut app.instances, chunks[1]),
        Tab::Roadmap => crate::roadmap::ui::render(f, app, chunks[1]),
        Tab::Worktree => crate::worktree::tab_ui::render(f, chunks[1], &app.worktree),
    }

    // Render exit confirmation dialog on top if showing
    if app.showing_exit_confirmation {
        crate::kanban::ui::dialogs::render_exit_confirmation_dialog(f, f.area());
    }
}
