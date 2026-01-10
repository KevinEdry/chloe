use super::InstanceState;
use crate::views::StatusBarContent;
use crate::widgets::claude_indicator;
use crate::widgets::terminal::{Cursor, PseudoTerminal};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const STATUS_BAR_WIDTH_THRESHOLD: u16 = 80;

pub fn render(f: &mut Frame, state: &mut InstanceState, area: Rect) {
    state.last_render_area = Some(area);

    if state.panes.is_empty() {
        render_empty_state(f, area);
    } else {
        render_panes(f, state, area);
    }
}

fn render_empty_state(f: &mut Frame, area: Rect) {
    let block = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "No instance panes open",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press 'c' to create a new instance pane",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Instances")
            .border_style(Style::default().fg(Color::DarkGray)),
    )
    .style(Style::default())
    .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(block, area);
}

fn render_panes(f: &mut Frame, state: &mut InstanceState, area: Rect) {
    let pane_areas =
        super::layout::calculate_pane_areas(area, state.layout_mode, state.panes.len());

    for (pane, pane_area) in state.panes.iter_mut().zip(pane_areas.iter()) {
        let inner_area = Block::default().borders(Borders::ALL).inner(*pane_area);

        let desired_rows = inner_area.height;
        let desired_columns = inner_area.width;

        if pane.rows != desired_rows || pane.columns != desired_columns {
            if let Some(session) = &pane.pty_session {
                let _ = session.resize(desired_rows, desired_columns);
            }
            pane.rows = desired_rows;
            pane.columns = desired_columns;
        }
    }

    for (index, (pane, pane_area)) in state.panes.iter().zip(pane_areas.iter()).enumerate() {
        let is_selected = index == state.selected_pane;
        let is_focused = is_selected && state.mode == super::InstanceMode::Focused;
        render_pane(f, pane, *pane_area, is_selected, is_focused, index);
    }
}

fn render_pane(
    f: &mut Frame,
    pane: &super::InstancePane,
    area: Rect,
    is_selected: bool,
    is_focused: bool,
    index: usize,
) {
    let border_color = if is_focused {
        Color::Green
    } else if is_selected {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let border_style = Style::default()
        .fg(border_color)
        .add_modifier(if is_selected {
            Modifier::BOLD
        } else {
            Modifier::empty()
        });

    let (state_indicator, indicator_color) = claude_indicator::dot(pane.claude_state);

    let pane_name = pane
        .name
        .clone()
        .unwrap_or_else(|| format!("Pane {}", index + 1));

    let title_prefix = if is_focused {
        "● "
    } else if is_selected {
        "→ "
    } else {
        "  "
    };

    let title_spans = vec![
        Span::styled(
            format!("{title_prefix}{pane_name} "),
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            state_indicator,
            Style::default()
                .fg(indicator_color)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(Line::from(title_spans));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let Some(session) = &pane.pty_session else {
        let message = "PTY session failed to start";
        let text = Paragraph::new(message).style(Style::default().fg(Color::Red));
        f.render_widget(text, inner_area);
        return;
    };

    let screen_mutex = session.screen();
    let Ok(mut parser) = screen_mutex.lock() else {
        return;
    };

    parser.set_scrollback(pane.scroll_offset);

    let cursor = Cursor::default().visibility(is_focused);
    let terminal = PseudoTerminal::new(parser.screen())
        .cursor(cursor)
        .scroll_offset(pane.scroll_offset);

    f.render_widget(terminal, inner_area);
}

#[must_use]
pub fn get_status_bar_content(state: &InstanceState, width: u16) -> StatusBarContent {
    let (mode_text, mode_color) = match state.mode {
        super::InstanceMode::Normal => ("NAVIGATE", Color::Cyan),
        super::InstanceMode::Focused => ("FOCUSED", Color::Green),
        super::InstanceMode::Scroll => ("SCROLL", Color::Yellow),
    };

    let pane_count = state.panes.len();
    let layout_name = match state.layout_mode {
        super::LayoutMode::Single => "Single",
        super::LayoutMode::HorizontalSplit => "Horizontal",
        super::LayoutMode::VerticalSplit => "Vertical",
        super::LayoutMode::Grid => "Grid",
    };

    let help_text = match state.mode {
        super::InstanceMode::Normal => {
            if pane_count == 0 {
                "c:create"
            } else if width < STATUS_BAR_WIDTH_THRESHOLD {
                "Arrows:nav  Enter:focus  c:create  x:close"
            } else {
                "Arrow-Keys:navigate  Enter:focus  c:create-pane  x:close-pane"
            }
        }
        super::InstanceMode::Focused => {
            if width < STATUS_BAR_WIDTH_THRESHOLD {
                "Ctrl+s:scroll  Esc:unfocus"
            } else {
                "Ctrl+s:scroll-mode  Esc:back-to-navigation"
            }
        }
        super::InstanceMode::Scroll => {
            if width < STATUS_BAR_WIDTH_THRESHOLD {
                "j/k:line  Ctrl+d/u:page  g/G:top/bottom  q:exit"
            } else {
                "j/k:scroll-line  Ctrl+d/u:half-page  g/G:top/bottom  q/Esc:exit-scroll"
            }
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: Some(format!("Panes: {pane_count}  Layout: {layout_name}  ")),
        help_text: help_text.to_string(),
    }
}
