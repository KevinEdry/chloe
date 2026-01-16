use crate::views::worktree::WorktreeStatus;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

const STATUS_HEADER_MAX_FILES: usize = 3;
const MODIFIED_FILE_PREFIX: &str = "  M ";
const UNTRACKED_FILE_PREFIX: &str = "  ? ";
const WORKTREE_MISSING_MESSAGE: &str = "No worktree associated";
const BRANCH_LABEL: &str = "Branch: ";
const READY_STATUS_MESSAGE: &str = "Status: Ready to merge";
const CONFLICT_STATUS_MESSAGE: &str = "Status: Has merge conflicts";
const UNCOMMITTED_STATUS_PREFIX: &str = "Status: ";
const UNCOMMITTED_STATUS_SUFFIX: &str = " uncommitted change";
const MORE_FILES_PREFIX: &str = "  ... and ";
const MORE_FILES_SUFFIX: &str = " more";
const AHEAD_STATUS_PREFIX: &str = "Ahead of ";
const AHEAD_STATUS_SEPARATOR: &str = ": ";
const AHEAD_STATUS_SUFFIX: &str = " commit";

#[must_use]
pub fn build_status_lines(
    branch_name: Option<&str>,
    base_branch_name: Option<&str>,
    commits_ahead_count: Option<usize>,
    status: &WorktreeStatus,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.push(build_branch_line(branch_name));
    if let Some(line) = build_ahead_line(base_branch_name, commits_ahead_count) {
        lines.push(line);
    }
    lines.push(build_status_line(status));
    append_changed_files_lines(status, &mut lines);

    lines
}

fn build_branch_line(branch_name: Option<&str>) -> Line<'static> {
    let Some(branch) = branch_name else {
        return Line::from(Span::styled(
            WORKTREE_MISSING_MESSAGE.to_string(),
            Style::default().fg(Color::DarkGray),
        ));
    };

    Line::from(vec![
        Span::styled(
            BRANCH_LABEL.to_string(),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(branch.to_string(), Style::default().fg(Color::White)),
    ])
}

fn build_ahead_line(
    base_branch_name: Option<&str>,
    commits_ahead_count: Option<usize>,
) -> Option<Line<'static>> {
    let base_branch_name = base_branch_name?;
    let commits_ahead_count = commits_ahead_count?;

    let suffix = if commits_ahead_count == 1 { "" } else { "s" };
    let label = format!("{AHEAD_STATUS_PREFIX}{base_branch_name}{AHEAD_STATUS_SEPARATOR}");
    let value = format!("{commits_ahead_count}{AHEAD_STATUS_SUFFIX}{suffix}");

    Some(Line::from(vec![
        Span::styled(label, Style::default().fg(Color::DarkGray)),
        Span::styled(value, Style::default().fg(Color::White)),
    ]))
}

fn build_status_line(status: &WorktreeStatus) -> Line<'static> {
    if status.is_clean {
        return Line::from(Span::styled(
            READY_STATUS_MESSAGE.to_string(),
            Style::default().fg(Color::Green),
        ));
    }

    if status.has_conflicts {
        return Line::from(Span::styled(
            CONFLICT_STATUS_MESSAGE.to_string(),
            Style::default().fg(Color::Red),
        ));
    }

    let count = status.uncommitted_count();
    let suffix = if count == 1 { "" } else { "s" };
    let message = format!("{UNCOMMITTED_STATUS_PREFIX}{count}{UNCOMMITTED_STATUS_SUFFIX}{suffix}");

    Line::from(Span::styled(message, Style::default().fg(Color::Yellow)))
}

fn append_changed_files_lines(status: &WorktreeStatus, lines: &mut Vec<Line<'static>>) {
    if status.is_clean {
        return;
    }

    lines.push(Line::from(String::new()));

    let mut shown = append_file_lines(
        lines,
        &status.modified_files,
        MODIFIED_FILE_PREFIX,
        Color::Yellow,
        0,
    );

    shown = append_file_lines(
        lines,
        &status.untracked_files,
        UNTRACKED_FILE_PREFIX,
        Color::Green,
        shown,
    );

    let total = status.uncommitted_count();
    if total > shown {
        lines.push(Line::from(Span::styled(
            format!("{MORE_FILES_PREFIX}{}{MORE_FILES_SUFFIX}", total - shown),
            Style::default().fg(Color::DarkGray),
        )));
    }
}

fn append_file_lines(
    lines: &mut Vec<Line<'static>>,
    files: &[String],
    prefix: &str,
    color: Color,
    shown: usize,
) -> usize {
    let mut current_shown = shown;

    for file in files {
        if current_shown >= STATUS_HEADER_MAX_FILES {
            break;
        }

        lines.push(Line::from(vec![
            Span::styled(prefix.to_string(), Style::default().fg(color)),
            Span::styled(file.clone(), Style::default().fg(Color::White)),
        ]));
        current_shown += 1;
    }

    current_shown
}
