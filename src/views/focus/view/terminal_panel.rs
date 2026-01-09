use crate::views::instances::InstancePane;
use crate::widgets::terminal::{TerminalView, claude_state};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, pane: Option<&InstancePane>, is_focused: bool, area: Rect) {
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

    if let Some(pane) = pane {
        let (indicator, color) = claude_state::get_indicator(pane.claude_state);
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
        Some(pane) => render_pane_content(frame, pane, inner_area),
        None => render_no_terminal(frame, inner_area),
    }
}

fn render_pane_content(frame: &mut Frame, pane: &InstancePane, area: Rect) {
    if let Some(session) = &pane.pty_session {
        frame.render_widget(TerminalView::new(session), area);
    } else {
        let message =
            Paragraph::new("PTY session not available").style(Style::default().fg(Color::Red));
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
