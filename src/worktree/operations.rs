use super::state::{Worktree, WorktreeInfo};
use anyhow::{Context, Result, anyhow};
use git2::{BranchType, Repository};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const MAX_SLUG_LENGTH: usize = 50;

/// Generate `.claude_settings.json` in the worktree to pre-configure permissions
fn generate_claude_settings(worktree_path: &Path) -> Result<()> {
    let settings = serde_json::json!({
        "permissions": {
            "Read": ["./**"],
            "Write": ["./**"],
            "Edit": ["./**"],
            "Glob": ["./**"],
            "Grep": ["./**"],
            "Skill": ["./**"]
        },
        "sandbox": {
            "enabled": true,
            "autoAllowBashIfSandboxed": true
        },
        "includeCoAuthoredBy": false,
        "gitAttribution": false
    });

    let settings_path = worktree_path.join(".claude_settings.json");
    let settings_content = serde_json::to_string_pretty(&settings)
        .context("Failed to serialize Claude settings")?;

    fs::write(&settings_path, settings_content)
        .context("Failed to write .claude_settings.json")?;

    Ok(())
}

/// Result of attempting to merge a worktree branch
#[derive(Debug)]
pub enum MergeResult {
    /// Merge completed successfully
    Success,
    /// Merge has conflicts that need resolution
    Conflicts { conflicted_files: Vec<String> },
}

/// Check if the current directory is inside a git repository
#[must_use]
pub fn is_git_repository(path: &Path) -> bool {
    Repository::discover(path).is_ok()
}

/// Get the repository root for a given path
pub fn find_repository_root(path: &Path) -> Result<PathBuf> {
    let repository = Repository::discover(path).context("Not a git repository")?;

    let workdir = repository
        .workdir()
        .ok_or_else(|| anyhow!("Bare repository has no working directory"))?;

    Ok(workdir.to_path_buf())
}

/// List all existing worktrees in the repository
pub fn list_worktrees(repository_path: &Path) -> Result<Vec<Worktree>> {
    let repository = Repository::open(repository_path).context("Failed to open git repository")?;

    let worktree_list = repository.worktrees().context("Failed to list worktrees")?;

    let mut worktrees = Vec::new();

    for worktree_name in worktree_list.iter().flatten() {
        let worktree = match repository.find_worktree(worktree_name) {
            Ok(wt) => wt,
            Err(_) => continue,
        };

        let path = worktree.path().to_path_buf();

        let branch_name = extract_branch_name_from_worktree(&repository, &path)
            .unwrap_or_else(|| "(detached)".to_string());

        worktrees.push(Worktree {
            path,
            branch_name: branch_name.clone(),
            is_bare: false,
            is_detached: branch_name == "(detached)",
        });
    }

    Ok(worktrees)
}

/// Generate a valid git branch name from a task title
/// Example: "Implement Worktree Support" -> "task/implement-worktree-support"
#[must_use]
pub fn generate_branch_name(task_title: &str) -> String {
    let slug = task_title
        .to_lowercase()
        .chars()
        .map(|character| {
            if character.is_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();

    let slug = slug
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    let truncated_slug = if slug.len() > MAX_SLUG_LENGTH {
        &slug[..MAX_SLUG_LENGTH]
    } else {
        &slug
    };

    format!("task/{truncated_slug}")
}

/// Create a new worktree for a task
/// Returns `WorktreeInfo` with the branch name and path
pub fn create_worktree(
    repository_path: &Path,
    task_title: &str,
    task_id: &Uuid,
) -> Result<WorktreeInfo> {
    let repository = Repository::open(repository_path).context("Failed to open git repository")?;

    let branch_name = generate_branch_name(task_title);

    let branch_exists = repository
        .find_branch(&branch_name, BranchType::Local)
        .is_ok();

    let final_branch_name = if branch_exists {
        let short_id = &task_id.to_string()[..8];
        format!("{branch_name}-{short_id}")
    } else {
        branch_name
    };

    let worktree_dir_name = final_branch_name.replace('/', "-");
    let worktree_path = repository_path
        .parent()
        .unwrap_or(repository_path)
        .join(format!(".worktrees/{worktree_dir_name}"));

    let output = std::process::Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg(&worktree_path)
        .arg("-b")
        .arg(&final_branch_name)
        .current_dir(repository_path)
        .output()
        .context("Failed to execute git worktree add")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Git worktree add failed: {error_message}"));
    }

    let worktree_info = WorktreeInfo::new(final_branch_name, worktree_path.clone());

    generate_claude_settings(&worktree_path)?;

    Ok(worktree_info)
}

/// Merge a worktree branch into main
/// Returns MergeResult indicating success or conflicts
pub fn merge_worktree_to_main(
    repository_path: &Path,
    worktree_info: &WorktreeInfo,
) -> Result<MergeResult> {
    let branch_name = &worktree_info.branch_name;

    let stash_output = std::process::Command::new("git")
        .arg("stash")
        .arg("--include-untracked")
        .current_dir(repository_path)
        .output()
        .context("Failed to stash changes")?;

    if !stash_output.status.success() {
        let error_message = String::from_utf8_lossy(&stash_output.stderr);
        return Err(anyhow!("Git stash failed: {error_message}"));
    }

    let checkout_output = std::process::Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(repository_path)
        .output()
        .context("Failed to checkout main branch")?;

    if !checkout_output.status.success() {
        let error_message = String::from_utf8_lossy(&checkout_output.stderr);
        return Err(anyhow!("Git checkout main failed: {error_message}"));
    }

    let merge_output = std::process::Command::new("git")
        .arg("merge")
        .arg(branch_name)
        .arg("--no-edit")
        .current_dir(repository_path)
        .output()
        .context("Failed to execute git merge")?;

    if !merge_output.status.success() {
        let stderr = String::from_utf8_lossy(&merge_output.stderr);

        if stderr.contains("CONFLICT") || stderr.contains("conflict") {
            let conflicted_files = get_conflicted_files(repository_path)?;
            return Ok(MergeResult::Conflicts { conflicted_files });
        }

        let error_message = String::from_utf8_lossy(&merge_output.stderr);
        return Err(anyhow!("Git merge failed: {error_message}"));
    }

    let push_output = std::process::Command::new("git")
        .arg("push")
        .current_dir(repository_path)
        .output()
        .context("Failed to push to remote")?;

    if !push_output.status.success() {
        let error_message = String::from_utf8_lossy(&push_output.stderr);
        return Err(anyhow!("Git push failed: {error_message}"));
    }

    Ok(MergeResult::Success)
}

/// Get list of conflicted files from git status
fn get_conflicted_files(repository_path: &Path) -> Result<Vec<String>> {
    let status_output = std::process::Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(repository_path)
        .output()
        .context("Failed to get git status")?;

    if !status_output.status.success() {
        return Err(anyhow!("Git status failed"));
    }

    let status_text = String::from_utf8_lossy(&status_output.stdout);
    let conflicted_files: Vec<String> = status_text
        .lines()
        .filter(|line| {
            line.starts_with("UU ") || line.starts_with("AA ") || line.starts_with("DD ")
        })
        .map(|line| line[3..].to_string())
        .collect();

    Ok(conflicted_files)
}

/// Delete a worktree (cleanup)
pub fn delete_worktree(repository_path: &Path, worktree_info: &WorktreeInfo) -> Result<()> {
    let output = std::process::Command::new("git")
        .arg("worktree")
        .arg("remove")
        .arg(&worktree_info.worktree_path)
        .arg("--force")
        .current_dir(repository_path)
        .output()
        .context("Failed to execute git worktree remove")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Git worktree remove failed: {error_message}"));
    }

    let repository = Repository::open(repository_path).context("Failed to open git repository")?;

    let mut branch = repository
        .find_branch(&worktree_info.branch_name, BranchType::Local)
        .context("Failed to find branch")?;

    branch.delete().context("Failed to delete branch")?;

    Ok(())
}

fn extract_branch_name_from_worktree(
    _repository: &Repository,
    worktree_path: &Path,
) -> Option<String> {
    let worktree_repository = Repository::open(worktree_path).ok()?;
    let head = worktree_repository.head().ok()?;

    if head.is_branch() {
        head.shorthand().map(String::from)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_branch_name_basic() {
        let result = generate_branch_name("Implement Worktree Support");
        assert_eq!(result, "task/implement-worktree-support");
    }

    #[test]
    fn test_generate_branch_name_with_special_chars() {
        let result = generate_branch_name("Fix bug #123: API timeout");
        assert_eq!(result, "task/fix-bug-123-api-timeout");
    }

    #[test]
    fn test_generate_branch_name_truncation() {
        let long_title = "A".repeat(100);
        let result = generate_branch_name(&long_title);
        assert!(result.len() <= 55);
    }

    #[test]
    fn test_generate_branch_name_with_consecutive_dashes() {
        let result = generate_branch_name("Multiple   spaces   and---dashes");
        assert_eq!(result, "task/multiple-spaces-and-dashes");
    }
}
