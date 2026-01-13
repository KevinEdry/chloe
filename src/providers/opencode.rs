use super::{GeneratedFile, OneShotPromptStyle, PromptStyle, ProviderSpec};
use std::path::Path;
use uuid::Uuid;

pub static SPEC: ProviderSpec = ProviderSpec {
    command: "opencode",
    prompt_style: PromptStyle::Flag("--prompt"),
    oneshot_style: OneShotPromptStyle::Subcommand("run"),
    generate_files,
};

fn generate_files(task_id: Uuid, working_directory: &Path) -> Vec<GeneratedFile> {
    let log_path = working_directory.join(".opencode").join("chloe-plugin.log");
    let log_path_str = log_path.to_string_lossy();

    let plugin_content = format!(
        r#"// Chloe integration plugin for OpenCode
import {{ appendFileSync }} from "fs";

const log = (msg) => appendFileSync("{log_path_str}", new Date().toISOString() + " " + msg + "\n");

export const ChloeNotifier = async () => {{
  const {{ spawn }} = await import("child_process");
  const taskId = "{task_id}";

  const notify = (type) => {{
    spawn("chloe", ["notify", type, "--worktree-id", taskId], {{
      detached: true,
      stdio: "ignore",
    }}).unref();
  }};

  log("Plugin loaded, taskId: " + taskId);

  return {{
    event: async ({{ event }}) => {{
      log("event: " + event.type);
      if (event.type === "permission.updated") {{
        notify("permission");
      }}
      if (event.type === "session.updated") {{
        notify("start");
      }}
      if (event.type === "session.idle") {{
        log("session.idle detected, notifying end");
        notify("end");
      }}
    }},
  }};
}};
"#
    );

    let settings = serde_json::json!({
        "permissions": {
            "allow": [
                "Read",
                "Edit",
                "Write",
                "MultiEdit",
                "Glob",
                "Grep",
                "Bash",
                "Skill"
            ]
        },
        "sandbox": {
            "enabled": true,
            "autoAllowBashIfSandboxed": true
        },
        "includeCoAuthoredBy": false,
        "gitAttribution": false
    });

    vec![
        GeneratedFile {
            path: working_directory
                .join(".opencode")
                .join("plugin")
                .join("chloe-hooks.js"),
            content: plugin_content,
        },
        GeneratedFile {
            path: working_directory
                .join(".opencode")
                .join("settings.local.json"),
            content: serde_json::to_string_pretty(&settings).unwrap_or_default(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_values() {
        assert_eq!(SPEC.command, "opencode");
    }

    #[test]
    fn test_generate_files_creates_plugin() {
        let task_id = Uuid::new_v4();
        let working_dir = Path::new("/tmp/test");

        let files = generate_files(task_id, working_dir);

        assert_eq!(files.len(), 2);
        assert_eq!(
            files[0].path,
            working_dir
                .join(".opencode")
                .join("plugin")
                .join("chloe-hooks.js")
        );
        assert!(files[0].content.contains("ChloeNotifier"));
        assert!(files[0].content.contains(&task_id.to_string()));

        assert_eq!(
            files[1].path,
            working_dir.join(".opencode").join("settings.local.json")
        );
        assert!(files[1].content.contains("permissions"));
        assert!(files[1].content.contains("Bash"));
    }
}
