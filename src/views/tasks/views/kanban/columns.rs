use super::helpers::{
    COLUMN_COLORS, COLUMN_COLORS_SELECTED, get_claude_state_indicator_for_card, truncate_string,
    wrap_text,
};
use crate::app::App;
use crate::views::tasks::Task;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const COLUMN_WIDTH_THRESHOLD: u16 = 120;
const COLUMN_WIDTH_PERCENT: u16 = 25;
const TASK_CARD_HEIGHT: u16 = 7;
const MAX_DESCRIPTION_LINES: usize = 3;

pub fn render_columns(frame: &mut Frame, app: &App, area: Rect) {
    let state = &app.tasks;
    let column_count = state.columns.len();

    #[allow(clippy::cast_possible_truncation)]
    let column_count_u32 = column_count as u32;

    let constraints = if area.width < COLUMN_WIDTH_THRESHOLD {
        vec![Constraint::Ratio(1, column_count_u32); column_count]
    } else {
        vec![Constraint::Percentage(COLUMN_WIDTH_PERCENT); column_count]
    };

    let column_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (column_index, (column, chunk)) in
        state.columns.iter().zip(column_chunks.iter()).enumerate()
    {
        let is_selected = column_index == state.kanban_selected_column;
        let border_color = if is_selected {
            COLUMN_COLORS_SELECTED[column_index]
        } else {
            COLUMN_COLORS[column_index]
        };

        let border_style = Style::default()
            .fg(border_color)
            .add_modifier(if is_selected {
                Modifier::BOLD
            } else {
                Modifier::empty()
            });

        let indicator = if is_selected { "→ " } else { "" };
        let name_with_indicator = format!("{}{}", indicator, column.name);

        let title_text = Line::from(vec![
            Span::raw(" "),
            Span::styled(
                name_with_indicator,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ]);

        let mut column_block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(title_text);

        if !column.tasks.is_empty()
            && is_selected
            && let Some(selected_index) = state.kanban_selected_task
        {
            let position_text = format!(" {} of {} ", selected_index + 1, column.tasks.len());
            column_block = column_block.title_bottom(
                Line::from(vec![Span::styled(
                    position_text,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )])
                .alignment(Alignment::Right),
            );
        }

        let inner_area = column_block.inner(*chunk);
        frame.render_widget(column_block, *chunk);

        if column.tasks.is_empty() {
            continue;
        }

        let card_height = TASK_CARD_HEIGHT;
        let available_height = inner_area.height;
        let cards_that_fit = (available_height / card_height) as usize;

        let start_index = state.kanban_selected_task.map_or(0, |selected_index| {
            if is_selected {
                selected_index.saturating_sub(cards_that_fit / 2)
            } else {
                0
            }
        });

        let visible_tasks = column
            .tasks
            .iter()
            .enumerate()
            .skip(start_index)
            .take(cards_that_fit);

        let mut y_offset = 0;

        for (task_index, task) in visible_tasks {
            if y_offset + card_height > available_height {
                break;
            }

            let card_area = Rect {
                x: inner_area.x,
                y: inner_area.y + y_offset,
                width: inner_area.width,
                height: card_height.min(available_height - y_offset),
            };

            let is_selected_task = is_selected && state.kanban_selected_task == Some(task_index);

            render_task_card(frame, app, task, card_area, is_selected_task);

            y_offset += card_height;
        }
    }
}

const CLASSIFYING_INDICATOR: &str = "◎";
const CLASSIFYING_INDICATOR_WIDTH: usize = 2;

fn render_task_card(frame: &mut Frame, app: &App, task: &Task, area: Rect, is_selected: bool) {
    let claude_indicator = task
        .instance_id
        .and_then(|instance_id| app.get_instance_claude_state(instance_id));

    let border_color = if task.is_classifying {
        Color::Yellow
    } else if is_selected {
        Color::White
    } else {
        Color::DarkGray
    };

    let border_style = if is_selected || task.is_classifying {
        Style::default()
            .fg(border_color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(border_color)
    };

    let title_max_width = area.width.saturating_sub(4) as usize;
    let title_line = build_task_card_title(task, claude_indicator, title_max_width);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title_line);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let max_width = inner.width.saturating_sub(2) as usize;
    let lines = build_task_card_content(task, max_width);

    frame.render_widget(Paragraph::new(lines), inner);
}

fn build_task_card_title(
    task: &Task,
    claude_indicator: Option<crate::views::instances::ClaudeState>,
    title_max_width: usize,
) -> Line<'_> {
    let badge_color = if task.is_classifying {
        Color::Yellow
    } else {
        task.kind.color()
    };

    let has_indicator = claude_indicator.is_some()
        && claude_indicator != Some(crate::views::instances::ClaudeState::Idle);
    let indicator_width = if has_indicator {
        2
    } else if task.is_classifying {
        CLASSIFYING_INDICATOR_WIDTH
    } else {
        0
    };

    let available_title_width = title_max_width.saturating_sub(8 + indicator_width);

    let mut title_spans = vec![Span::raw(" ")];

    if task.is_classifying {
        title_spans.push(Span::styled(
            format!("{CLASSIFYING_INDICATOR} "),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        title_spans.push(Span::styled(
            format!("[{}]", task.kind.badge_text()),
            Style::default()
                .fg(badge_color)
                .add_modifier(Modifier::BOLD),
        ));
        title_spans.push(Span::raw(" "));

        if let Some(state) = claude_indicator
            && state != crate::views::instances::ClaudeState::Idle
        {
            let (indicator, color) = get_claude_state_indicator_for_card(state);
            title_spans.push(Span::styled(
                format!("{indicator} "),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
        }
    }

    title_spans.push(Span::styled(
        truncate_string(&task.title, available_title_width),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ));
    title_spans.push(Span::raw(" "));

    Line::from(title_spans)
}

fn build_task_card_content(task: &Task, max_width: usize) -> Vec<Line<'static>> {
    let mut lines = vec![];

    if task.is_classifying {
        lines.push(Line::from(Span::styled(
            "Classifying with AI...",
            Style::default().fg(Color::Yellow),
        )));
        lines.push(Line::from(""));
    } else if !task.description.is_empty() {
        let wrapped_lines = wrap_text(&task.description, max_width);
        let has_more = wrapped_lines.len() > MAX_DESCRIPTION_LINES;

        for (index, description_line) in
            wrapped_lines.iter().take(MAX_DESCRIPTION_LINES).enumerate()
        {
            let line_text = if has_more && index == MAX_DESCRIPTION_LINES - 1 {
                format!("{description_line}...")
            } else {
                description_line.clone()
            };

            lines.push(Line::from(Span::styled(
                line_text,
                Style::default().fg(Color::Gray),
            )));
        }
        lines.push(Line::from(""));
    }

    let created = task.created_at.format("%Y/%m/%d-%H:%M:%S").to_string();
    lines.push(Line::from(Span::styled(
        format!("Created: {created}"),
        Style::default().fg(Color::DarkGray),
    )));

    lines
}
