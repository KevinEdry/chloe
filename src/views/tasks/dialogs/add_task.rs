use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
};

use super::{centered_rect, render_popup_background};

const POPUP_WIDTH_PERCENT: u16 = 70;
const POPUP_HEIGHT_PERCENT: u16 = 45;
const TIP_BLOCK_HEIGHT: u16 = 7;
const VERTICAL_GAP: u16 = 1;

pub struct AddTaskDialogState<'a> {
    pub input: &'a str,
    pub prompt: &'a str,
}

pub fn render_add_task_dialog(frame: &mut Frame, state: &AddTaskDialogState<'_>, area: Rect) {
    let popup_area = centered_rect(POPUP_WIDTH_PERCENT, POPUP_HEIGHT_PERCENT, area);
    render_popup_background(frame, popup_area);

    let outer_block = Block::default()
        .title(" Add Task ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let inner_area = outer_block.inner(popup_area);
    frame.render_widget(outer_block, popup_area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(VERTICAL_GAP),
            Constraint::Length(TIP_BLOCK_HEIGHT),
        ])
        .split(inner_area);

    render_input_area(frame, layout[0], state);
    render_tip_block(frame, layout[2]);
}

fn render_input_area(frame: &mut Frame, area: Rect, state: &AddTaskDialogState<'_>) {
    let content = vec![
        Line::from(Span::styled(
            state.prompt,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(state.input.to_string(), Style::default().fg(Color::White)),
            Span::styled("▏", Style::default().fg(Color::Cyan)),
        ]),
    ];

    frame.render_widget(Paragraph::new(content), area);
}

fn render_tip_block(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" How It Works ")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Just describe your task briefly - an AI agent will",
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "automatically expand it into a full task with title,",
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "description, and relevant tags.",
            Style::default().fg(Color::Gray),
        )),
    ];

    frame.render_widget(Paragraph::new(content), inner);
}
