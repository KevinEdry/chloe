use super::state::{Worktree, WorktreeInfo};
use anyhow::{anyhow, Context, Result};
use git2::{BranchType, Repository};
use std::path::{Path, PathBuf};
use uuid::Uuid;

const MAX_SLUG_LENGTH: usize = 50;

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

    let worktree_list = repository
        .worktrees()
        .context("Failed to list worktrees")?;

    let mut worktrees = Vec::new();

    for worktree_name in worktree_list.iter().flatten() {
        let worktree = match repository.find_worktree(worktree_name) {
            Ok(wt) => wt,
            Err(_) => continue,
        };

        let path = worktree.path().to_path_buf();

        let branch_name =
            extract_branch_name_from_worktree(&repository, &path).unwrap_or_else(|| "(detached)".to_string());

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
pub fn create_worktree(repository_path: &Path, task_title: &str, task_id: &Uuid) -> Result<WorktreeInfo> {
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

    Ok(WorktreeInfo::new(final_branch_name, worktree_path))
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

fn extract_branch_name_from_worktree(_repository: &Repository, worktree_path: &Path) -> Option<String> {
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
