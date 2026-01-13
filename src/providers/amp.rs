use super::{GeneratedFile, OneShotPromptStyle, PromptStyle, ProviderSpec};
use std::path::Path;
use uuid::Uuid;

pub static SPEC: ProviderSpec = ProviderSpec {
    command: "amp",
    prompt_style: PromptStyle::Direct,
    oneshot_style: OneShotPromptStyle::Direct,
    generate_files,
};

fn generate_files(task_id: Uuid, working_directory: &Path) -> Vec<GeneratedFile> {
    let notify_start = format!("chloe notify start --worktree-id {task_id}");
    let notify_end = format!("chloe notify end --worktree-id {task_id}");

    let settings = serde_json::json!({
        "amp.hooks": [
            {
                "compatibilityDate": "2025-05-13",
                "id": "chloe-start",
                "on": {
                    "event": "tool:pre-execute"
                },
                "action": {
                    "type": "run-command",
                    "command": notify_start
                }
            },
            {
                "compatibilityDate": "2025-05-13",
                "id": "chloe-end",
                "on": {
                    "event": "tool:post-execute"
                },
                "action": {
                    "type": "run-command",
                    "command": notify_end
                }
            }
        ]
    });

    vec![GeneratedFile {
        path: working_directory.join(".amp").join("settings.json"),
        content: serde_json::to_string_pretty(&settings).unwrap_or_default(),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_values() {
        assert_eq!(SPEC.command, "amp");
    }

    #[test]
    fn test_generate_files_creates_settings() {
        let task_id = Uuid::new_v4();
        let working_dir = Path::new("/tmp/test");

        let files = generate_files(task_id, working_dir);

        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].path,
            working_dir.join(".amp").join("settings.json")
        );
        assert!(files[0].content.contains("amp.hooks"));
        assert!(files[0].content.contains(&task_id.to_string()));
    }
}
