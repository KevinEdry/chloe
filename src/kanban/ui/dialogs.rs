use super::helpers::{centered_rect, render_popup_background};
use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

const DIALOG_WIDTH_THRESHOLD: u16 = 80;
const DIALOG_WIDTH_SMALL: u16 = 80;
const DIALOG_WIDTH_NORMAL: u16 = 60;
const DIALOG_HEIGHT_THRESHOLD: u16 = 30;
const DIALOG_HEIGHT_SMALL: u16 = 30;
const DIALOG_HEIGHT_NORMAL: u16 = 20;

const CONFIRM_DIALOG_WIDTH_PERCENT: u16 = 50;
const CONFIRM_DIALOG_HEIGHT_PERCENT: u16 = 20;

const CLASSIFYING_DIALOG_WIDTH_PERCENT: u16 = 60;
const CLASSIFYING_DIALOG_HEIGHT_PERCENT: u16 = 30;

const REVIEW_POPUP_WIDTH_PERCENT: u16 = 90;
const REVIEW_POPUP_HEIGHT_PERCENT: u16 = 90;

const DIALOG_PADDING: u16 = 2;
const DIALOG_VERTICAL_PADDING: u16 = 3;
const DIALOG_PADDING_DOUBLE: u16 = 4;

const BUTTON_COUNT: usize = 4;
const BUTTON_WIDTH_PERCENT: u16 = 25;

const SPINNER_FRAME_DURATION_MS: u128 = 100;
const SPINNER_FRAME_COUNT: u128 = 10;

const EXIT_CONFIRM_DIALOG_WIDTH_PERCENT: u16 = 50;
const EXIT_CONFIRM_DIALOG_HEIGHT_PERCENT: u16 = 25;

pub fn render_input_dialog(f: &mut Frame, title: &str, input: &str, area: Rect) {
    let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
        DIALOG_WIDTH_SMALL
    } else {
        DIALOG_WIDTH_NORMAL
    };
    let dialog_height = if area.height < DIALOG_HEIGHT_THRESHOLD {
        DIALOG_HEIGHT_SMALL
    } else {
        DIALOG_HEIGHT_NORMAL
    };
    let dialog_area = centered_rect(dialog_width, dialog_height, area);

    render_popup_background(f, dialog_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .title(Span::styled(
            format!(" {} ", title),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(Color::Black));

    f.render_widget(block, dialog_area);

    let inner_area = Rect {
        x: dialog_area.x + DIALOG_PADDING,
        y: dialog_area.y + DIALOG_PADDING,
        width: dialog_area.width.saturating_sub(DIALOG_PADDING_DOUBLE),
        height: dialog_area.height.saturating_sub(DIALOG_PADDING_DOUBLE),
    };

    let input_text = Paragraph::new(input)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: false });

    f.render_widget(input_text, inner_area);

    let cursor_x = dialog_area.x + DIALOG_PADDING + (input.len() as u16 % inner_area.width);
    let cursor_y = dialog_area.y + DIALOG_PADDING + (input.len() as u16 / inner_area.width);
    f.set_cursor_position((cursor_x, cursor_y));
}

pub fn render_confirm_dialog(f: &mut Frame, message: &str, area: Rect) {
    let dialog_area = centered_rect(
        CONFIRM_DIALOG_WIDTH_PERCENT,
        CONFIRM_DIALOG_HEIGHT_PERCENT,
        area,
    );

    render_popup_background(f, dialog_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .title(Span::styled(
            " ⚠ Confirm ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(Color::Black));

    f.render_widget(block, dialog_area);

    let inner_area = Rect {
        x: dialog_area.x + DIALOG_PADDING,
        y: dialog_area.y + DIALOG_VERTICAL_PADDING,
        width: dialog_area.width.saturating_sub(DIALOG_PADDING_DOUBLE),
        height: dialog_area.height.saturating_sub(DIALOG_PADDING_DOUBLE),
    };

    let text = Paragraph::new(message)
        .style(
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    f.render_widget(text, inner_area);
}

pub fn render_classifying_dialog(f: &mut Frame, raw_input: &str, area: Rect) {
    let dialog_area = centered_rect(
        CLASSIFYING_DIALOG_WIDTH_PERCENT,
        CLASSIFYING_DIALOG_HEIGHT_PERCENT,
        area,
    );

    render_popup_background(f, dialog_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .title(Span::styled(
            " AI Classification ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(Color::Black));

    f.render_widget(block, dialog_area);

    let inner_area = Rect {
        x: dialog_area.x + DIALOG_PADDING,
        y: dialog_area.y + DIALOG_PADDING,
        width: dialog_area.width.saturating_sub(DIALOG_PADDING_DOUBLE),
        height: dialog_area.height.saturating_sub(DIALOG_PADDING_DOUBLE),
    };

    let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let frame_index = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        / SPINNER_FRAME_DURATION_MS)
        % SPINNER_FRAME_COUNT;
    let spinner = spinner_frames[frame_index as usize];

    let lines = vec![
        Line::from(vec![
            Span::styled(
                spinner,
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Analyzing task with Claude..."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Input: ", Style::default().fg(Color::Gray)),
            Span::styled(raw_input, Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc to cancel",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    let text = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    f.render_widget(text, inner_area);
}

pub fn render_review_popup(
    f: &mut Frame,
    app: &App,
    task_idx: usize,
    scroll_offset: usize,
    selected_action: crate::kanban::ReviewAction,
    area: Rect,
) {
    let dialog_area = centered_rect(
        REVIEW_POPUP_WIDTH_PERCENT,
        REVIEW_POPUP_HEIGHT_PERCENT,
        area,
    );

    render_popup_background(f, dialog_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(dialog_area);

    let review_column_index = 2;
    let task = app
        .kanban
        .columns
        .get(review_column_index)
        .and_then(|column| column.tasks.get(task_idx));

    let output_text = if let Some(task) = task {
        if let Some(instance_id) = task.instance_id {
            app.get_instance_output(instance_id)
                .unwrap_or("No output available")
        } else {
            "No instance associated with this task"
        }
    } else {
        "Task not found"
    };

    let output_lines: Vec<&str> = output_text.lines().collect();
    let total_lines = output_lines.len();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .title(Span::styled(
            " Claude Code Output ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .title_bottom(
            Line::from(vec![Span::styled(
                format!(
                    " Lines {}-{} of {} ",
                    scroll_offset + 1,
                    (scroll_offset + chunks[0].height as usize).min(total_lines),
                    total_lines
                ),
                Style::default().fg(Color::DarkGray),
            )])
            .alignment(Alignment::Right),
        )
        .style(Style::default().bg(Color::Black));

    let inner_area = block.inner(chunks[0]);
    f.render_widget(block, chunks[0]);

    let visible_lines: Vec<Line> = output_lines
        .iter()
        .skip(scroll_offset)
        .take(inner_area.height as usize)
        .map(|line| Line::from(Span::styled(*line, Style::default().fg(Color::White))))
        .collect();

    let text = Paragraph::new(visible_lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    f.render_widget(text, inner_area);

    render_action_buttons(f, selected_action, chunks[1]);
}

fn render_action_buttons(f: &mut Frame, selected_action: crate::kanban::ReviewAction, area: Rect) {
    let actions = crate::kanban::ReviewAction::all();
    let button_constraints = vec![Constraint::Percentage(BUTTON_WIDTH_PERCENT); BUTTON_COUNT];

    let button_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(button_constraints)
        .split(area);

    for (index, action) in actions.iter().enumerate() {
        let is_selected = *action == selected_action;
        let style = if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan).bg(Color::Black)
        };

        let button = Paragraph::new(action.label())
            .alignment(Alignment::Center)
            .style(style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(if is_selected {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }),
            );

        f.render_widget(button, button_areas[index]);
    }
}

pub fn render_exit_confirmation_dialog(f: &mut Frame, area: Rect) {
    let dialog_area = centered_rect(
        EXIT_CONFIRM_DIALOG_WIDTH_PERCENT,
        EXIT_CONFIRM_DIALOG_HEIGHT_PERCENT,
        area,
    );

    render_popup_background(f, dialog_area);

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

    f.render_widget(block, dialog_area);

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

    f.render_widget(text, inner_area);
}
