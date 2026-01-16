use super::{RoadmapItem, RoadmapPriority, RoadmapStatus};
use crate::events::AppEvent;
use crate::types::Result;
use serde::{Deserialize, Serialize};
use std::thread;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedRoadmapItem {
    pub title: String,
    pub description: String,
    pub rationale: String,
    pub user_stories: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub priority: String,
    pub complexity: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedRoadmap {
    pub project_name: String,
    pub target_audience: String,
    pub items: Vec<GeneratedRoadmapItem>,
}

pub fn spawn_roadmap_generation(
    project_path: String,
    event_sender: mpsc::UnboundedSender<AppEvent>,
) {
    thread::spawn(move || {
        let result = generate_with_claude(&project_path);
        let event_result = result.map_err(|error| error.to_string());
        let _ = event_sender.send(AppEvent::RoadmapGenerationCompleted {
            result: event_result,
        });
    });
}

fn generate_with_claude(project_path: &str) -> Result<GeneratedRoadmap> {
    let prompt = build_roadmap_prompt(project_path);

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

    let json_string = extract_json(&stdout)?;

    let roadmap: GeneratedRoadmap = serde_json::from_str(&json_string).map_err(|error| {
        crate::types::AppError::Config(format!("Failed to parse JSON: {error}"))
    })?;

    Ok(roadmap)
}

fn build_roadmap_prompt(project_path: &str) -> String {
    format!(
        r#"You are an AI Product Strategist analyzing a software project to generate a strategic feature roadmap.

# Your Task

Analyze the project at: {project_path}

Perform these phases autonomously:

## Phase 1: Project Discovery
Examine the codebase to understand:
- Project purpose and type (analyze README, package files, main source files)
- Current tech stack and architecture
- Target audience (infer from project type and problem domain)
- Current maturity level (prototype/MVP/established)
- Existing features and capabilities

## Phase 2: Feature Brainstorming
Generate 5-10 strategic features that:
- Address user pain points
- Fill gaps in current functionality
- Provide competitive advantages
- Improve user experience
- Enhance technical capabilities
- Reduce technical debt

## Phase 3: Prioritization
For each feature, assess:
- Priority: High/Medium/Low
- Complexity: Low/Medium/High
- Impact: Low/Medium/High

Focus on features that provide maximum value with reasonable effort.

## Phase 4: Create Detailed Roadmap

Output ONLY valid JSON in this exact format (no markdown, no explanation):

{{
  "project_name": "Project Name",
  "target_audience": "Primary user persona description",
  "items": [
    {{
      "title": "Feature Title (concise, max 60 chars)",
      "description": "Detailed feature description explaining what it does and why it matters",
      "rationale": "Strategic reason why this feature is important for users and the product",
      "user_stories": [
        "As a [user type], I want [goal] so that [benefit]",
        "As a [user type], I want [goal] so that [benefit]"
      ],
      "acceptance_criteria": [
        "Specific, testable criterion 1",
        "Specific, testable criterion 2",
        "Specific, testable criterion 3"
      ],
      "priority": "High|Medium|Low",
      "complexity": "Low|Medium|High",
      "impact": "High|Medium|Low"
    }}
  ]
}}

Requirements:
- Minimum 5 features, maximum 10
- Each feature must have 2-3 user stories
- Each feature must have 3-5 acceptance criteria
- Prioritize features that provide the most user value
- Be specific and actionable in descriptions
- Consider both short-term wins and long-term strategic features

Output JSON only:"#
    )
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

impl GeneratedRoadmapItem {
    #[must_use]
    pub fn into_roadmap_item(self) -> RoadmapItem {
        let priority = match self.priority.to_lowercase().as_str() {
            "high" => RoadmapPriority::High,
            "low" => RoadmapPriority::Low,
            _ => RoadmapPriority::Medium,
        };

        let mut item = RoadmapItem::new(self.title, self.description, self.rationale, priority);

        item.user_stories = self.user_stories;
        item.acceptance_criteria = self.acceptance_criteria;
        item.status = RoadmapStatus::Planned;

        item
    }
}
