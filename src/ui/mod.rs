pub mod widgets;

use crate::app::{App, Tab};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs as RatTabs},
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Min(0),    // Content
        ])
        .split(f.area());

    let current_directory = std::env::current_dir()
        .ok()
        .and_then(|path| path.to_str().map(String::from))
        .unwrap_or_else(|| String::from("?"));

    let tab_bar_block = Block::default().borders(Borders::ALL);
    let tab_bar_inner = tab_bar_block.inner(chunks[0]);
    f.render_widget(tab_bar_block, chunks[0]);

    let tab_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(current_directory.len() as u16)])
        .split(tab_bar_inner);

    let tab_titles = vec!["Kanban", "Instances", "Roadmap", "Worktree"];
    let tabs = RatTabs::new(tab_titles)
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

    let directory_display = Paragraph::new(Line::from(Span::styled(
        current_directory,
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    )))
    .alignment(Alignment::Right);

    f.render_widget(tabs, tab_bar_layout[0]);
    f.render_widget(directory_display, tab_bar_layout[1]);

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
