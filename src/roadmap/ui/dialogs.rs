use crate::roadmap::RoadmapState;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap},
};

const DIALOG_WIDTH_THRESHOLD: u16 = 80;
const DIALOG_WIDTH_SMALL: u16 = 80;
const DIALOG_WIDTH_NORMAL: u16 = 60;
const DIALOG_HEIGHT: u16 = 7;
const DETAILS_POPUP_HEIGHT_PERCENT: u16 = 90;
const DETAILS_POPUP_WIDTH_PERCENT: u16 = 90;

pub fn render_input_dialog(f: &mut Frame, title: &str, input: &str, area: Rect) {
    let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
        DIALOG_WIDTH_SMALL
    } else {
        DIALOG_WIDTH_NORMAL
    };

    let popup_area = centered_rect(dialog_width, DIALOG_HEIGHT, area);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);

    f.render_widget(Clear, popup_area);
    f.render_widget(block, popup_area);

    let input_text = Paragraph::new(input)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(input_text, inner_area);
}

pub fn render_confirm_dialog(f: &mut Frame, message: &str, area: Rect) {
    let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
        DIALOG_WIDTH_SMALL
    } else {
        DIALOG_WIDTH_NORMAL
    };

    let popup_area = centered_rect(dialog_width, DIALOG_HEIGHT, area);

    let block = Block::default()
        .title("Confirm")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);

    f.render_widget(Clear, popup_area);
    f.render_widget(block, popup_area);

    let text = Paragraph::new(message)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    f.render_widget(text, inner_area);
}

pub fn render_convert_dialog(
    f: &mut Frame,
    state: &RoadmapState,
    item_index: usize,
    area: Rect,
) {
    let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
        DIALOG_WIDTH_SMALL
    } else {
        DIALOG_WIDTH_NORMAL
    };

    let popup_area = centered_rect(dialog_width, 10, area);

    let block = Block::default()
        .title("Convert to Task")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);

    f.render_widget(Clear, popup_area);
    f.render_widget(block, popup_area);

    if let Some(item) = state.items.get(item_index) {
        let message = vec![
            Line::from(vec![Span::styled(
                "Convert this roadmap item to a task in Planning column?",
                Style::default().fg(Color::White),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Title: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&item.title, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Press 'y' to confirm, 'n' to cancel",
                Style::default().fg(Color::DarkGray),
            )]),
        ];

        let text = Paragraph::new(message).alignment(Alignment::Left);
        f.render_widget(text, inner_area);
    }
}

pub fn render_details_view(
    f: &mut Frame,
    state: &RoadmapState,
    item_index: usize,
    scroll_offset: usize,
    area: Rect,
) {
    let popup_width = (area.width * DETAILS_POPUP_WIDTH_PERCENT) / 100;
    let popup_height = (area.height * DETAILS_POPUP_HEIGHT_PERCENT) / 100;
    let popup_area = centered_rect(popup_width, popup_height, area);

    let block = Block::default()
        .title("Roadmap Item Details")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);

    f.render_widget(Clear, popup_area);
    f.render_widget(block, popup_area);

    if let Some(item) = state.items.get(item_index) {
        let mut lines = vec![
            Line::from(vec![
                Span::styled("Title: ", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)),
                Span::styled(&item.title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    item.priority.label(),
                    Style::default().fg(item.priority.color()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    item.status.label(),
                    Style::default().fg(item.status.color()),
                ),
            ]),
            Line::from(""),
        ];

        if !item.description.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Description:",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(Span::styled(
                &item.description,
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(""));
        }

        if !item.rationale.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Rationale:",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(Span::styled(
                &item.rationale,
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(""));
        }

        if !item.user_stories.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "User Stories:",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
            )]));
            for story in &item.user_stories {
                lines.push(Line::from(vec![
                    Span::raw("  • "),
                    Span::styled(story, Style::default().fg(Color::White)),
                ]));
            }
            lines.push(Line::from(""));
        }

        if !item.acceptance_criteria.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Acceptance Criteria:",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
            )]));
            for criterion in &item.acceptance_criteria {
                lines.push(Line::from(vec![
                    Span::raw("  • "),
                    Span::styled(criterion, Style::default().fg(Color::White)),
                ]));
            }
            lines.push(Line::from(""));
        }

        if !item.tags.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Tags:",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
            )]));
            let tags_text = item.tags.join(", ");
            lines.push(Line::from(Span::styled(
                tags_text,
                Style::default().fg(Color::Magenta),
            )));
        }

        let skip_lines = scroll_offset.min(lines.len().saturating_sub(1));
        let visible_lines: Vec<Line> = lines.into_iter().skip(skip_lines).collect();

        let text = Paragraph::new(visible_lines)
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Left);

        f.render_widget(text, inner_area);
    }
}

pub fn render_loading_dialog(f: &mut Frame, state: &RoadmapState, area: Rect) {
    let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
        DIALOG_WIDTH_SMALL
    } else {
        DIALOG_WIDTH_NORMAL
    };

    let popup_area = centered_rect(dialog_width, 10, area);

    let block = Block::default()
        .title("AI Roadmap Generation")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);

    f.render_widget(Clear, popup_area);
    f.render_widget(block, popup_area);

    let spinner = state.get_spinner_char();

    let message = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("  {}  ", spinner),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Analyzing your project...",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Claude is discovering features and creating",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![Span::styled(
            "a strategic roadmap for your project.",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Press Esc to cancel",
            Style::default().fg(Color::Yellow),
        )]),
    ];

    let text = Paragraph::new(message)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    f.render_widget(text, inner_area);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical_margin = area.height.saturating_sub(height) / 2;
    let horizontal_margin = area.width.saturating_sub(width) / 2;

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(horizontal_margin),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}
