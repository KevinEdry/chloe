use crate::views::instances::state::ActivitySummary;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Widget, Wrap},
};

const DIALOG_WIDTH_THRESHOLD: u16 = 100;
const DIALOG_WIDTH_SMALL: u16 = 70;
const DIALOG_WIDTH_NORMAL: u16 = 90;
const DIALOG_HEIGHT_PERCENT: u16 = 80;
const HEADER_HEIGHT: u16 = 3;

pub struct ActivitySummaryWidget<'a> {
    summary: &'a ActivitySummary,
    scroll_offset: usize,
}

impl<'a> ActivitySummaryWidget<'a> {
    #[must_use]
    pub const fn new(summary: &'a ActivitySummary) -> Self {
        Self {
            summary,
            scroll_offset: 0,
        }
    }

    #[must_use]
    pub const fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }
}

impl Widget for ActivitySummaryWidget<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
            DIALOG_WIDTH_SMALL
        } else {
            DIALOG_WIDTH_NORMAL
        }
        .min(area.width.saturating_sub(2));

        let dialog_height =
            (area.height * DIALOG_HEIGHT_PERCENT / 100).min(area.height.saturating_sub(2));
        let popup_area = centered_rect(dialog_width, dialog_height, area);

        Clear.render(popup_area, buffer);

        let block = Block::default()
            .title(" Activity Summary ")
            .title_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::uniform(1));

        let inner_area = block.inner(popup_area);
        block.render(popup_area, buffer);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(HEADER_HEIGHT), Constraint::Min(0)])
            .split(inner_area);

        render_header(self.summary, chunks[0], buffer);
        render_events(self.summary, chunks[1], buffer, self.scroll_offset);
    }
}

fn render_header(summary: &ActivitySummary, area: Rect, buffer: &mut Buffer) {
    let elapsed_minutes = summary.elapsed_seconds / 60;
    let elapsed_seconds = summary.elapsed_seconds % 60;
    let time_display = if elapsed_minutes > 0 {
        format!("{elapsed_minutes}m {elapsed_seconds}s ago")
    } else {
        format!("{elapsed_seconds}s ago")
    };

    let header_lines = vec![
        Line::from(vec![
            Span::styled("Since: ", Style::default().fg(Color::Gray)),
            Span::styled(
                summary.since.format("%H:%M:%S").to_string(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" ({time_display})"),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("Activity: ", Style::default().fg(Color::Gray)),
            Span::styled(
                summary.format_as_summary_line(),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
    ];

    let header_paragraph = Paragraph::new(header_lines);
    header_paragraph.render(area, buffer);
}

fn render_events(summary: &ActivitySummary, area: Rect, buffer: &mut Buffer, scroll_offset: usize) {
    let mut items = Vec::new();

    add_commands_section(&mut items, summary);
    add_files_section(&mut items, summary);
    add_tasks_section(&mut items, summary);
    add_errors_section(&mut items, summary);
    add_notifications_section(&mut items, summary);

    if items.is_empty() {
        render_empty_activity(area, buffer);
        return;
    }

    render_activity_list(items, area, buffer, scroll_offset);
}

fn add_commands_section<'a>(items: &mut Vec<ListItem<'a>>, summary: &'a ActivitySummary) {
    if summary.commands_executed.is_empty() {
        return;
    }

    items.push(create_section_header(
        "Commands Executed",
        summary.commands_executed.len(),
        Color::Green,
    ));

    for command in &summary.commands_executed {
        items.push(create_list_item(command, Color::White));
    }
    items.push(ListItem::new(Line::from("")));
}

fn add_files_section<'a>(items: &mut Vec<ListItem<'a>>, summary: &'a ActivitySummary) {
    if summary.files_changed.is_empty() {
        return;
    }

    items.push(create_section_header(
        "Files Changed",
        summary.files_changed.len(),
        Color::Blue,
    ));

    for file in &summary.files_changed {
        items.push(create_list_item(file, Color::White));
    }
    items.push(ListItem::new(Line::from("")));
}

fn add_tasks_section<'a>(items: &mut Vec<ListItem<'a>>, summary: &'a ActivitySummary) {
    if summary.tasks_completed == 0 {
        return;
    }

    items.push(create_section_header(
        "Tasks Completed",
        summary.tasks_completed,
        Color::Magenta,
    ));
    items.push(ListItem::new(Line::from("")));
}

fn add_errors_section<'a>(items: &mut Vec<ListItem<'a>>, summary: &'a ActivitySummary) {
    if summary.errors.is_empty() {
        return;
    }

    items.push(create_section_header(
        "Errors",
        summary.errors.len(),
        Color::Red,
    ));

    for error in &summary.errors {
        items.push(create_list_item(error, Color::Red));
    }
    items.push(ListItem::new(Line::from("")));
}

fn add_notifications_section<'a>(items: &mut Vec<ListItem<'a>>, summary: &'a ActivitySummary) {
    if summary.notifications.is_empty() {
        return;
    }

    items.push(create_section_header(
        "Provider Notifications",
        summary.notifications.len(),
        Color::Yellow,
    ));

    for notification in &summary.notifications {
        items.push(create_list_item(notification, Color::Yellow));
    }
}

fn create_section_header(title: &str, count: usize, color: Color) -> ListItem<'_> {
    ListItem::new(Line::from(vec![
        Span::styled(
            format!("{title} "),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!("({count})"), Style::default().fg(Color::DarkGray)),
    ]))
}

fn create_list_item(text: &str, color: Color) -> ListItem<'_> {
    ListItem::new(Line::from(vec![
        Span::raw("  ● "),
        Span::styled(text, Style::default().fg(color)),
    ]))
}

fn render_empty_activity(area: Rect, buffer: &mut Buffer) {
    let empty_message = Paragraph::new("No activity recorded")
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: false });
    empty_message.render(area, buffer);
}

fn render_activity_list(
    items: Vec<ListItem>,
    area: Rect,
    buffer: &mut Buffer,
    scroll_offset: usize,
) {
    let visible_items: Vec<ListItem> = items
        .into_iter()
        .skip(scroll_offset)
        .take(area.height as usize)
        .collect();

    let list = List::new(visible_items);
    list.render(area, buffer);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical_margin = area.height.saturating_sub(height) / 2;
    let horizontal_margin = area.width.saturating_sub(width) / 2;

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(horizontal_margin),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}
