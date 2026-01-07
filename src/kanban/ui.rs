use super::{KanbanMode, KanbanState};
use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

const COLUMN_COLORS: [Color; 4] = [
    Color::Cyan,    // Planning
    Color::Yellow,  // In Progress
    Color::Magenta, // Review
    Color::Green,   // Done
];

const COLUMN_COLORS_SELECTED: [Color; 4] = [
    Color::LightCyan,    // Planning
    Color::LightYellow,  // In Progress
    Color::LightMagenta, // Review
    Color::LightGreen,   // Done
];

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let state = &app.kanban;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    render_columns(f, app, chunks[0]);
    render_status_bar(f, state, chunks[1]);

    match &state.mode {
        KanbanMode::AddingTask { input } => {
            render_input_dialog(f, "Add Task to Planning", input, area);
        }
        KanbanMode::EditingTask { input, .. } => {
            render_input_dialog(f, "Edit Task", input, area);
        }
        KanbanMode::ConfirmDelete { .. } => {
            render_confirm_dialog(f, "Delete this task? (y/n)", area);
        }
        KanbanMode::ConfirmMoveBack { .. } => {
            render_confirm_dialog(
                f,
                "Move back to Planning? This will terminate the Claude Code instance. (y/n)",
                area,
            );
        }
        KanbanMode::ClassifyingTask { raw_input } => {
            render_classifying_dialog(f, raw_input, area);
        }
        KanbanMode::ReviewPopup {
            task_idx,
            scroll_offset,
        } => {
            render_review_popup(f, app, *task_idx, *scroll_offset, area);
        }
        KanbanMode::Normal => {}
    }
}

fn render_columns(f: &mut Frame, app: &App, area: Rect) {
    let state = &app.kanban;
    let column_count = state.columns.len();

    let constraints = if area.width < 120 {
        vec![Constraint::Ratio(1, column_count as u32); column_count]
    } else {
        vec![Constraint::Percentage(25); column_count]
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

        let card_height = 7;
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

            render_task_card(f, app, task, card_area, is_selected_task, border_color);

            y_offset += card_height;
        }
    }
}

fn render_task_card(
    f: &mut Frame,
    app: &App,
    task: &super::Task,
    area: Rect,
    is_selected: bool,
    _accent_color: Color,
) {
    let border_style = if is_selected {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let badge_color = task.task_type.color();
    let created = task.created_at.format("%Y/%m/%d-%H:%M:%S").to_string();

    let title_max_width = area.width.saturating_sub(4) as usize;

    let claude_indicator = if let Some(instance_id) = task.instance_id {
        app.get_instance_claude_state(instance_id)
    } else {
        None
    };

    let has_indicator =
        claude_indicator.is_some() && claude_indicator != Some(crate::instance::ClaudeState::Idle);
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
        if state != crate::instance::ClaudeState::Idle {
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
        let max_desc_lines = 3;
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

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len > 3 {
        format!("{}...", &s[..max_len - 3])
    } else {
        String::new()
    }
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return lines;
    }

    let mut current_line = String::new();

    for word in words {
        let word_len = word.len();
        let space_len = if current_line.is_empty() { 0 } else { 1 };

        if current_line.len() + space_len + word_len <= max_width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }

            if word_len <= max_width {
                current_line.push_str(word);
            } else {
                current_line.push_str(&word[..max_width.saturating_sub(3)]);
                current_line.push_str("...");
            }
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn render_status_bar(f: &mut Frame, state: &KanbanState, area: Rect) {
    let mode_color = match &state.mode {
        KanbanMode::Normal => Color::Cyan,
        KanbanMode::AddingTask { .. } => Color::Green,
        KanbanMode::EditingTask { .. } => Color::Yellow,
        KanbanMode::ConfirmDelete { .. } => Color::Red,
        KanbanMode::ConfirmMoveBack { .. } => Color::LightRed,
        KanbanMode::ClassifyingTask { .. } => Color::Magenta,
        KanbanMode::ReviewPopup { .. } => Color::Cyan,
    };

    let mode_text = match &state.mode {
        KanbanMode::Normal => "NORMAL",
        KanbanMode::AddingTask { .. } => "ADD TO PLANNING",
        KanbanMode::EditingTask { .. } => "EDIT TASK",
        KanbanMode::ConfirmDelete { .. } => "CONFIRM DELETE",
        KanbanMode::ConfirmMoveBack { .. } => "CONFIRM MOVE BACK",
        KanbanMode::ClassifyingTask { .. } => "AI CLASSIFYING",
        KanbanMode::ReviewPopup { .. } => "REVIEW OUTPUT",
    };

    let help_text = if area.width < 100 {
        match &state.mode {
            KanbanMode::Normal => "hjkl/arrows:navigate  a:add  e:edit  d:delete",
            KanbanMode::ClassifyingTask { .. } => "Esc:cancel",
            KanbanMode::AddingTask { .. } | KanbanMode::EditingTask { .. } => {
                "Enter:save  Esc:cancel"
            }
            KanbanMode::ConfirmDelete { .. } => "y:yes  n:no",
            KanbanMode::ConfirmMoveBack { .. } => "y:yes  n:no",
            KanbanMode::ReviewPopup { .. } => "jk/arrows:scroll  q/Esc:close",
        }
    } else {
        match &state.mode {
            KanbanMode::Normal => {
                "↑↓/jk:task  ←→/hl:column  a:add-to-planning  e:edit  d:delete  Enter:move→  Backspace:move←  q:quit"
            }
            KanbanMode::AddingTask { .. } | KanbanMode::EditingTask { .. } => {
                "Type to enter text  Enter:save  Esc:cancel"
            }
            KanbanMode::ConfirmDelete { .. } => "y:yes  n:no  Esc:cancel",
            KanbanMode::ConfirmMoveBack { .. } => "y:yes  n:no  Esc:cancel",
            KanbanMode::ClassifyingTask { .. } => "Press Esc to cancel classification",
            KanbanMode::ReviewPopup { .. } => "↑↓/jk:scroll  PgUp/PgDown:fast-scroll  q/Esc:close",
        }
    };

    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("[{}] ", mode_text),
            Style::default().fg(mode_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(help_text, Style::default().fg(Color::DarkGray)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(status, area);
}

fn render_input_dialog(f: &mut Frame, title: &str, input: &str, area: Rect) {
    let dialog_width = if area.width < 80 { 80 } else { 60 };
    let dialog_height = if area.height < 30 { 30 } else { 20 };
    let dialog_area = centered_rect(dialog_width, dialog_height, area);

    // Render opaque background overlay covering entire screen
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        area,
    );

    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        dialog_area,
    );

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
        x: dialog_area.x + 2,
        y: dialog_area.y + 2,
        width: dialog_area.width.saturating_sub(4),
        height: dialog_area.height.saturating_sub(4),
    };

    let input_text = Paragraph::new(input)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(input_text, inner_area);

    let cursor_x = dialog_area.x + 2 + (input.len() as u16 % inner_area.width);
    let cursor_y = dialog_area.y + 2 + (input.len() as u16 / inner_area.width);
    f.set_cursor_position((cursor_x, cursor_y));
}

fn render_confirm_dialog(f: &mut Frame, message: &str, area: Rect) {
    let dialog_area = centered_rect(50, 20, area);

    // Render opaque background overlay covering entire screen
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        area,
    );

    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        dialog_area,
    );

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
        x: dialog_area.x + 2,
        y: dialog_area.y + 3,
        width: dialog_area.width.saturating_sub(4),
        height: dialog_area.height.saturating_sub(4),
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

fn render_classifying_dialog(f: &mut Frame, raw_input: &str, area: Rect) {
    let dialog_area = centered_rect(60, 30, area);

    // Render opaque background overlay covering entire screen
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        area,
    );

    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        dialog_area,
    );

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
        x: dialog_area.x + 2,
        y: dialog_area.y + 2,
        width: dialog_area.width.saturating_sub(4),
        height: dialog_area.height.saturating_sub(4),
    };

    let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let frame_index = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        / 100)
        % 10;
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

fn render_review_popup(
    f: &mut Frame,
    app: &App,
    task_idx: usize,
    scroll_offset: usize,
    area: Rect,
) {
    let dialog_area = centered_rect(90, 90, area);

    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        area,
    );

    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        dialog_area,
    );

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
                    (scroll_offset + dialog_area.height as usize).min(total_lines),
                    total_lines
                ),
                Style::default().fg(Color::DarkGray),
            )])
            .alignment(Alignment::Right),
        )
        .style(Style::default().bg(Color::Black));

    let inner_area = block.inner(dialog_area);
    f.render_widget(block, dialog_area);

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
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn get_claude_state_indicator_for_card(
    state: crate::instance::ClaudeState,
) -> (&'static str, Color) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let should_show = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => (duration.as_millis() / 500) % 2 == 0,
        Err(_) => true,
    };

    match state {
        crate::instance::ClaudeState::Idle => (" ", Color::Gray),
        crate::instance::ClaudeState::Running if should_show => ("●", Color::Green),
        crate::instance::ClaudeState::Running => (" ", Color::Green),
        crate::instance::ClaudeState::NeedsPermissions if should_show => ("●", Color::Magenta),
        crate::instance::ClaudeState::NeedsPermissions => (" ", Color::Magenta),
        crate::instance::ClaudeState::Done if should_show => ("●", Color::White),
        crate::instance::ClaudeState::Done => (" ", Color::White),
    }
}
