use crate::roadmap::{RoadmapItem, RoadmapPriority, RoadmapStatus};
use crate::types::Result;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{Receiver, channel};
use std::thread;

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

#[derive(Debug)]
pub struct RoadmapGenerationRequest {
    receiver: Receiver<Result<GeneratedRoadmap>>,
}

impl Clone for RoadmapGenerationRequest {
    fn clone(&self) -> Self {
        panic!("RoadmapGenerationRequest cannot be cloned");
    }
}

impl RoadmapGenerationRequest {
    pub fn spawn(project_path: String) -> Self {
        let (sender, receiver) = channel();

        thread::spawn(move || {
            let result = Self::generate_with_claude(&project_path);
            let _ = sender.send(result);
        });

        Self { receiver }
    }

    fn generate_with_claude(project_path: &str) -> Result<GeneratedRoadmap> {
        let prompt = Self::build_roadmap_prompt(project_path);

        let output = std::process::Command::new("claude")
            .arg(&prompt)
            .output()
            .map_err(|e| {
                crate::types::AppError::Config(format!("Failed to run claude CLI: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::types::AppError::Config(format!(
                "claude CLI failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        let json_str = Self::extract_json(&stdout)?;

        let roadmap: GeneratedRoadmap = serde_json::from_str(&json_str)
            .map_err(|e| crate::types::AppError::Config(format!("Failed to parse JSON: {}", e)))?;

        Ok(roadmap)
    }

    fn build_roadmap_prompt(project_path: &str) -> String {
        format!(
            r#"You are an AI Product Strategist analyzing a software project to generate a strategic feature roadmap.

# Your Task

Analyze the project at: {}

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

Output JSON only:"#,
            project_path
        )
    }

    fn extract_json(text: &str) -> Result<String> {
        if let Some(start) = text.find('{') {
            if let Some(end) = text.rfind('}') {
                return Ok(text[start..=end].to_string());
            }
        }

        Err(crate::types::AppError::Config(
            "No JSON found in claude output".to_string(),
        ))
    }

    pub fn try_recv(&self) -> Option<Result<GeneratedRoadmap>> {
        self.receiver.try_recv().ok()
    }
}

impl GeneratedRoadmapItem {
    pub fn to_roadmap_item(self) -> RoadmapItem {
        let priority = match self.priority.to_lowercase().as_str() {
            "high" => RoadmapPriority::High,
            "low" => RoadmapPriority::Low,
            _ => RoadmapPriority::Medium,
        };

        let mut item = RoadmapItem::new(
            self.title,
            self.description,
            self.rationale,
            priority,
        );

        item.user_stories = self.user_stories;
        item.acceptance_criteria = self.acceptance_criteria;
        item.status = RoadmapStatus::Planned;

        item
    }
}
