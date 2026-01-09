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
const DIALOG_HEIGHT: u16 = 9;

const SPINNER_FRAME_DURATION_MS: u128 = 100;
const SPINNER_FRAME_COUNT: u128 = 10;

pub struct LoadingDialog<'a> {
    title: &'a str,
    input: &'a str,
}

impl<'a> LoadingDialog<'a> {
    #[must_use]
    pub const fn new(title: &'a str, input: &'a str) -> Self {
        Self { title, input }
    }
}

impl Widget for LoadingDialog<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
            DIALOG_WIDTH_SMALL
        } else {
            DIALOG_WIDTH_NORMAL
        };

        let popup_area = centered_rect(dialog_width, DIALOG_HEIGHT, area);
        let border_color = Color::Magenta;

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

        let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame_index = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            / SPINNER_FRAME_DURATION_MS)
            % SPINNER_FRAME_COUNT;
        let spinner = spinner_frames[frame_index as usize];

        let lines = vec![
            Line::from(vec![
                Span::styled(
                    spinner,
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" Processing..."),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Input: ", Style::default().fg(Color::Gray)),
                Span::styled(self.input, Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Press Esc to cancel",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )),
        ];

        let content = Paragraph::new(lines)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        content.render(inner_area, buf);
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
