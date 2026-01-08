use crate::views::instances::{ClaudeState, InstancePane};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, pane: Option<&InstancePane>, is_focused: bool, area: Rect) {

    let border_color = if is_focused { Color::Green } else { Color::Cyan };

    let title = if is_focused {
        "● Terminal (Esc to exit)"
    } else {
        "Terminal"
    };

    let mut block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(if is_focused {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Cyan)
        });

    if let Some(pane) = pane {
        let (indicator, color) = get_claude_state_indicator(pane.claude_state);
        if !indicator.is_empty() {
            block = block.title_bottom(
                Line::from(vec![Span::styled(
                    format!(" {} ", indicator),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                )])
                .right_aligned(),
            );
        }
    }

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    match pane {
        Some(pane) => render_terminal_content(frame, pane, inner_area),
        None => render_no_terminal(frame, inner_area),
    }
}

fn render_terminal_content(frame: &mut Frame, pane: &InstancePane, area: Rect) {
    if let Some(session) = &pane.pty_session {
        if let Ok(parser) = session.screen().lock() {
            let screen = parser.screen();
            let mut lines = Vec::new();

            let max_rows = area.height.min(screen.size().0);
            let max_cols = area.width.min(screen.size().1);

            for row in 0..max_rows {
                let mut line_spans = Vec::new();
                let mut current_text = String::new();
                let mut current_style = Style::default();
                let mut last_foreground = vt100::Color::Default;
                let mut last_background = vt100::Color::Default;
                let mut last_attrs = (false, false, false);

                for col in 0..max_cols {
                    let cell = match screen.cell(row, col) {
                        Some(cell) => cell,
                        None => continue,
                    };

                    let fg = cell.fgcolor();
                    let bg = cell.bgcolor();
                    let attrs = (cell.bold(), cell.italic(), cell.underline());

                    if fg != last_foreground || bg != last_background || attrs != last_attrs {
                        if !current_text.is_empty() {
                            line_spans.push(Span::styled(current_text.clone(), current_style));
                            current_text.clear();
                        }

                        current_style = Style::default()
                            .fg(convert_vt100_color(fg))
                            .bg(convert_vt100_color(bg));

                        if attrs.0 {
                            current_style = current_style.add_modifier(Modifier::BOLD);
                        }
                        if attrs.1 {
                            current_style = current_style.add_modifier(Modifier::ITALIC);
                        }
                        if attrs.2 {
                            current_style = current_style.add_modifier(Modifier::UNDERLINED);
                        }

                        last_foreground = fg;
                        last_background = bg;
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

                lines.push(Line::from(line_spans));
            }

            let text = Paragraph::new(lines);
            frame.render_widget(text, area);
        }
    } else {
        let message = Paragraph::new("PTY session not available")
            .style(Style::default().fg(Color::Red));
        frame.render_widget(message, area);
    }
}

fn render_no_terminal(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "No terminal for this task",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Move task to In Progress to start a terminal",
            Style::default().fg(Color::DarkGray),
        )]),
    ]);

    frame.render_widget(text, area);
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
        ClaudeState::Idle => ("Idle", Color::Gray),
        ClaudeState::Running if should_flash => ("Running ●", Color::Rgb(255, 165, 0)),
        ClaudeState::Running => ("Running", Color::Rgb(255, 165, 0)),
        ClaudeState::NeedsPermissions => ("Needs Permission ●", Color::Rgb(138, 43, 226)),
        ClaudeState::Done => ("Done ●", Color::Green),
    }
}

fn convert_vt100_color(color: vt100::Color) -> Color {
    match color {
        vt100::Color::Default => Color::Reset,
        vt100::Color::Idx(idx) => match idx {
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::Gray,
            8 => Color::DarkGray,
            9 => Color::LightRed,
            10 => Color::LightGreen,
            11 => Color::LightYellow,
            12 => Color::LightBlue,
            13 => Color::LightMagenta,
            14 => Color::LightCyan,
            15 => Color::White,
            _ => Color::Reset,
        },
        vt100::Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
    }
}
