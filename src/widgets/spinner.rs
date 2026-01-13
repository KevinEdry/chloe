use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

const SPINNER_FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

pub struct Spinner<'a> {
    frame: usize,
    message: Option<&'a str>,
    style: Style,
}

impl<'a> Spinner<'a> {
    #[must_use]
    pub const fn new(frame: usize) -> Self {
        Self {
            frame,
            message: None,
            style: Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        }
    }

    #[must_use]
    pub const fn message(mut self, message: &'a str) -> Self {
        self.message = Some(message);
        self
    }

    #[must_use]
    const fn get_char(&self) -> char {
        get_spinner_char(self.frame)
    }

    #[must_use]
    pub fn to_span(&self) -> Span<'static> {
        let spinner_char = self.get_char();
        self.message.map_or_else(
            || Span::styled(spinner_char.to_string(), self.style),
            |msg| Span::styled(format!("{spinner_char} {msg}"), self.style),
        )
    }

    #[must_use]
    pub fn to_line(&self) -> Line<'static> {
        Line::from(vec![self.to_span()])
    }
}

impl Widget for Spinner<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let line = self.to_line();
        let paragraph = Paragraph::new(line);
        paragraph.render(area, buffer);
    }
}

#[must_use]
pub const fn get_spinner_char(frame: usize) -> char {
    SPINNER_FRAMES[frame % SPINNER_FRAMES.len()]
}

#[must_use]
pub fn spinner_span(frame: usize, message: &str) -> Span<'static> {
    Spinner::new(frame).message(message).to_span()
}
