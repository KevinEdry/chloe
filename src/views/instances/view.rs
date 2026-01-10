use super::layout;
use super::state::{InstancePane, InstanceState};
use crate::views::StatusBarContent;
use crate::widgets::claude_indicator;
use crate::widgets::terminal::{AlacrittyScreen, Cursor, PseudoTerminal};
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

    if state.root.is_none() {
        render_empty_state(f, area);
        return;
    }

    render_panes(f, state, area);
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
    let Some(root) = &state.root else {
        return;
    };

    let pane_areas = layout::calculate_pane_areas(area, root);
    state.pane_areas.clone_from(&pane_areas);

    resize_panes_to_match_areas(state, &pane_areas);

    let selected_id = state.selected_pane_id;
    let mode = state.mode;

    for (index, (pane_id, pane_area)) in pane_areas.iter().enumerate() {
        let Some(pane) = state.find_pane(*pane_id) else {
            continue;
        };

        let is_selected = selected_id == Some(*pane_id);

        render_pane(f, pane, *pane_area, is_selected, mode, index);
    }
}

fn resize_panes_to_match_areas(state: &mut InstanceState, pane_areas: &[(uuid::Uuid, Rect)]) {
    for (pane_id, pane_area) in pane_areas {
        let inner_area = Block::default().borders(Borders::ALL).inner(*pane_area);
        let desired_rows = inner_area.height;
        let desired_columns = inner_area.width;

        let Some(pane) = state.find_pane_mut(*pane_id) else {
            continue;
        };

        if pane.rows != desired_rows || pane.columns != desired_columns {
            if let Some(session) = &mut pane.pty_session {
                session.resize(desired_rows, desired_columns);
            }
            pane.rows = desired_rows;
            pane.columns = desired_columns;
        }
    }
}

fn render_pane(
    f: &mut Frame,
    pane: &InstancePane,
    area: Rect,
    is_selected: bool,
    mode: super::InstanceMode,
    index: usize,
) {
    let is_focused = is_selected && mode == super::InstanceMode::Focused;
    let is_scroll = is_selected && mode == super::InstanceMode::Scroll;

    let border_color = if is_scroll {
        Color::Yellow
    } else if is_focused {
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

    let title_prefix = if is_scroll {
        "⏸ "
    } else if is_focused {
        "● "
    } else if is_selected {
        "→ "
    } else {
        "  "
    };

    let mut title_spans = vec![
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

    if is_scroll {
        let max_scrollback = pane.scrollback_len();
        let scroll_info = if max_scrollback > 0 {
            format!(" [↑{}/{}]", pane.scroll_offset, max_scrollback)
        } else {
            " [no history]".to_string()
        };
        title_spans.push(Span::styled(
            scroll_info,
            Style::default().fg(Color::Yellow),
        ));
    }

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

    let term_mutex = session.term();
    let Ok(term) = term_mutex.lock() else {
        return;
    };

    let screen = AlacrittyScreen::new(&*term);
    let cursor = Cursor::default().visibility(is_focused);
    let terminal = PseudoTerminal::new(&screen)
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

    let pane_count = state.pane_count();

    let help_text = match state.mode {
        super::InstanceMode::Normal => {
            if pane_count == 0 {
                "c:create"
            } else if width < STATUS_BAR_WIDTH_THRESHOLD {
                "h/j/k/l:nav  Enter:focus  c:create  x:close"
            } else {
                "h/j/k/l:navigate  Enter:focus  c:create-pane  x:close-pane"
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
        extra_info: Some(format!("Panes: {pane_count}  ")),
        help_text: help_text.to_string(),
    }
}
