use crate::types::Result;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{Receiver, channel};
use std::thread;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedTask {
    pub title: String,
    pub description: String,
    pub task_type: String,
}

#[derive(Debug)]
pub struct ClassificationRequest {
    receiver: Receiver<Result<ClassifiedTask>>,
}

impl Clone for ClassificationRequest {
    fn clone(&self) -> Self {
        panic!("ClassificationRequest cannot be cloned");
    }
}

impl ClassificationRequest {
    #[must_use]
    pub fn spawn(raw_input: String) -> Self {
        let (sender, receiver) = channel();

        thread::spawn(move || {
            let result = Self::classify_with_claude(&raw_input);
            let _ = sender.send(result);
        });

        Self { receiver }
    }

    fn classify_with_claude(raw_input: &str) -> Result<ClassifiedTask> {
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

        let output = std::process::Command::new("claude")
            .arg(&prompt)
            .output()
            .map_err(|error| {
                crate::types::AppError::Config(format!("Failed to run claude CLI: {error}"))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::types::AppError::Config(format!(
                "claude CLI failed: {stderr}"
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        let json_string = Self::extract_json(&stdout)?;

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
            "No JSON found in claude output".to_string(),
        ))
    }

    #[must_use]
    pub fn try_recv(&self) -> Option<Result<ClassifiedTask>> {
        self.receiver.try_recv().ok()
    }
}
