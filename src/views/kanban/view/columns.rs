use super::helpers::{
    COLUMN_COLORS, COLUMN_COLORS_SELECTED, get_claude_state_indicator_for_card, truncate_string,
    wrap_text,
};
use crate::app::App;
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

pub fn render_columns(f: &mut Frame, app: &App, area: Rect) {
    let state = &app.kanban;
    let column_count = state.columns.len();

    let constraints = if area.width < COLUMN_WIDTH_THRESHOLD {
        vec![Constraint::Ratio(1, column_count as u32); column_count]
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
        let is_selected = column_index == state.selected_column;
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

        if !column.tasks.is_empty() && is_selected {
            if let Some(selected_idx) = state.selected_task {
                let position_text = format!(" {} of {} ", selected_idx + 1, column.tasks.len());
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
        }

        let inner_area = column_block.inner(*chunk);
        f.render_widget(column_block, *chunk);

        if column.tasks.is_empty() {
            continue;
        }

        let card_height = TASK_CARD_HEIGHT;
        let available_height = inner_area.height;
        let cards_that_fit = (available_height / card_height) as usize;

        let start_index = if let Some(selected_idx) = state.selected_task {
            if is_selected {
                selected_idx.saturating_sub(cards_that_fit / 2)
            } else {
                0
            }
        } else {
            0
        };

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

            let is_selected_task = is_selected && state.selected_task == Some(task_index);

            render_task_card(f, app, task, card_area, is_selected_task);

            y_offset += card_height;
        }
    }
}

fn render_task_card(
    f: &mut Frame,
    app: &App,
    task: &crate::views::kanban::Task,
    area: Rect,
    is_selected: bool,
) {
    let badge_color = task.task_type.color();
    let created = task.created_at.format("%Y/%m/%d-%H:%M:%S").to_string();

    let title_max_width = area.width.saturating_sub(4) as usize;

    let claude_indicator = if let Some(instance_id) = task.instance_id {
        app.get_instance_claude_state(instance_id)
    } else {
        None
    };

    let border_color = if is_selected {
        Color::White
    } else {
        Color::DarkGray
    };

    let border_style = if is_selected {
        Style::default()
            .fg(border_color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(border_color)
    };

    let has_indicator =
        claude_indicator.is_some() && claude_indicator != Some(crate::views::instances::ClaudeState::Idle);
    let indicator_width = if has_indicator { 2 } else { 0 };

    let available_title_width = title_max_width.saturating_sub(8 + indicator_width);

    let mut title_spans = vec![
        Span::raw(" "),
        Span::styled(
            format!("[{}]", task.task_type.badge_text()),
            Style::default()
                .fg(badge_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
    ];

    if let Some(state) = claude_indicator {
        if state != crate::views::instances::ClaudeState::Idle {
            let (indicator, color) = get_claude_state_indicator_for_card(state);
            title_spans.push(Span::styled(
                format!("{} ", indicator),
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

    let title_line = Line::from(title_spans);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title_line);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let max_width = inner.width.saturating_sub(2) as usize;

    let mut lines = vec![];

    if !task.description.is_empty() {
        let max_desc_lines = MAX_DESCRIPTION_LINES;
        let wrapped_lines = wrap_text(&task.description, max_width);
        let has_more = wrapped_lines.len() > max_desc_lines;

        for (index, desc_line) in wrapped_lines.iter().take(max_desc_lines).enumerate() {
            let line_text = if has_more && index == max_desc_lines - 1 {
                format!("{}...", desc_line)
            } else {
                desc_line.clone()
            };

            lines.push(Line::from(Span::styled(
                line_text,
                Style::default().fg(Color::Gray),
            )));
        }
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        format!("Created: {}", created),
        Style::default().fg(Color::DarkGray),
    )));

    let text = Paragraph::new(lines);

    f.render_widget(text, inner);
}
