use super::state::{ActivityEventType, InstancePane};
use regex::Regex;

pub fn detect_and_log_activity(pane: &mut InstancePane, output: &str) {
    if let Some(command) = detect_command_execution(output) {
        pane.add_activity_event(
            ActivityEventType::CommandExecuted,
            format!("Executed: {command}"),
            None,
        );
    }

    if let Some(file_change) = detect_file_change(output) {
        pane.add_activity_event(
            ActivityEventType::FileChanged,
            format!("Modified: {file_change}"),
            None,
        );
    }

    if detect_task_completion(output) {
        pane.add_activity_event(
            ActivityEventType::TaskCompleted,
            "Task marked as complete".to_string(),
            None,
        );
    }

    if let Some(error_message) = detect_error(output) {
        pane.add_activity_event(ActivityEventType::ErrorOccurred, error_message, None);
    }

    if let Some(notification) = detect_provider_notification(output) {
        pane.add_activity_event(ActivityEventType::ProviderNotification, notification, None);
    }
}

fn detect_command_execution(output: &str) -> Option<String> {
    let shell_prompt_pattern: Regex = Regex::new(r"(?m)^\$\s+(.+)$").ok()?;

    shell_prompt_pattern
        .captures(output)
        .and_then(|captures| captures.get(1))
        .map(|command| command.as_str().trim().to_string())
        .filter(|s| !s.is_empty())
}

fn detect_file_change(output: &str) -> Option<String> {
    let pattern1: Regex =
        Regex::new(r"(?:Created|Writing|Wrote|Modified|Updated)\s+(?:file\s+)?[`']?([^\s`']+)")
            .ok()?;
    if let Some(captures) = pattern1.captures(output)
        && let Some(file_match) = captures.get(1)
    {
        return Some(file_match.as_str().to_string());
    }

    let pattern2: Regex = Regex::new(r">\s+([^\s]+\.(?:rs|js|ts|py|go|java|cpp|c|h))").ok()?;
    if let Some(captures) = pattern2.captures(output)
        && let Some(file_match) = captures.get(1)
    {
        return Some(file_match.as_str().to_string());
    }

    if output.contains("git diff") || output.contains("modified:") {
        return Some("detected via git diff".to_string());
    }

    None
}

fn detect_task_completion(output: &str) -> bool {
    let completion_indicators = [
        "task complete",
        "done",
        "finished",
        "successfully completed",
        "all tests passed",
    ];

    let output_lower = output.to_lowercase();
    completion_indicators
        .iter()
        .any(|indicator| output_lower.contains(indicator))
}

fn detect_error(output: &str) -> Option<String> {
    let error_pattern: Regex = Regex::new(r"(?i)error:\s*(.{0,100})").ok()?;
    if let Some(captures) = error_pattern.captures(output)
        && let Some(error_match) = captures.get(1)
    {
        let error_text = error_match.as_str().trim();
        if !error_text.is_empty() {
            return Some(format!("Error: {error_text}"));
        }
    }

    let exception_pattern: Regex = Regex::new(r"(?i)exception:\s*(.{0,100})").ok()?;
    if let Some(captures) = exception_pattern.captures(output)
        && let Some(error_match) = captures.get(1)
    {
        let error_text = error_match.as_str().trim();
        if !error_text.is_empty() {
            return Some(format!("Exception: {error_text}"));
        }
    }

    let failed_pattern: Regex = Regex::new(r"(?i)failed:\s*(.{0,100})").ok()?;
    if let Some(captures) = failed_pattern.captures(output)
        && let Some(error_match) = captures.get(1)
    {
        let error_text = error_match.as_str().trim();
        if !error_text.is_empty() {
            return Some(format!("Failed: {error_text}"));
        }
    }

    let exit_code_pattern: Regex = Regex::new(r"exit code:\s*([1-9]\d*)").ok()?;
    if exit_code_pattern.is_match(output) {
        return Some("Process exited with error code".to_string());
    }

    None
}

fn detect_provider_notification(output: &str) -> Option<String> {
    if let Ok(claude_pattern) = Regex::new(r"(?i)claude(?:\s+code)?:\s*(.{0,100})")
        && let Some(captures) = claude_pattern.captures(output)
        && let Some(notification_match) = captures.get(1)
    {
        let notification_text = notification_match.as_str().trim();
        if !notification_text.is_empty() {
            return Some(notification_text.to_string());
        }
    }

    if let Ok(agent_pattern) = Regex::new(r"(?i)agent:\s*(.{0,100})")
        && let Some(captures) = agent_pattern.captures(output)
        && let Some(notification_match) = captures.get(1)
    {
        let notification_text = notification_match.as_str().trim();
        if !notification_text.is_empty() {
            return Some(notification_text.to_string());
        }
    }

    if let Ok(assistant_pattern) = Regex::new(r"(?i)assistant:\s*(.{0,100})")
        && let Some(captures) = assistant_pattern.captures(output)
        && let Some(notification_match) = captures.get(1)
    {
        let notification_text = notification_match.as_str().trim();
        if !notification_text.is_empty() {
            return Some(notification_text.to_string());
        }
    }

    None
}
