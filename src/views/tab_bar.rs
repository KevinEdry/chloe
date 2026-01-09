use crate::app::{App, Tab};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const TAB_COLORS: [Color; 5] = [
    Color::Cyan,      // Kanban
    Color::Yellow,    // Instances
    Color::Magenta,   // Roadmap
    Color::Green,     // Worktree
    Color::LightBlue, // Focus
];

const TAB_COLORS_INACTIVE: [Color; 5] = [
    Color::DarkGray,
    Color::DarkGray,
    Color::DarkGray,
    Color::DarkGray,
    Color::DarkGray,
];

const TAB_NAMES: [&str; 5] = ["Kanban", "Instances", "Roadmap", "Worktree", "Focus"];

const DIRECTORY_PADDING: u16 = 2;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let current_directory = std::env::current_dir()
        .ok()
        .and_then(|path| path.to_str().map(String::from))
        .unwrap_or_else(|| String::from("?"));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let directory_width = current_directory.len() as u16 + DIRECTORY_PADDING;

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(directory_width)])
        .split(inner);

    let selected_index = match app.active_tab {
        Tab::Kanban => 0,
        Tab::Instances => 1,
        Tab::Roadmap => 2,
        Tab::Worktree => 3,
        Tab::Focus => 4,
    };

    let tab_spans: Vec<Span> = TAB_NAMES
        .iter()
        .enumerate()
        .flat_map(|(index, name)| {
            let is_selected = index == selected_index;
            let color = if is_selected {
                TAB_COLORS[index]
            } else {
                TAB_COLORS_INACTIVE[index]
            };

            let tab_style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(color)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };

            let bracket_style = Style::default().fg(color);

            let mut spans = vec![];

            if is_selected {
                spans.push(Span::styled(format!(" {} ", name), tab_style));
            } else {
                spans.push(Span::styled(" [", bracket_style));
                spans.push(Span::styled(*name, tab_style));
                spans.push(Span::styled("] ", bracket_style));
            }

            spans
        })
        .collect();

    let tabs_line = Paragraph::new(Line::from(tab_spans));

    let directory_display = Paragraph::new(Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled(
            current_directory,
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        ),
    ]))
    .alignment(Alignment::Right);

    frame.render_widget(tabs_line, layout[0]);
    frame.render_widget(directory_display, layout[1]);
}
