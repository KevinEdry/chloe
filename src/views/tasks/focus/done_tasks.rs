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
    let is_focused = state.focus_panel == FocusPanel::DoneTasks;

    let done_column = columns.get(3);
    let done_count = done_column.map(|column| column.tasks.len()).unwrap_or(0);

    let border_color = if is_focused {
        Color::Green
    } else {
        Color::DarkGray
    };
    let title_color = if is_focused {
        Color::Green
    } else {
        Color::DarkGray
    };

    let mut block = Block::default()
        .title("Done")
        .title_style(
            Style::default()
                .fg(title_color)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    if done_count > 0 {
        let position_text = format!(" {} of {} ", state.focus_done_index + 1, done_count);
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

    if done_count == 0 {
        render_empty_state(frame, inner_area);
        return;
    }

    let items = build_done_task_items(app, is_focused);
    let list = List::new(items);
    frame.render_widget(list, inner_area);
}

fn render_empty_state(frame: &mut Frame, area: Rect) {
    let empty_message = List::new(vec![
        ListItem::new(Line::from("")),
        ListItem::new(Line::from(vec![Span::styled(
            "No completed tasks",
            Style::default().fg(Color::DarkGray),
        )])),
    ]);
    frame.render_widget(empty_message, area);
}

fn build_done_task_items(app: &App, is_panel_focused: bool) -> Vec<ListItem<'static>> {
    let mut items = Vec::new();
    let columns = &app.tasks.columns;
    let selected_index = app.tasks.focus_done_index;

    let Some(done_column) = columns.get(3) else {
        return items;
    };

    for (index, task) in done_column.tasks.iter().enumerate() {
        let is_selected = is_panel_focused && index == selected_index;
        items.push(create_task_item(&task.title, task.task_type, is_selected));
    }

    items
}

fn create_task_item(
    title: &str,
    task_type: crate::views::tasks::TaskType,
    is_selected: bool,
) -> ListItem<'static> {
    let mut item = TaskItem::new(title, task_type)
        .selected(is_selected)
        .selection_color(Color::Green);

    if !is_selected {
        item = item.badge_color(Color::DarkGray);
    }

    item.build()
}
