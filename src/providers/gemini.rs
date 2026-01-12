use super::{GeneratedFile, PromptStyle, ProviderSpec};
use crate::types::AgentProvider;
use std::path::Path;
use uuid::Uuid;

pub static SPEC: ProviderSpec = ProviderSpec {
    provider: AgentProvider::Gemini,
    command: "gemini",
    prompt_style: PromptStyle::Direct,
    generate_files: generate_files,
};

fn generate_files(task_id: Uuid, working_directory: &Path) -> Vec<GeneratedFile> {
    let notify_start = format!("chloe notify start --worktree-id {task_id}");
    let notify_end = format!("chloe notify end --worktree-id {task_id}");

    let settings = serde_json::json!({
        "hooks": {
            "SessionStart": [
                {
                    "command": notify_start
                }
            ],
            "SessionEnd": [
                {
                    "command": notify_end
                }
            ]
        }
    });

    vec![GeneratedFile {
        path: working_directory.join(".gemini").join("settings.json"),
        content: serde_json::to_string_pretty(&settings).unwrap_or_default(),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_values() {
        assert_eq!(SPEC.provider, AgentProvider::Gemini);
        assert_eq!(SPEC.command, "gemini");
    }

    #[test]
    fn test_generate_files_creates_settings() {
        let task_id = Uuid::new_v4();
        let working_dir = Path::new("/tmp/test");

        let files = generate_files(task_id, working_dir);

        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].path,
            working_dir.join(".gemini").join("settings.json")
        );
        assert!(files[0].content.contains("hooks"));
        assert!(files[0].content.contains(&task_id.to_string()));
    }
}
