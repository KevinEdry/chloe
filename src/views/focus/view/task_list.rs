use crate::app::App;
use crate::views::focus::state::FocusPanel;
use crate::views::instances::ClaudeState;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

const TITLE_ELLIPSIS_THRESHOLD: usize = 25;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let state = &app.focus;
    let columns = &app.kanban.columns;
    let is_focused = state.focused_panel == FocusPanel::ActiveTasks;

    let active_task_count: usize = columns.iter().take(3).map(|c| c.tasks.len()).sum();

    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };
    let title_color = if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let mut block = Block::default()
        .title("Tasks")
        .title_style(
            Style::default()
                .fg(title_color)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    if active_task_count > 0 {
        let position_text = format!(
            " {} of {} ",
            state.active_selected_index + 1,
            active_task_count
        );
        block = block.title_bottom(
            Line::from(vec![Span::styled(
                position_text,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )])
            .alignment(Alignment::Right),
        );
    }

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if active_task_count == 0 {
        render_empty_state(frame, inner_area);
        return;
    }

    let items = build_task_list_items(app, is_focused);
    let list = List::new(items);
    frame.render_widget(list, inner_area);
}

fn render_empty_state(frame: &mut Frame, area: Rect) {
    let empty_message = List::new(vec![
        ListItem::new(Line::from("")),
        ListItem::new(Line::from(vec![Span::styled(
            "No tasks yet",
            Style::default().fg(Color::DarkGray),
        )])),
        ListItem::new(Line::from("")),
        ListItem::new(Line::from(vec![Span::styled(
            "Go to Kanban to add tasks",
            Style::default().fg(Color::DarkGray),
        )])),
    ]);
    frame.render_widget(empty_message, area);
}

fn build_task_list_items(app: &App, is_panel_focused: bool) -> Vec<ListItem<'static>> {
    let mut items = Vec::new();
    let mut current_index = 0;
    let columns = &app.kanban.columns;
    let selected_index = app.focus.active_selected_index;

    for (column_index, column) in columns.iter().take(3).enumerate() {
        if !column.tasks.is_empty() {
            items.push(create_column_header(&column.name, column_index));

            for task in &column.tasks {
                let is_selected = is_panel_focused && current_index == selected_index;
                let instance_id = task.instance_id;
                let claude_state = instance_id.and_then(|id| app.get_instance_claude_state(id));

                items.push(create_task_item(
                    &task.title,
                    task.task_type,
                    is_selected,
                    claude_state,
                ));
                current_index += 1;
            }
        }
    }

    items
}

fn create_column_header(name: &str, column_index: usize) -> ListItem<'static> {
    let header_color = match column_index {
        0 => Color::Yellow,
        1 => Color::Cyan,
        2 => Color::Magenta,
        3 => Color::Green,
        _ => Color::Gray,
    };

    ListItem::new(Line::from(vec![Span::styled(
        format!(" {}", name.to_uppercase()),
        Style::default()
            .fg(header_color)
            .add_modifier(Modifier::BOLD),
    )]))
}

fn create_task_item(
    title: &str,
    task_type: crate::views::kanban::TaskType,
    is_selected: bool,
    claude_state: Option<ClaudeState>,
) -> ListItem<'static> {
    let truncated_title = if title.len() > TITLE_ELLIPSIS_THRESHOLD {
        format!("{}...", &title[..TITLE_ELLIPSIS_THRESHOLD])
    } else {
        title.to_string()
    };

    let badge_text = task_type.badge_text();
    let badge_color = task_type.color();

    let title_color = if is_selected {
        Color::White
    } else {
        Color::Gray
    };

    let mut spans = vec![
        if is_selected {
            Span::styled("▶ ", Style::default().fg(Color::Cyan))
        } else {
            Span::raw("  ")
        },
        Span::styled(
            format!("[{}]", badge_text),
            Style::default().fg(badge_color),
        ),
        Span::raw(" "),
        Span::styled(
            truncated_title,
            Style::default()
                .fg(title_color)
                .add_modifier(if is_selected {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ),
    ];

    if let Some(state) = claude_state {
        let (indicator, color) = get_claude_state_indicator(state);
        if !indicator.is_empty() {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                indicator.to_string(),
                Style::default().fg(color),
            ));
        }
    }

    ListItem::new(Line::from(spans))
}

fn get_claude_state_indicator(state: ClaudeState) -> (&'static str, Color) {
    use std::time::{SystemTime, UNIX_EPOCH};

    const BLINK_DURATION_MS: u128 = 500;
    const BLINK_PHASES: u128 = 2;

    let should_flash = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => (duration.as_millis() / BLINK_DURATION_MS) % BLINK_PHASES == 0,
        Err(_) => true,
    };

    match state {
        ClaudeState::Idle => ("", Color::Gray),
        ClaudeState::Running if should_flash => ("●", Color::Rgb(255, 165, 0)),
        ClaudeState::Running => ("", Color::Rgb(255, 165, 0)),
        ClaudeState::NeedsPermissions => ("●", Color::Rgb(138, 43, 226)),
        ClaudeState::Done => ("●", Color::Green),
    }
}
