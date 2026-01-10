use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Widget, Wrap},
};

const DIALOG_WIDTH_THRESHOLD: u16 = 80;
const DIALOG_WIDTH_SMALL: u16 = 80;
const DIALOG_WIDTH_NORMAL: u16 = 60;
const DIALOG_MIN_HEIGHT: u16 = 7;
const DIALOG_MAX_HEIGHT: u16 = 15;

pub struct ErrorDialog<'a> {
    title: &'a str,
    message: &'a str,
}

impl<'a> ErrorDialog<'a> {
    #[must_use]
    pub const fn new(title: &'a str, message: &'a str) -> Self {
        Self { title, message }
    }
}

impl Widget for ErrorDialog<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
            DIALOG_WIDTH_SMALL
        } else {
            DIALOG_WIDTH_NORMAL
        };

        #[allow(clippy::cast_possible_truncation)]
        let message_lines = self.message.lines().count().min(usize::from(u16::MAX)) as u16;
        let dialog_height = (message_lines + 6).clamp(DIALOG_MIN_HEIGHT, DIALOG_MAX_HEIGHT);

        let popup_area = centered_rect(dialog_width, dialog_height, area);
        let border_color = Color::Red;

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

        let help_line = Line::from(Span::styled(
            "Press any key to dismiss",
            Style::default().fg(Color::DarkGray),
        ));

        let text = Paragraph::new(vec![
            Line::from(Span::styled(
                self.message,
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            help_line,
        ])
        .wrap(Wrap { trim: true })
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
