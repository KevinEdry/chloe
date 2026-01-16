use crate::events::AppEvent;
use crate::providers;
use crate::types::{AgentProvider, Result};
use serde::{Deserialize, Serialize};
use std::thread;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedTask {
    pub title: String,
    pub description: String,
    pub task_type: String,
}

pub fn spawn_classification(
    raw_input: String,
    task_id: Uuid,
    provider: AgentProvider,
    event_sender: mpsc::UnboundedSender<AppEvent>,
) {
    thread::spawn(move || {
        let result = classify_with_provider(&raw_input, provider);
        let event_result = result.map_err(|error| error.to_string());
        let _ = event_sender.send(AppEvent::ClassificationCompleted {
            task_id,
            result: event_result,
        });
    });
}

fn classify_with_provider(raw_input: &str, provider: AgentProvider) -> Result<ClassifiedTask> {
    let prompt = format!(
        r#"Classify this task description and respond with ONLY valid JSON (no markdown, no explanation):

User input: "{raw_input}"

Output format:
{{
  "title": "concise title (max 60 chars)",
  "description": "detailed description",
  "task_type": "feature|bug|task|chore"
}}

Rules:
- feature: new functionality
- bug: fix broken behavior
- task: general work item
- chore: maintenance, refactoring, docs

Output JSON only:"#
    );

    let spec = providers::get_spec(provider);
    let command = spec.build_oneshot_command(&prompt);

    let mut process_command = std::process::Command::new(&command.program);
    process_command.args(&command.arguments);
    for (key, value) in &command.environment {
        process_command.env(key, value);
    }

    let output = process_command.output().map_err(|error| {
        crate::types::AppError::Config(format!(
            "Failed to run {} CLI: {error}",
            provider.display_name()
        ))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(crate::types::AppError::Config(format!(
            "{} CLI failed: {stderr}",
            provider.display_name()
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let json_string = extract_json(&stdout)?;

    let classified: ClassifiedTask = serde_json::from_str(&json_string).map_err(|error| {
        crate::types::AppError::Config(format!("Failed to parse JSON: {error}"))
    })?;

    Ok(classified)
}

fn extract_json(text: &str) -> Result<String> {
    if let Some(start) = text.find('{')
        && let Some(end) = text.rfind('}')
    {
        return Ok(text[start..=end].to_string());
    }

    Err(crate::types::AppError::Config(
        "No JSON found in AI output".to_string(),
    ))
}
