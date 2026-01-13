use super::state::{Worktree, WorktreeInfo};
use crate::views::settings::VcsCommand;
use anyhow::{Context, Result, anyhow};
use git2::{BranchType, Repository};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const MAX_SLUG_LENGTH: usize = 50;

/// Generate `.claude/settings.local.json` in the worktree to pre-configure permissions
fn generate_claude_settings(worktree_path: &Path, task_id: &Uuid) -> Result<()> {
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
        "gitAttribution": false,
        "hooks": {
            "SessionStart": [{
                "hooks": [{
                    "type": "command",
                    "command": format!("chloe notify start --worktree-id {}", task_id)
                }]
            }],
            "SessionEnd": [{
                "hooks": [{
                    "type": "command",
                    "command": format!("chloe notify end --worktree-id {}", task_id)
                }]
            }],
            "PermissionRequest": [{
                "matcher": "*",
                "hooks": [{
                    "type": "command",
                    "command": format!("chloe notify permission --worktree-id {}", task_id)
                }]
            }],
            "PostToolUse": [{
                "matcher": "*",
                "hooks": [{
                    "type": "command",
                    "command": format!("chloe notify start --worktree-id {}", task_id)
                }]
            }],
            "Stop": [{
                "hooks": [{
                    "type": "command",
                    "command": format!("chloe notify end --worktree-id {}", task_id)
                }]
            }],
            "UserPromptSubmit": [{
                "hooks": [{
                    "type": "command",
                    "command": format!("chloe notify start --worktree-id {}", task_id)
                }]
            }]
        }
    });

    let claude_dir = worktree_path.join(".claude");
    fs::create_dir_all(&claude_dir).context("Failed to create .claude directory")?;

    let settings_path = claude_dir.join("settings.local.json");
    let settings_content =
        serde_json::to_string_pretty(&settings).context("Failed to serialize Claude settings")?;

    fs::write(&settings_path, settings_content)
        .context("Failed to write .claude/settings.local.json")?;

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

/// Status of a worktree's working directory
#[derive(Debug, Clone, Default)]
pub struct WorktreeStatus {
    /// Whether the worktree has no uncommitted changes
    pub is_clean: bool,
    /// Files that have been modified
    pub modified_files: Vec<String>,
    /// Files that are untracked
    pub untracked_files: Vec<String>,
    /// Whether there are merge conflicts
    pub has_conflicts: bool,
}

impl WorktreeStatus {
    /// Total count of uncommitted files (modified + untracked)
    #[must_use]
    pub const fn uncommitted_count(&self) -> usize {
        self.modified_files.len() + self.untracked_files.len()
    }
}

/// Get the status of a worktree (uncommitted changes, untracked files, conflicts)
///
/// # Errors
///
/// Returns an error if git status command fails.
pub fn get_worktree_status(worktree_path: &Path) -> Result<WorktreeStatus> {
    let output = std::process::Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(worktree_path)
        .output()
        .context("Failed to get git status")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Git status failed: {error_message}"));
    }

    let status_text = String::from_utf8_lossy(&output.stdout);
    let mut modified_files = Vec::new();
    let mut untracked_files = Vec::new();
    let mut has_conflicts = false;

    for line in status_text.lines() {
        if line.len() < 3 {
            continue;
        }

        let status_code = &line[..2];
        let file_path = line[3..].to_string();

        match status_code {
            "UU" | "AA" | "DD" | "AU" | "UA" | "DU" | "UD" => {
                has_conflicts = true;
                modified_files.push(file_path);
            }
            "??" => {
                untracked_files.push(file_path);
            }
            _ => {
                if !status_code.trim().is_empty() {
                    modified_files.push(file_path);
                }
            }
        }
    }

    let is_clean = modified_files.is_empty() && untracked_files.is_empty();

    Ok(WorktreeStatus {
        is_clean,
        modified_files,
        untracked_files,
        has_conflicts,
    })
}

/// Get the current branch name for a repository
///
/// # Errors
///
/// Returns an error if git command fails or HEAD is detached.
pub fn get_current_branch(repository_path: &Path) -> Result<String> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(repository_path)
        .output()
        .context("Failed to get current branch")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to get current branch: {error_message}"));
    }

    let branch_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if branch_name == "HEAD" {
        return Err(anyhow!("HEAD is detached, no branch checked out"));
    }

    Ok(branch_name)
}

/// Commit all changes in a worktree
///
/// # Errors
///
/// Returns an error if git add or commit fails.
#[allow(dead_code)]
pub fn commit_worktree_changes(worktree_path: &Path, message: &str) -> Result<()> {
    let add_output = std::process::Command::new("git")
        .arg("add")
        .arg("-A")
        .current_dir(worktree_path)
        .output()
        .context("Failed to stage changes")?;

    if !add_output.status.success() {
        let error_message = String::from_utf8_lossy(&add_output.stderr);
        return Err(anyhow!("Git add failed: {error_message}"));
    }

    let commit_output = std::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(worktree_path)
        .output()
        .context("Failed to commit changes")?;

    if !commit_output.status.success() {
        let error_message = String::from_utf8_lossy(&commit_output.stderr);
        return Err(anyhow!("Git commit failed: {error_message}"));
    }

    Ok(())
}

/// Get the default branch name (main or master) for the repository
///
/// # Errors
///
/// Returns an error if the repository cannot be opened or HEAD cannot be read.
pub fn get_default_branch(repository_path: &Path) -> Result<String> {
    let repository = Repository::open(repository_path).context("Failed to open git repository")?;

    if repository.find_branch("main", BranchType::Local).is_ok() {
        return Ok("main".to_string());
    }

    if repository.find_branch("master", BranchType::Local).is_ok() {
        return Ok("master".to_string());
    }

    let head = repository.head().context("Failed to get HEAD reference")?;
    if let Some(name) = head.shorthand() {
        return Ok(name.to_string());
    }

    Ok("main".to_string())
}

/// Check if merging a branch into the default branch would cause conflicts
///
/// # Errors
///
/// Returns an error if the repository cannot be opened or merge analysis fails.
pub fn check_merge_conflicts(
    repository_path: &Path,
    worktree_info: &WorktreeInfo,
) -> Result<Option<Vec<String>>> {
    let repository = Repository::open(repository_path).context("Failed to open git repository")?;
    let default_branch = get_default_branch(repository_path)?;

    let our_branch = repository
        .find_branch(&default_branch, BranchType::Local)
        .context("Failed to find default branch")?;
    let their_branch = repository
        .find_branch(&worktree_info.branch_name, BranchType::Local)
        .context("Failed to find worktree branch")?;

    let our_commit = our_branch
        .get()
        .peel_to_commit()
        .context("Failed to get default branch commit")?;
    let their_commit = their_branch
        .get()
        .peel_to_commit()
        .context("Failed to get worktree branch commit")?;

    let ancestor = repository
        .find_commit(
            repository
                .merge_base(our_commit.id(), their_commit.id())
                .context("Failed to find merge base")?,
        )
        .context("Failed to find ancestor commit")?;

    let merge_options = git2::MergeOptions::new();
    let index = repository
        .merge_commits(&ancestor, &their_commit, Some(&merge_options))
        .context("Failed to perform merge analysis")?;

    if index.has_conflicts() {
        let conflicts: Vec<String> = index
            .conflicts()
            .context("Failed to get conflicts")?
            .filter_map(Result::ok)
            .filter_map(|conflict| {
                conflict
                    .our
                    .or(conflict.their)
                    .or(conflict.ancestor)
                    .and_then(|entry| String::from_utf8(entry.path).ok())
            })
            .collect();
        return Ok(Some(conflicts));
    }

    Ok(None)
}

/// Get the repository root for a given path
///
/// # Errors
///
/// Returns an error if the path is not within a git repository.
pub fn find_repository_root(path: &Path) -> Result<PathBuf> {
    let repository = Repository::discover(path).context("Not a git repository")?;

    let workdir = repository
        .workdir()
        .ok_or_else(|| anyhow!("Bare repository has no working directory"))?;

    Ok(workdir.to_path_buf())
}

/// List all existing worktrees/workspaces in the repository
///
/// # Errors
///
/// Returns an error if the repository cannot be opened or worktrees cannot be listed.
pub fn list_worktrees(repository_path: &Path, vcs_command: &VcsCommand) -> Result<Vec<Worktree>> {
    match vcs_command {
        VcsCommand::Git => list_git_worktrees(repository_path),
        VcsCommand::Jujutsu => list_jj_workspaces(repository_path),
    }
}

fn list_git_worktrees(repository_path: &Path) -> Result<Vec<Worktree>> {
    let repository = Repository::open(repository_path).context("Failed to open git repository")?;

    let worktree_list = repository.worktrees().context("Failed to list worktrees")?;

    let mut worktrees = Vec::new();

    for worktree_name in worktree_list.iter().flatten() {
        let Ok(worktree) = repository.find_worktree(worktree_name) else {
            continue;
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

fn list_jj_workspaces(repository_path: &Path) -> Result<Vec<Worktree>> {
    let output = std::process::Command::new("jj")
        .arg("workspace")
        .arg("list")
        .current_dir(repository_path)
        .output()
        .context("Failed to execute jj workspace list")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("jj workspace list failed: {error_message}"));
    }

    let output_text = String::from_utf8_lossy(&output.stdout);
    let mut workspaces = Vec::new();

    for line in output_text.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            continue;
        }

        let workspace_name = parts[0].trim();
        let workspace_path_str = parts[1].trim();
        let workspace_path = PathBuf::from(workspace_path_str);

        workspaces.push(Worktree {
            path: workspace_path,
            branch_name: workspace_name.to_string(),
            is_bare: false,
            is_detached: false,
        });
    }

    Ok(workspaces)
}

/// Generate a valid git branch name from a task title
/// Example: "Implement Worktree Support" -> "chloe/implement-worktree-support"
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

    format!("chloe/{truncated_slug}")
}

/// Create a new worktree/workspace for a task
/// Returns `WorktreeInfo` with the branch/workspace name and path
///
/// # Errors
///
/// Returns an error if the repository cannot be opened or worktree/workspace creation fails.
pub fn create_worktree(
    repository_path: &Path,
    task_title: &str,
    task_id: &Uuid,
    vcs_command: &VcsCommand,
) -> Result<WorktreeInfo> {
    match vcs_command {
        VcsCommand::Git => create_git_worktree(repository_path, task_title, task_id),
        VcsCommand::Jujutsu => create_jj_workspace(repository_path, task_title, task_id),
    }
}

fn create_git_worktree(
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
    let worktree_path = repository_path.join(format!(".chloe/worktrees/{worktree_dir_name}"));

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

    generate_claude_settings(&worktree_path, task_id)?;

    Ok(worktree_info)
}

fn create_jj_workspace(
    repository_path: &Path,
    task_title: &str,
    task_id: &Uuid,
) -> Result<WorktreeInfo> {
    let workspace_name = generate_workspace_name(task_title, task_id);
    let workspace_path = repository_path.join(format!(".chloe/workspaces/{workspace_name}"));

    let workspaces_parent = repository_path.join(".chloe/workspaces");
    fs::create_dir_all(&workspaces_parent)
        .context("Failed to create .chloe/workspaces directory")?;

    let output = std::process::Command::new("jj")
        .arg("workspace")
        .arg("add")
        .arg("--name")
        .arg(&workspace_name)
        .arg(&workspace_path)
        .current_dir(repository_path)
        .output()
        .context("Failed to execute jj workspace add")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("jj workspace add failed: {error_message}"));
    }

    let worktree_info = WorktreeInfo::new(workspace_name, workspace_path.clone());

    generate_claude_settings(&workspace_path, task_id)?;

    Ok(worktree_info)
}

fn generate_workspace_name(task_title: &str, task_id: &Uuid) -> String {
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

    let short_id = &task_id.to_string()[..8];
    format!("chloe-{truncated_slug}-{short_id}")
}

/// Merge a worktree branch into main (legacy function, calls `merge_worktree` with "main")
/// Returns `MergeResult` indicating success or conflicts
///
/// # Errors
///
/// Returns an error if checkout, stash, or merge operations fail.
pub fn merge_worktree_to_main(
    repository_path: &Path,
    worktree_info: &WorktreeInfo,
) -> Result<MergeResult> {
    merge_worktree(repository_path, worktree_info, "main")
}

/// Merge a worktree branch into a target branch
/// Returns `MergeResult` indicating success or conflicts
///
/// # Errors
///
/// Returns an error if checkout, stash, or merge operations fail.
pub fn merge_worktree(
    repository_path: &Path,
    worktree_info: &WorktreeInfo,
    target_branch: &str,
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
        .arg(target_branch)
        .current_dir(repository_path)
        .output()
        .context("Failed to checkout target branch")?;

    if !checkout_output.status.success() {
        let error_message = String::from_utf8_lossy(&checkout_output.stderr);
        return Err(anyhow!(
            "Git checkout {target_branch} failed: {error_message}"
        ));
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

/// Delete a worktree/workspace (cleanup)
///
/// # Errors
///
/// Returns an error if the worktree/workspace or branch cannot be deleted.
pub fn delete_worktree(
    repository_path: &Path,
    worktree_info: &WorktreeInfo,
    vcs_command: &VcsCommand,
) -> Result<()> {
    match vcs_command {
        VcsCommand::Git => delete_git_worktree(repository_path, worktree_info),
        VcsCommand::Jujutsu => delete_jj_workspace(repository_path, worktree_info),
    }
}

fn delete_git_worktree(repository_path: &Path, worktree_info: &WorktreeInfo) -> Result<()> {
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

fn delete_jj_workspace(repository_path: &Path, worktree_info: &WorktreeInfo) -> Result<()> {
    const DEFAULT_WORKSPACE_NAME: &str = "default";

    let is_default_workspace = worktree_info.branch_name == DEFAULT_WORKSPACE_NAME;
    if is_default_workspace {
        return Err(anyhow!(
            "Cannot delete the default workspace. The default workspace is the main workspace and must be preserved."
        ));
    }

    let output = std::process::Command::new("jj")
        .arg("workspace")
        .arg("forget")
        .arg(&worktree_info.branch_name)
        .current_dir(repository_path)
        .output()
        .context("Failed to execute jj workspace forget")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("jj workspace forget failed: {error_message}"));
    }

    if worktree_info.worktree_path.exists() {
        fs::remove_dir_all(&worktree_info.worktree_path)
            .context("Failed to remove workspace directory")?;
    }

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
        assert_eq!(result, "chloe/implement-worktree-support");
    }

    #[test]
    fn test_generate_branch_name_with_special_chars() {
        let result = generate_branch_name("Fix bug #123: API timeout");
        assert_eq!(result, "chloe/fix-bug-123-api-timeout");
    }

    #[test]
    fn test_generate_branch_name_truncation() {
        let long_title = "A".repeat(100);
        let result = generate_branch_name(&long_title);
        assert!(result.len() <= 57);
    }

    #[test]
    fn test_generate_branch_name_with_consecutive_dashes() {
        let result = generate_branch_name("Multiple   spaces   and---dashes");
        assert_eq!(result, "chloe/multiple-spaces-and-dashes");
    }
}
