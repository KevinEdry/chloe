use crate::views::instances::InstancePane;
use crate::widgets::claude_indicator;
use crate::widgets::terminal::{Cursor, PseudoTerminal};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, pane: Option<&mut InstancePane>, is_focused: bool, area: Rect) {
    let border_color = if is_focused {
        Color::Green
    } else {
        Color::Cyan
    };

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

    if let Some(pane) = &pane {
        let (indicator, color) = claude_indicator::label(pane.claude_state);
        block = block.title_bottom(
            Line::from(vec![Span::styled(
                format!(" {} ", indicator),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )])
            .right_aligned(),
        );
    }

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    match pane {
        Some(pane) => render_pane_content(frame, pane, is_focused, inner_area),
        None => render_no_terminal(frame, inner_area),
    }
}

fn render_pane_content(frame: &mut Frame, pane: &mut InstancePane, is_focused: bool, area: Rect) {
    let Some(session) = &mut pane.pty_session else {
        let message =
            Paragraph::new("PTY session not available").style(Style::default().fg(Color::Red));
        frame.render_widget(message, area);
        return;
    };

    let desired_rows = area.height;
    let desired_columns = area.width;

    if pane.rows != desired_rows || pane.columns != desired_columns {
        let _ = session.resize(desired_rows, desired_columns);
        pane.rows = desired_rows;
        pane.columns = desired_columns;
    }

    let screen_mutex = session.screen();
    let Ok(mut parser) = screen_mutex.lock() else {
        return;
    };

    parser.set_scrollback(pane.scroll_offset);

    let cursor = Cursor::default().visibility(is_focused);
    let terminal = PseudoTerminal::new(parser.screen())
        .cursor(cursor)
        .scroll_offset(pane.scroll_offset);

    frame.render_widget(terminal, area);
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
