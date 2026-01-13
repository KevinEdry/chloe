use super::state::{PullRequest, PullRequestStatusState, PullRequestsMode, PullRequestsState};
use crate::views::StatusBarContent;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

const STATUS_BAR_WIDTH_THRESHOLD: u16 = 100;

pub fn render(frame: &mut Frame, area: Rect, state: &PullRequestsState) {
    render_pull_request_list(frame, area, state);
}

fn render_pull_request_list(frame: &mut Frame, area: Rect, state: &PullRequestsState) {
    if state.is_loading {
        let loading_text = Paragraph::new("Loading pull requests...")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Pull Requests")
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        frame.render_widget(loading_text, area);
        return;
    }

    if let Some(error) = &state.error_message {
        let error_text = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Pull Requests - Error")
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        frame.render_widget(error_text, area);
        return;
    }

    if state.pull_requests.is_empty() {
        let empty_text = Paragraph::new("No pull requests found. Press 'r' to refresh.")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Pull Requests")
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        frame.render_widget(empty_text, area);
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_list(frame, layout[0], state);
    render_details(frame, layout[1], state);
}

fn render_list(frame: &mut Frame, area: Rect, state: &PullRequestsState) {
    let available_width = area.width.saturating_sub(4) as usize;

    let items: Vec<ListItem> = state
        .pull_requests
        .iter()
        .enumerate()
        .map(|(index, pull_request)| create_list_item(index, pull_request, state, available_width))
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Pull Requests ({})", state.pull_requests.len()))
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(list, area);
}

fn create_list_item(
    index: usize,
    pull_request: &PullRequest,
    state: &PullRequestsState,
    available_width: usize,
) -> ListItem<'static> {
    let is_selected = state.selected_index == Some(index);

    let state_indicator = match pull_request.state {
        PullRequestStatusState::Open => {
            if pull_request.is_draft {
                Span::styled(" [Draft] ", Style::default().fg(Color::Gray))
            } else {
                Span::styled(" [Open] ", Style::default().fg(Color::Green))
            }
        }
        PullRequestStatusState::Closed => {
            Span::styled(" [Closed] ", Style::default().fg(Color::Red))
        }
        PullRequestStatusState::Merged => {
            Span::styled(" [Merged] ", Style::default().fg(Color::Magenta))
        }
    };

    let number_span = Span::styled(
        format!("#{}", pull_request.number),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let number_width = format!("#{}", pull_request.number).len();
    let state_indicator_width = 9;
    let spacing_width = 2;
    let title_max_width = available_width
        .saturating_sub(number_width)
        .saturating_sub(state_indicator_width)
        .saturating_sub(spacing_width);

    let title_span = Span::styled(
        truncate_string(&pull_request.title, title_max_width),
        Style::default().fg(Color::White),
    );

    let content = Line::from(vec![
        Span::raw("  "),
        number_span,
        state_indicator,
        title_span,
    ]);

    let mut item = ListItem::new(content);

    if is_selected {
        item = item.style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );
    }

    item
}

fn render_details(frame: &mut Frame, area: Rect, state: &PullRequestsState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Details")
        .border_style(Style::default().fg(Color::DarkGray));

    let Some(pull_request) = state.get_selected_pull_request() else {
        let no_selection = Paragraph::new("Select a pull request to view details")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(block);
        frame.render_widget(no_selection, area);
        return;
    };

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let details_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .margin(1)
        .split(inner_area);

    let title_line = Line::from(vec![
        Span::styled("Title: ", Style::default().fg(Color::Gray)),
        Span::styled(&pull_request.title, Style::default().fg(Color::White)),
    ]);

    let number_line = Line::from(vec![
        Span::styled("Number: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("#{}", pull_request.number),
            Style::default().fg(Color::Cyan),
        ),
    ]);

    let author_line = Line::from(vec![
        Span::styled("Author: ", Style::default().fg(Color::Gray)),
        Span::styled(&pull_request.author, Style::default().fg(Color::Yellow)),
    ]);

    let branch_line = Line::from(vec![
        Span::styled("Branch: ", Style::default().fg(Color::Gray)),
        Span::styled(&pull_request.branch, Style::default().fg(Color::Green)),
        Span::styled(" -> ", Style::default().fg(Color::DarkGray)),
        Span::styled(&pull_request.base_branch, Style::default().fg(Color::Blue)),
    ]);

    let state_color = match pull_request.state {
        PullRequestStatusState::Open => {
            if pull_request.is_draft {
                Color::Gray
            } else {
                Color::Green
            }
        }
        PullRequestStatusState::Closed => Color::Red,
        PullRequestStatusState::Merged => Color::Magenta,
    };

    let state_text = match pull_request.state {
        PullRequestStatusState::Open => {
            if pull_request.is_draft {
                "Draft"
            } else {
                "Open"
            }
        }
        PullRequestStatusState::Closed => "Closed",
        PullRequestStatusState::Merged => "Merged",
    };

    let state_line = Line::from(vec![
        Span::styled("Status: ", Style::default().fg(Color::Gray)),
        Span::styled(state_text, Style::default().fg(state_color)),
    ]);

    let changes_line = Line::from(vec![
        Span::styled("Changes: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("+{}", pull_request.additions),
            Style::default().fg(Color::Green),
        ),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("-{}", pull_request.deletions),
            Style::default().fg(Color::Red),
        ),
    ]);

    let url_line = Line::from(vec![
        Span::styled("URL: ", Style::default().fg(Color::Gray)),
        Span::styled(&pull_request.url, Style::default().fg(Color::Blue)),
    ]);

    frame.render_widget(Paragraph::new(title_line), details_layout[0]);
    frame.render_widget(Paragraph::new(number_line), details_layout[1]);
    frame.render_widget(Paragraph::new(author_line), details_layout[2]);
    frame.render_widget(Paragraph::new(branch_line), details_layout[3]);
    frame.render_widget(Paragraph::new(state_line), details_layout[4]);
    frame.render_widget(Paragraph::new(changes_line), details_layout[5]);
    frame.render_widget(Paragraph::new(url_line), details_layout[7]);
}

fn truncate_string(string: &str, max_length: usize) -> String {
    if string.len() <= max_length {
        string.to_string()
    } else {
        format!("{}...", &string[..max_length.saturating_sub(3)])
    }
}

#[must_use]
pub fn get_status_bar_content(state: &PullRequestsState, width: u16) -> StatusBarContent {
    let mode_color = match state.mode {
        PullRequestsMode::Normal => Color::Cyan,
        PullRequestsMode::Viewing => Color::Yellow,
    };

    let mode_text = match state.mode {
        PullRequestsMode::Normal => "NORMAL",
        PullRequestsMode::Viewing => "VIEWING",
    };

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match state.mode {
            PullRequestsMode::Normal => "jk:navigate  o:open  r:refresh",
            PullRequestsMode::Viewing => "Esc:back",
        }
    } else {
        match state.mode {
            PullRequestsMode::Normal => {
                "jk/arrows:navigate  o/Enter:open in browser  r:refresh  Tab:switch-tabs  q:quit"
            }
            PullRequestsMode::Viewing => "Esc/q:back to list",
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: None,
        help_text: help_text.to_string(),
    }
}
