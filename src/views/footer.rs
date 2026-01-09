use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const VERSION_TEXT: &str = "Chloe v0.1.0";
const VERSION_TEXT_LENGTH: u16 = 13;
const MINIMUM_SPACE_FOR_VERSION: u16 = 15;

pub struct StatusBarContent {
    pub mode_text: String,
    pub mode_color: Color,
    pub extra_info: Option<String>,
    pub help_text: String,
}

pub fn render_footer(frame: &mut Frame, area: Rect, content: StatusBarContent) {
    let inner_area = Block::default().borders(Borders::ALL).inner(area);
    let should_show_version = inner_area.width >= MINIMUM_SPACE_FOR_VERSION;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(VERSION_TEXT_LENGTH)])
        .split(inner_area);

    let mut spans = vec![Span::styled(
        format!("[{}] ", content.mode_text),
        Style::default()
            .fg(content.mode_color)
            .add_modifier(Modifier::BOLD),
    )];

    if let Some(extra_info) = content.extra_info {
        spans.push(Span::styled(extra_info, Style::default().fg(Color::Gray)));
    }

    spans.push(Span::styled(
        content.help_text,
        Style::default().fg(Color::DarkGray),
    ));

    let status = Paragraph::new(Line::from(spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(status, area);

    if should_show_version {
        let version = Paragraph::new(VERSION_TEXT)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Right);
        frame.render_widget(version, chunks[1]);
    }
}
