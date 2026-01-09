use super::{centered_rect, render_popup_background};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

const EXIT_CONFIRM_DIALOG_WIDTH_PERCENT: u16 = 50;
const EXIT_CONFIRM_DIALOG_HEIGHT_PERCENT: u16 = 25;

const DIALOG_PADDING: u16 = 2;
const DIALOG_VERTICAL_PADDING: u16 = 3;
const DIALOG_PADDING_DOUBLE: u16 = 4;

pub fn render_exit_confirmation_dialog(frame: &mut Frame, area: Rect) {
    let dialog_area = centered_rect(
        EXIT_CONFIRM_DIALOG_WIDTH_PERCENT,
        EXIT_CONFIRM_DIALOG_HEIGHT_PERCENT,
        area,
    );

    render_popup_background(frame, dialog_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .title(Span::styled(
            " Confirm Exit ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block, dialog_area);

    let inner_area = Rect {
        x: dialog_area.x + DIALOG_PADDING,
        y: dialog_area.y + DIALOG_VERTICAL_PADDING,
        width: dialog_area.width.saturating_sub(DIALOG_PADDING_DOUBLE),
        height: dialog_area.height.saturating_sub(DIALOG_PADDING_DOUBLE),
    };

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Are you sure you want to exit?",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::Gray)),
            Span::styled(
                "Y",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to confirm or ", Style::default().fg(Color::Gray)),
            Span::styled(
                "N",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("/", Style::default().fg(Color::Gray)),
            Span::styled(
                "Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to cancel", Style::default().fg(Color::Gray)),
        ]),
    ];

    let text = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(text, inner_area);
}
