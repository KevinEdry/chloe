use super::style::DialogStyle;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Widget},
};

const DIALOG_WIDTH_THRESHOLD: u16 = 80;
const DIALOG_WIDTH_SMALL: u16 = 80;
const DIALOG_WIDTH_NORMAL: u16 = 60;
const DIALOG_HEIGHT: u16 = 7;

pub struct ConfirmDialog<'a> {
    title: &'a str,
    message: &'a str,
    style: DialogStyle,
}

impl<'a> ConfirmDialog<'a> {
    #[must_use]
    pub fn new(title: &'a str, message: &'a str) -> Self {
        Self {
            title,
            message,
            style: DialogStyle::default(),
        }
    }

    #[must_use]
    pub const fn style(mut self, style: DialogStyle) -> Self {
        self.style = style;
        self
    }
}

impl Widget for ConfirmDialog<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
            DIALOG_WIDTH_SMALL
        } else {
            DIALOG_WIDTH_NORMAL
        };

        let popup_area = centered_rect(dialog_width, DIALOG_HEIGHT, area);
        let border_color = self.style.color();

        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(self.title)
            .title_style(
                Style::default()
                    .fg(border_color)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .padding(Padding::uniform(1));

        let inner_area = block.inner(popup_area);
        block.render(popup_area, buf);

        let text = Paragraph::new(vec![Line::from(""), Line::from(Span::raw(self.message))])
            .alignment(Alignment::Center);

        text.render(inner_area, buf);
    }
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
