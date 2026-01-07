use super::{KanbanMode, KanbanState};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

const COLUMN_COLORS: [Color; 4] = [
    Color::Blue,      // Planning
    Color::Yellow,    // In Progress
    Color::Magenta,   // Review
    Color::Green,     // Done
];

const COLUMN_TASK_COLORS: [Color; 4] = [
    Color::Cyan,      // Planning tasks
    Color::LightYellow, // In Progress tasks
    Color::LightMagenta, // Review tasks
    Color::LightGreen,   // Done tasks
];

pub fn render(f: &mut Frame, state: &KanbanState) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    render_columns(f, state, chunks[0]);
    render_status_bar(f, state, chunks[1]);

    match &state.mode {
        KanbanMode::AddingTask { input } => {
            render_input_dialog(f, "Add New Task", input, area);
        }
        KanbanMode::EditingTask { input, .. } => {
            render_input_dialog(f, "Edit Task", input, area);
        }
        KanbanMode::ConfirmDelete { .. } => {
            render_confirm_dialog(f, "Delete this task? (y/n)", area);
        }
        KanbanMode::Normal => {}
    }
}

fn render_columns(f: &mut Frame, state: &KanbanState, area: Rect) {
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

    for (column_index, (column, chunk)) in state.columns.iter().zip(column_chunks.iter()).enumerate() {
        let is_selected = column_index == state.selected_column;
        let column_color = COLUMN_COLORS[column_index];
        let task_color = COLUMN_TASK_COLORS[column_index];

        let items: Vec<ListItem> = column
            .tasks
            .iter()
            .enumerate()
            .map(|(task_index, task)| {
                let is_selected_task = is_selected && state.selected_task == Some(task_index);

                let task_style = if is_selected_task {
                    Style::default()
                        .fg(Color::Black)
                        .bg(task_color)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(task_color)
                };

                let content = Line::from(vec![
                    Span::styled(
                        if is_selected_task { "► " } else { "  " },
                        Style::default().fg(column_color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(&task.title, task_style),
                ]);

                ListItem::new(content)
            })
            .collect();

        let border_style = if is_selected {
            Style::default()
                .fg(column_color)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(column_color)
        };

        let title_style = if is_selected {
            Style::default()
                .fg(column_color)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(column_color)
        };

        let task_count_text = format!(" {} ({}) ", column.name, column.tasks.len());

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(Span::styled(task_count_text, title_style)),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_widget(list, *chunk);
    }
}

fn render_status_bar(f: &mut Frame, state: &KanbanState, area: Rect) {
    let mode_color = match &state.mode {
        KanbanMode::Normal => Color::Cyan,
        KanbanMode::AddingTask { .. } => Color::Green,
        KanbanMode::EditingTask { .. } => Color::Yellow,
        KanbanMode::ConfirmDelete { .. } => Color::Red,
    };

    let mode_text = match &state.mode {
        KanbanMode::Normal => "NORMAL",
        KanbanMode::AddingTask { .. } => "ADD TASK",
        KanbanMode::EditingTask { .. } => "EDIT TASK",
        KanbanMode::ConfirmDelete { .. } => "CONFIRM DELETE",
    };

    let help_text = if area.width < 100 {
        match &state.mode {
            KanbanMode::Normal => "hjkl/arrows:navigate  a:add  e:edit  d:delete",
            KanbanMode::AddingTask { .. } | KanbanMode::EditingTask { .. } => {
                "Enter:save  Esc:cancel"
            }
            KanbanMode::ConfirmDelete { .. } => "y:yes  n:no",
        }
    } else {
        match &state.mode {
            KanbanMode::Normal => {
                "↑↓/jk:task  ←→/hl:column  a:add  e:edit  d:delete  Enter:move→  Backspace:move←  q:quit"
            }
            KanbanMode::AddingTask { .. } | KanbanMode::EditingTask { .. } => {
                "Type to enter text  Enter:save  Esc:cancel"
            }
            KanbanMode::ConfirmDelete { .. } => "y:yes  n:no  Esc:cancel",
        }
    };

    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("[{}] ", mode_text),
            Style::default()
                .fg(mode_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(help_text, Style::default().fg(Color::DarkGray)),
    ]))
    .block(Block::default().borders(Borders::ALL).border_style(
        Style::default().fg(Color::DarkGray),
    ));

    f.render_widget(status, area);
}

fn render_input_dialog(f: &mut Frame, title: &str, input: &str, area: Rect) {
    let dialog_width = if area.width < 80 { 80 } else { 60 };
    let dialog_height = if area.height < 30 { 30 } else { 20 };
    let dialog_area = centered_rect(dialog_width, dialog_height, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
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
        .style(Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
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
