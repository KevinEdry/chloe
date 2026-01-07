pub mod widgets;
pub mod styles;

use crate::app::{App, Tab};
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders, Tabs as RatTabs}, style::{Color, Modifier, Style}, text::Line};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Min(0),    // Content
        ])
        .split(f.area());

    // Render tab bar
    let tab_titles = vec!["Kanban", "Terminals"];
    let tabs = RatTabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Chloe"))
        .select(match app.active_tab {
            Tab::Kanban => 0,
            Tab::Terminals => 1,
        })
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    f.render_widget(tabs, chunks[0]);

    // Render active tab content
    match app.active_tab {
        Tab::Kanban => crate::kanban::ui::render(f, &app.kanban),
        Tab::Terminals => crate::terminal::ui::render(f, &app.terminals),
    }
}
