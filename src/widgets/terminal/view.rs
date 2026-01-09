use super::colors::convert_vt100_color;
use crate::views::instances::pty::PtySession;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

pub struct TerminalView<'a> {
    session: &'a PtySession,
}

impl<'a> TerminalView<'a> {
    pub const fn new(session: &'a PtySession) -> Self {
        Self { session }
    }
}

impl Widget for TerminalView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let screen_mutex = self.session.screen();
        let Ok(parser) = screen_mutex.lock() else {
            return;
        };

        let screen = parser.screen();
        let lines = build_terminal_lines(screen, area);
        let text = Paragraph::new(lines);
        text.render(area, buf);
    }
}

fn build_terminal_lines(screen: &vt100::Screen, area: Rect) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    let max_rows = area.height.min(screen.size().0);
    let max_cols = area.width.min(screen.size().1);

    for row in 0..max_rows {
        let line = build_line(screen, row, max_cols);
        lines.push(line);
    }

    lines
}

fn build_line(screen: &vt100::Screen, row: u16, max_cols: u16) -> Line<'static> {
    let mut line_spans = Vec::new();
    let mut current_text = String::new();
    let mut current_style = Style::default();
    let mut last_foreground = vt100::Color::Default;
    let mut last_background = vt100::Color::Default;
    let mut last_attrs = (false, false, false);

    for col in 0..max_cols {
        let Some(cell) = screen.cell(row, col) else {
            continue;
        };

        let foreground = cell.fgcolor();
        let background = cell.bgcolor();
        let attrs = (cell.bold(), cell.italic(), cell.underline());

        let style_changed =
            foreground != last_foreground || background != last_background || attrs != last_attrs;

        if style_changed {
            if !current_text.is_empty() {
                line_spans.push(Span::styled(current_text.clone(), current_style));
                current_text.clear();
            }

            current_style = build_style(foreground, background, attrs);
            last_foreground = foreground;
            last_background = background;
            last_attrs = attrs;
        }

        current_text.push_str(&cell.contents());
    }

    if !current_text.is_empty() {
        line_spans.push(Span::styled(current_text, current_style));
    }

    if line_spans.is_empty() {
        line_spans.push(Span::raw(""));
    }

    Line::from(line_spans)
}

fn build_style(
    foreground: vt100::Color,
    background: vt100::Color,
    attrs: (bool, bool, bool),
) -> Style {
    let mut style = Style::default()
        .fg(convert_vt100_color(foreground))
        .bg(convert_vt100_color(background));

    if attrs.0 {
        style = style.add_modifier(Modifier::BOLD);
    }
    if attrs.1 {
        style = style.add_modifier(Modifier::ITALIC);
    }
    if attrs.2 {
        style = style.add_modifier(Modifier::UNDERLINED);
    }

    style
}
