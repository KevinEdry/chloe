use super::{centered_rect, render_popup_background, review_details, review_status};
use crate::app::App;
use crate::views::instances::InstancePane;
use crate::views::tasks::state::{ReviewAction, ReviewPanel};
use crate::views::worktree::{WorktreeStatus, get_worktree_status};
use crate::widgets::terminal::{AlacrittyScreen, Cursor, PseudoTerminal};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use uuid::Uuid;

const REVIEW_POPUP_WIDTH_PERCENT: u16 = 90;
const REVIEW_POPUP_HEIGHT_PERCENT: u16 = 90;

const BUTTON_COUNT: usize = 5;
const BUTTON_WIDTH_PERCENT: u16 = 20;

const STATUS_HEADER_HEIGHT: u16 = 8;
const BUTTON_ROW_HEIGHT: u16 = 3;

const DIFF_SECTION_PERCENT: u16 = 70;
const OUTPUT_SECTION_PERCENT: u16 = 30;
const FILE_LIST_SECTION_PERCENT: u16 = 30;
const DIFF_CONTENT_SECTION_PERCENT: u16 = 70;

const DIFF_FILES_TITLE: &str = " Files ";
const DIFF_CONTENT_TITLE: &str = " Diff Preview ";
const OUTPUT_SECTION_TITLE: &str = " Instance Output ";
const OUTPUT_MISSING_MESSAGE: &str = "No instance associated with this task";
const OUTPUT_SESSION_MISSING_MESSAGE: &str = "PTY session not available";
const OUTPUT_LOCK_MESSAGE: &str = "Terminal output unavailable";
const LINES_TITLE_PREFIX: &str = " Lines ";
const LINES_TITLE_SEPARATOR: &str = "-";
const LINES_TITLE_SUFFIX: &str = " of ";
const OUTPUT_SCROLL_PREFIX: &str = " Scroll ";
const OUTPUT_SCROLL_SEPARATOR: &str = " / ";
const BORDER_HEIGHT_OFFSET: u16 = 2;

pub struct ReviewInfo {
    pub branch_name: Option<String>,
    pub worktree_status: WorktreeStatus,
    pub task_title: String,
}

pub struct ReviewPopupViewState {
    pub task_id: Uuid,
    pub diff_scroll_offset: usize,
    pub output_scroll_offset: usize,
    pub selected_file_index: usize,
    pub focused_panel: ReviewPanel,
    pub selected_action: ReviewAction,
}

pub fn render_review_popup(
    frame: &mut Frame,
    app: &App,
    popup_state: &ReviewPopupViewState,
    area: Rect,
) {
    let dialog_area = centered_rect(
        REVIEW_POPUP_WIDTH_PERCENT,
        REVIEW_POPUP_HEIGHT_PERCENT,
        area,
    );

    render_popup_background(frame, dialog_area);

    let task_id = popup_state.task_id;

    let review_info = get_review_info(app, task_id);
    let is_clean = review_info.worktree_status.is_clean;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(STATUS_HEADER_HEIGHT),
            Constraint::Min(0),
            Constraint::Length(BUTTON_ROW_HEIGHT),
        ])
        .split(dialog_area);

    render_status_header(frame, &review_info, chunks[0]);
    render_review_sections(frame, app, popup_state, chunks[1]);
    render_action_buttons(frame, popup_state.selected_action, is_clean, chunks[2]);
}

fn get_review_info(app: &App, task_id: Uuid) -> ReviewInfo {
    let task = app.tasks.find_task_by_id(task_id);

    let default_info = ReviewInfo {
        branch_name: None,
        worktree_status: WorktreeStatus::default(),
        task_title: "Unknown Task".to_string(),
    };

    let Some(task) = task else {
        return default_info;
    };

    let task_title = task.title.clone();

    let Some(worktree_info) = &task.worktree_info else {
        return ReviewInfo {
            branch_name: None,
            worktree_status: WorktreeStatus {
                is_clean: true,
                ..WorktreeStatus::default()
            },
            task_title,
        };
    };

    let worktree_status = get_worktree_status(&worktree_info.worktree_path).unwrap_or_default();

    ReviewInfo {
        branch_name: Some(worktree_info.branch_name.clone()),
        worktree_status,
        task_title,
    }
}

fn render_status_header(frame: &mut Frame, info: &ReviewInfo, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            format!(" Review: {} ", info.task_title),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let lines =
        review_status::build_status_lines(info.branch_name.as_deref(), &info.worktree_status);
    let text = Paragraph::new(lines);
    frame.render_widget(text, inner_area);
}

fn render_review_sections(
    frame: &mut Frame,
    app: &App,
    popup_state: &ReviewPopupViewState,
    area: Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(DIFF_SECTION_PERCENT),
            Constraint::Percentage(OUTPUT_SECTION_PERCENT),
        ])
        .split(area);

    render_diff_section(
        frame,
        app,
        popup_state.task_id,
        popup_state.diff_scroll_offset,
        popup_state.selected_file_index,
        popup_state.focused_panel,
        chunks[0],
    );
    render_output_section(
        frame,
        app,
        popup_state.task_id,
        popup_state.output_scroll_offset,
        popup_state.focused_panel == ReviewPanel::Output,
        chunks[1],
    );
}

fn render_diff_section(
    frame: &mut Frame,
    app: &App,
    task_id: Uuid,
    diff_scroll_offset: usize,
    selected_file_index: usize,
    focused_panel: ReviewPanel,
    area: Rect,
) {
    let diff_panel = review_details::build_diff_panel(app, task_id, selected_file_index);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(FILE_LIST_SECTION_PERCENT),
            Constraint::Percentage(DIFF_CONTENT_SECTION_PERCENT),
        ])
        .split(area);

    render_file_list_section(
        frame,
        &diff_panel,
        focused_panel == ReviewPanel::FileList,
        chunks[0],
    );
    render_diff_content_section(
        frame,
        &diff_panel,
        diff_scroll_offset,
        focused_panel == ReviewPanel::DiffContent,
        chunks[1],
    );
}

fn render_file_list_section(
    frame: &mut Frame,
    diff_panel: &review_details::DiffPanelState,
    is_focused: bool,
    area: Rect,
) {
    let block = section_block(DIFF_FILES_TITLE, is_focused);
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let lines = review_details::build_file_list_lines(
        &diff_panel.files,
        diff_panel.selected_index,
        is_focused,
        inner_area.height as usize,
    );

    let text = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    frame.render_widget(text, inner_area);
}

fn render_diff_content_section(
    frame: &mut Frame,
    diff_panel: &review_details::DiffPanelState,
    diff_scroll_offset: usize,
    is_focused: bool,
    area: Rect,
) {
    let total_lines = diff_panel.lines.len().max(1);
    let visible_height = inner_height(area);
    let scroll_offset = diff_scroll_offset.min(total_lines.saturating_sub(1));
    let block = lines_block(
        DIFF_CONTENT_TITLE,
        total_lines,
        scroll_offset,
        visible_height,
        is_focused,
    );

    render_lines_section(frame, diff_panel.lines.clone(), scroll_offset, area, block);
}

fn render_output_section(
    frame: &mut Frame,
    app: &App,
    task_id: Uuid,
    output_scroll_offset: usize,
    is_focused: bool,
    area: Rect,
) {
    let pane = find_output_pane(app, task_id);
    let Some(pane) = pane else {
        render_output_message(frame, OUTPUT_MISSING_MESSAGE, is_focused, area);
        return;
    };

    let max_scrollback = pane.scrollback_len();
    let scroll_offset = output_scroll_offset.min(max_scrollback);
    let block = output_block(scroll_offset, max_scrollback, is_focused);
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    render_output_content(frame, pane, scroll_offset, inner_area);
}

fn render_output_content(frame: &mut Frame, pane: &InstancePane, scroll_offset: usize, area: Rect) {
    let Some(session) = &pane.pty_session else {
        render_output_placeholder(frame, OUTPUT_SESSION_MISSING_MESSAGE, area);
        return;
    };

    let terminal_handle = session.term();
    let Ok(term) = terminal_handle.lock() else {
        render_output_placeholder(frame, OUTPUT_LOCK_MESSAGE, area);
        return;
    };

    let screen = AlacrittyScreen::new(&*term);
    let cursor = Cursor::default().visibility(false);
    let terminal = PseudoTerminal::new(&screen)
        .cursor(cursor)
        .scroll_offset(scroll_offset);

    frame.render_widget(terminal, area);
}

fn render_output_message(frame: &mut Frame, message: &str, is_focused: bool, area: Rect) {
    let block = section_block(OUTPUT_SECTION_TITLE, is_focused);
    render_lines_section(
        frame,
        vec![Line::from(Span::styled(
            message.to_string(),
            Style::default().fg(Color::DarkGray),
        ))],
        0,
        area,
        block,
    );
}

fn render_output_placeholder(frame: &mut Frame, message: &str, area: Rect) {
    let text = Paragraph::new(Line::from(Span::styled(
        message.to_string(),
        Style::default().fg(Color::DarkGray),
    )));
    frame.render_widget(text, area);
}

fn find_output_pane(app: &App, task_id: Uuid) -> Option<&InstancePane> {
    let task = app.tasks.find_task_by_id(task_id)?;
    let instance_id = task.instance_id?;
    app.instances.find_pane(instance_id)
}

fn section_block(title: &'static str, is_focused: bool) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(section_border_color(is_focused)))
        .title(Span::styled(title, Style::default().fg(Color::DarkGray)))
}

fn lines_block(
    title: &'static str,
    total_lines: usize,
    scroll_offset: usize,
    visible_height: usize,
    is_focused: bool,
) -> Block<'static> {
    let end_line = (scroll_offset + visible_height).min(total_lines);

    section_block(title, is_focused).title_bottom(
        Line::from(vec![Span::styled(
            format!(
                "{LINES_TITLE_PREFIX}{}{LINES_TITLE_SEPARATOR}{}{LINES_TITLE_SUFFIX}{} ",
                scroll_offset + 1,
                end_line,
                total_lines
            ),
            Style::default().fg(Color::DarkGray),
        )])
        .alignment(Alignment::Right),
    )
}

fn output_block(scroll_offset: usize, max_scrollback: usize, is_focused: bool) -> Block<'static> {
    section_block(OUTPUT_SECTION_TITLE, is_focused).title_bottom(
        Line::from(vec![Span::styled(
            format!(
                "{OUTPUT_SCROLL_PREFIX}{scroll_offset}{OUTPUT_SCROLL_SEPARATOR}{max_scrollback} ",
            ),
            Style::default().fg(Color::DarkGray),
        )])
        .alignment(Alignment::Right),
    )
}

const fn section_border_color(is_focused: bool) -> Color {
    if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    }
}

fn inner_height(area: Rect) -> usize {
    let height = usize::from(area.height.saturating_sub(BORDER_HEIGHT_OFFSET));
    height.max(1)
}

fn render_lines_section(
    frame: &mut Frame,
    lines: Vec<Line<'static>>,
    scroll_offset: usize,
    area: Rect,
    block: Block<'static>,
) {
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let visible_lines: Vec<Line> = lines
        .into_iter()
        .skip(scroll_offset)
        .take(inner_area.height as usize)
        .collect();

    let text = Paragraph::new(visible_lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    frame.render_widget(text, inner_area);
}

fn render_action_buttons(
    frame: &mut Frame,
    selected_action: ReviewAction,
    is_clean: bool,
    area: Rect,
) {
    let actions = ReviewAction::all();
    let button_areas = build_action_button_areas(area);

    for (index, action) in actions.iter().enumerate() {
        render_action_button(
            frame,
            button_areas[index],
            *action,
            selected_action,
            is_clean,
        );
    }
}

fn build_action_button_areas(area: Rect) -> Vec<Rect> {
    let button_constraints = vec![Constraint::Percentage(BUTTON_WIDTH_PERCENT); BUTTON_COUNT];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(button_constraints)
        .split(area)
        .to_vec()
}

fn render_action_button(
    frame: &mut Frame,
    area: Rect,
    action: ReviewAction,
    selected_action: ReviewAction,
    is_clean: bool,
) {
    let is_selected = action == selected_action;
    let is_enabled = action.is_enabled(is_clean);
    let label = action.label();

    let button = Paragraph::new(label)
        .alignment(Alignment::Center)
        .style(action_button_style(is_enabled, is_selected))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(action_border_style(is_enabled, is_selected)),
        );

    frame.render_widget(button, area);
}

fn action_button_style(is_enabled: bool, is_selected: bool) -> Style {
    if !is_enabled {
        return Style::default().fg(Color::DarkGray).bg(Color::Black);
    }

    if is_selected {
        return Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }

    Style::default().fg(Color::Cyan).bg(Color::Black)
}

fn action_border_style(is_enabled: bool, is_selected: bool) -> Style {
    if !is_enabled {
        return Style::default().fg(Color::DarkGray);
    }

    if is_selected {
        return Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }

    Style::default().fg(Color::DarkGray)
}
