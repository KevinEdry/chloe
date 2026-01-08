use crate::app::{App, Tab};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let current_directory = std::env::current_dir()
        .ok()
        .and_then(|path| path.to_str().map(String::from))
        .unwrap_or_else(|| String::from("?"));

    let block = Block::default().borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(current_directory.len() as u16),
        ])
        .split(inner);

    let tab_titles = vec!["Kanban", "Instances", "Roadmap", "Worktree", "Focus"];
    let tabs = Tabs::new(tab_titles)
        .select(match app.active_tab {
            Tab::Kanban => 0,
            Tab::Instances => 1,
            Tab::Roadmap => 2,
            Tab::Worktree => 3,
            Tab::Focus => 4,
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

    frame.render_widget(tabs, layout[0]);
    frame.render_widget(directory_display, layout[1]);
}
