use crate::app::App;
use super::diff;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use std::{fs, path::Path, process::Command};
use uuid::Uuid;

const MAX_DIFF_LINES_PER_FILE: usize = 120;
const FILE_DIFF_TRUNCATION_MESSAGE: &str = "... file diff truncated ...";
const TASK_NOT_FOUND_MESSAGE: &str = "Task not found";
const NO_CHANGES_MESSAGE: &str = "No changes to review";
const NO_DIFF_MESSAGE: &str = "No diff available";
const UNREADABLE_FILE_MESSAGE: &str = "Binary or unreadable file";
const ORIGINAL_LABEL_PREFIX: &str = "original/";
const UPDATED_LABEL_PREFIX: &str = "updated/";
const SUMMARY_PREFIX: &str = "Changes: +";
const SUMMARY_SEPARATOR: &str = " -";
const STATUS_CODE_LENGTH: usize = 2;
const STATUS_PATH_OFFSET: usize = 3;
const STATUS_RENAME_SEPARATOR: &str = " -> ";
const STATUS_UNTRACKED: &str = "??";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangedFileKind {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Conflict,
}

impl ChangedFileKind {
    #[must_use]
    pub const fn color(self) -> Color {
        match self {
            Self::Modified => Color::Yellow,
            Self::Added | Self::Untracked => Color::Green,
            Self::Deleted => Color::Red,
            Self::Renamed => Color::Cyan,
            Self::Copied => Color::Blue,
            Self::Conflict => Color::LightRed,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Modified => "Modified",
            Self::Added => "Added",
            Self::Deleted => "Deleted",
            Self::Renamed => "Renamed",
            Self::Copied => "Copied",
            Self::Untracked => "Untracked",
            Self::Conflict => "Conflict",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChangedFile {
    pub path: String,
    pub original_path: Option<String>,
    pub kind: ChangedFileKind,
}

pub struct DiffPanelState {
    pub files: Vec<ChangedFile>,
    pub selected_index: usize,
    pub lines: Vec<Line<'static>>,
}

#[must_use]
pub fn build_diff_panel(app: &App, task_id: Uuid, selected_index: usize) -> DiffPanelState {
    let Some(worktree_path) = task_worktree_path(app, task_id) else {
        return empty_diff_panel(TASK_NOT_FOUND_MESSAGE);
    };

    let files = load_changed_files(&worktree_path);
    if files.is_empty() {
        return empty_diff_panel(NO_CHANGES_MESSAGE);
    }

    let selected_index = selected_index.min(files.len().saturating_sub(1));
    let selected_file = &files[selected_index];
    let lines = build_selected_file_diff_lines(&worktree_path, selected_file);

    DiffPanelState {
        files,
        selected_index,
        lines,
    }
}

#[must_use]
pub fn build_file_list_lines(
    files: &[ChangedFile],
    selected_index: usize,
    is_focused: bool,
    max_lines: usize,
) -> Vec<Line<'static>> {
    if files.is_empty() {
        return vec![message_line(NO_CHANGES_MESSAGE, Color::DarkGray)];
    }

    let visible_range = list_window_range(files.len(), selected_index, max_lines);
    let mut lines = Vec::new();

    for (index, file) in files
        .iter()
        .enumerate()
        .skip(visible_range.start)
        .take(visible_range.length)
    {
        lines.push(build_file_list_line(
            file,
            index == selected_index,
            is_focused,
        ));
    }

    lines
}

fn task_worktree_path(app: &App, task_id: Uuid) -> Option<std::path::PathBuf> {
    let task = app.tasks.find_task_by_id(task_id)?;
    let worktree_info = task.worktree_info.as_ref()?;
    Some(worktree_info.worktree_path.clone())
}

fn empty_diff_panel(message: &str) -> DiffPanelState {
    DiffPanelState {
        files: Vec::new(),
        selected_index: 0,
        lines: vec![message_line(message, Color::DarkGray)],
    }
}

fn load_changed_files(worktree_path: &Path) -> Vec<ChangedFile> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(worktree_path)
        .output();

    let Ok(output) = output else {
        return Vec::new();
    };

    if !output.status.success() {
        return Vec::new();
    }

    let status_text = String::from_utf8_lossy(&output.stdout);
    let mut files = Vec::new();

    for line in status_text.lines() {
        if let Some(file) = parse_status_line(line) {
            files.push(file);
        }
    }

    files
}

fn parse_status_line(line: &str) -> Option<ChangedFile> {
    if line.len() < STATUS_PATH_OFFSET {
        return None;
    }

    let status_code = &line[..STATUS_CODE_LENGTH];
    let path_segment = &line[STATUS_PATH_OFFSET..];
    let kind = parse_status_kind(status_code);
    let (path, original_path) = split_status_path(path_segment, kind);

    Some(ChangedFile {
        path,
        original_path,
        kind,
    })
}

fn parse_status_kind(status_code: &str) -> ChangedFileKind {
    if status_code == STATUS_UNTRACKED {
        return ChangedFileKind::Untracked;
    }

    if status_code.contains('U') {
        return ChangedFileKind::Conflict;
    }

    if status_code.contains('R') {
        return ChangedFileKind::Renamed;
    }

    if status_code.contains('C') {
        return ChangedFileKind::Copied;
    }

    if status_code.contains('D') {
        return ChangedFileKind::Deleted;
    }

    if status_code.contains('A') {
        return ChangedFileKind::Added;
    }

    ChangedFileKind::Modified
}

fn split_status_path(path_segment: &str, kind: ChangedFileKind) -> (String, Option<String>) {
    if matches!(kind, ChangedFileKind::Renamed | ChangedFileKind::Copied)
        && let Some((original, updated)) = path_segment.split_once(STATUS_RENAME_SEPARATOR)
    {
        return (updated.to_string(), Some(original.to_string()));
    }

    (path_segment.to_string(), None)
}

fn build_selected_file_diff_lines(worktree_path: &Path, file: &ChangedFile) -> Vec<Line<'static>> {
    let contents = resolve_diff_contents(worktree_path, file);
    if contents.is_unreadable {
        return vec![message_line(UNREADABLE_FILE_MESSAGE, Color::DarkGray)];
    }

    let original_label = format!("{ORIGINAL_LABEL_PREFIX}{}", contents.original_label);
    let updated_label = format!("{UPDATED_LABEL_PREFIX}{}", contents.updated_label);

    let mut diff_lines = build_file_diff_lines(
        &contents.original,
        &contents.updated,
        &original_label,
        &updated_label,
    );

    if let Some(summary) = build_change_summary_line(&contents.original, &contents.updated) {
        diff_lines.insert(0, summary);
    }

    if diff_lines.len() > MAX_DIFF_LINES_PER_FILE {
        diff_lines.truncate(MAX_DIFF_LINES_PER_FILE);
        diff_lines.push(message_line(FILE_DIFF_TRUNCATION_MESSAGE, Color::DarkGray));
    }

    diff_lines
}

struct DiffContents {
    original: String,
    updated: String,
    original_label: String,
    updated_label: String,
    is_unreadable: bool,
}

fn resolve_diff_contents(worktree_path: &Path, file: &ChangedFile) -> DiffContents {
    let original_path = file.original_path.as_deref().unwrap_or(&file.path);
    let updated_path = &file.path;

    let original_content = match file.kind {
        ChangedFileKind::Added | ChangedFileKind::Untracked => Some(String::new()),
        _ => read_repository_file(worktree_path, Path::new(original_path)),
    };

    let updated_content = match file.kind {
        ChangedFileKind::Deleted => Some(String::new()),
        _ => read_worktree_file(worktree_path, Path::new(updated_path)),
    };

    let is_unreadable = original_content.is_none() || updated_content.is_none();

    DiffContents {
        original: original_content.unwrap_or_default(),
        updated: updated_content.unwrap_or_default(),
        original_label: original_path.to_string(),
        updated_label: updated_path.clone(),
        is_unreadable,
    }
}

fn build_file_diff_lines(
    original_content: &str,
    updated_content: &str,
    original_label: &str,
    updated_label: &str,
) -> Vec<Line<'static>> {
    let difference = diff::build_unified_diff(
        original_content,
        updated_content,
        original_label,
        updated_label,
    );

    let mut lines: Vec<Line<'static>> = difference
        .lines()
        .map(|line| Line::from(Span::styled(line.to_string(), diff_line_style(line))))
        .collect();

    if lines.is_empty() {
        lines.push(message_line(NO_DIFF_MESSAGE, Color::DarkGray));
    }

    lines
}

fn build_change_summary_line(original: &str, updated: &str) -> Option<Line<'static>> {
    let changes = diff::build_side_by_side_lines(original, updated);
    let mut added = 0;
    let mut removed = 0;

    for change in changes {
        match change.kind {
            diff::DiffLineKind::Added => added += 1,
            diff::DiffLineKind::Removed => removed += 1,
            diff::DiffLineKind::Unchanged => {}
        }
    }

    if added == 0 && removed == 0 {
        return None;
    }

    Some(Line::from(Span::styled(
        format!("{SUMMARY_PREFIX}{added}{SUMMARY_SEPARATOR}{removed}"),
        Style::default().fg(Color::Yellow),
    )))
}

fn diff_line_style(line: &str) -> Style {
    if line.starts_with("+++") || line.starts_with("---") {
        return Style::default().fg(Color::Cyan);
    }

    if line.starts_with("@@") {
        return Style::default().fg(Color::Yellow);
    }

    if line.starts_with('+') {
        return Style::default().fg(Color::Green);
    }

    if line.starts_with('-') {
        return Style::default().fg(Color::Red);
    }

    Style::default().fg(Color::White)
}

fn message_line(message: &str, color: Color) -> Line<'static> {
    Line::from(Span::styled(
        message.to_string(),
        Style::default().fg(color),
    ))
}

fn read_worktree_file(worktree_path: &Path, relative_path: &Path) -> Option<String> {
    let file_path = worktree_path.join(relative_path);
    let content = fs::read(file_path).ok()?;
    String::from_utf8(content).ok()
}

fn read_repository_file(worktree_path: &Path, relative_path: &Path) -> Option<String> {
    let file_reference = format!("HEAD:{}", relative_path.to_string_lossy());
    let output = Command::new("git")
        .arg("show")
        .arg(file_reference)
        .current_dir(worktree_path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout).ok()
}

struct ListWindowRange {
    start: usize,
    length: usize,
}

fn list_window_range(
    total_files: usize,
    selected_index: usize,
    max_lines: usize,
) -> ListWindowRange {
    let max_lines = max_lines.max(1).min(total_files.max(1));
    let selected_index = selected_index.min(total_files.saturating_sub(1));

    if total_files <= max_lines {
        return ListWindowRange {
            start: 0,
            length: total_files,
        };
    }

    let start = selected_index.saturating_sub(max_lines.saturating_sub(1));

    ListWindowRange {
        start,
        length: max_lines,
    }
}

fn build_file_list_line(file: &ChangedFile, is_selected: bool, is_focused: bool) -> Line<'static> {
    let prefix = format!("{} ", file.kind.label());
    let base_style = Style::default().fg(file.kind.color());
    let line_style = selected_line_style(base_style, is_selected, is_focused);

    Line::from(vec![
        Span::styled(prefix, line_style),
        Span::styled(file.path.clone(), line_style),
    ])
}

const fn selected_line_style(base_style: Style, is_selected: bool, is_focused: bool) -> Style {
    if !is_selected {
        return base_style;
    }

    if is_focused {
        return base_style
            .bg(Color::Cyan)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD);
    }

    base_style
        .bg(Color::DarkGray)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}
