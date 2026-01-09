use crate::app::App;
use crate::views::tasks::state::FocusPanel;
use crate::widgets::task::TaskItem;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let state = &app.tasks;
    let columns = &state.columns;
    let is_focused = state.focus_panel == FocusPanel::ActiveTasks;

    let active_task_count: usize = columns
        .iter()
        .take(3)
        .map(|column| column.tasks.len())
        .sum();

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
            state.focus_active_index + 1,
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

    let title_max_length = calculate_title_max_length(inner_area.width);
    let items = build_task_list_items(app, is_focused, title_max_length);
    let list = List::new(items);
    frame.render_widget(list, inner_area);
}

const fn calculate_title_max_length(panel_width: u16) -> usize {
    const SELECTION_INDICATOR_WIDTH: usize = 2;
    const BADGE_MAX_WIDTH: usize = 7;
    const SPACE_AFTER_BADGE: usize = 1;
    const CLAUDE_INDICATOR_WIDTH: usize = 2;
    const SAFETY_MARGIN: usize = 1;

    let fixed_width = SELECTION_INDICATOR_WIDTH
        + BADGE_MAX_WIDTH
        + SPACE_AFTER_BADGE
        + CLAUDE_INDICATOR_WIDTH
        + SAFETY_MARGIN;

    (panel_width as usize).saturating_sub(fixed_width)
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
            "Press 'a' to add a task",
            Style::default().fg(Color::DarkGray),
        )])),
    ]);
    frame.render_widget(empty_message, area);
}

fn build_task_list_items(
    app: &App,
    is_panel_focused: bool,
    title_max_length: usize,
) -> Vec<ListItem<'static>> {
    let mut items = Vec::new();
    let mut current_index = 0;
    let columns = &app.tasks.columns;
    let selected_index = app.tasks.focus_active_index;

    for (column_index, column) in columns.iter().take(3).enumerate() {
        if !column.tasks.is_empty() {
            items.push(create_column_header(&column.name, column_index));

            for task in &column.tasks {
                let is_selected = is_panel_focused && current_index == selected_index;
                let instance_id = task.instance_id;
                let claude_state = instance_id.and_then(|id| app.get_instance_claude_state(id));

                items.push(create_task_item(
                    &task.title,
                    task.kind,
                    is_selected,
                    claude_state,
                    title_max_length,
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
    task_type: crate::views::tasks::TaskType,
    is_selected: bool,
    claude_state: Option<crate::views::instances::ClaudeState>,
    title_max_length: usize,
) -> ListItem<'static> {
    TaskItem::new(title, task_type)
        .selected(is_selected)
        .claude_state(claude_state)
        .title_max_length(title_max_length)
        .build()
}
