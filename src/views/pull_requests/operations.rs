use super::state::{PullRequest, PullRequestStatusState, PullRequestsState};

pub fn refresh(state: &mut PullRequestsState) {
    state.is_loading = true;

    let pull_requests = fetch_from_github();

    match pull_requests {
        Ok(pull_requests) => {
            state.set_pull_requests(pull_requests);
            state.mark_refreshed();
        }
        Err(error) => {
            state.set_error(error);
            state.mark_refreshed();
        }
    }
}

fn fetch_from_github() -> Result<Vec<PullRequest>, String> {
    let output = std::process::Command::new("gh")
        .args([
            "pr",
            "list",
            "--json",
            "number,title,author,headRefName,baseRefName,state,isDraft,additions,deletions,url",
            "--limit",
            "50",
        ])
        .output()
        .map_err(|error| format!("Failed to run gh command: {error}. Is GitHub CLI installed?"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("GitHub CLI error: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_github_response(&stdout)
}

fn parse_github_response(json_output: &str) -> Result<Vec<PullRequest>, String> {
    let parsed: serde_json::Value = serde_json::from_str(json_output)
        .map_err(|error| format!("Failed to parse JSON: {error}"))?;

    let array = parsed
        .as_array()
        .ok_or_else(|| "Expected JSON array".to_string())?;

    let pull_requests = array
        .iter()
        .filter_map(|item| {
            let number = item.get("number")?.as_u64()?;
            let title = item.get("title")?.as_str()?.to_string();
            let author = item
                .get("author")
                .and_then(|author| author.get("login"))
                .and_then(|login| login.as_str())
                .unwrap_or("unknown")
                .to_string();
            let branch = item.get("headRefName")?.as_str()?.to_string();
            let base_branch = item.get("baseRefName")?.as_str()?.to_string();
            let state_string = item.get("state")?.as_str()?;
            let is_draft = item
                .get("isDraft")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            let additions = item
                .get("additions")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            let deletions = item
                .get("deletions")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            let url = item.get("url")?.as_str()?.to_string();

            let state = match state_string {
                "CLOSED" => PullRequestStatusState::Closed,
                "MERGED" => PullRequestStatusState::Merged,
                _ => PullRequestStatusState::Open,
            };

            Some(PullRequest {
                number,
                title,
                author,
                branch,
                base_branch,
                state,
                is_draft,
                additions,
                deletions,
                url,
            })
        })
        .collect();

    Ok(pull_requests)
}

pub fn open_url_in_browser(url: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let command = "open";
    #[cfg(target_os = "linux")]
    let command = "xdg-open";
    #[cfg(target_os = "windows")]
    let command = "start";

    std::process::Command::new(command)
        .arg(url)
        .spawn()
        .map_err(|error| format!("Failed to open URL: {error}"))?;

    Ok(())
}
