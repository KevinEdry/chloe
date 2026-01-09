use serde::{Deserialize, Serialize};
use std::time::Instant;

pub const REFRESH_INTERVAL_SECONDS: u64 = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub author: String,
    pub branch: String,
    pub base_branch: String,
    pub state: PullRequestStatusState,
    pub is_draft: bool,
    pub additions: u64,
    pub deletions: u64,
    pub url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PullRequestStatusState {
    Open,
    Closed,
    Merged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestsState {
    pub pull_requests: Vec<PullRequest>,
    pub selected_index: Option<usize>,
    pub mode: PullRequestsMode,
    #[serde(skip)]
    pub error_message: Option<String>,
    #[serde(skip)]
    pub(super) last_refresh: Option<Instant>,
    #[serde(skip)]
    pub(super) needs_initial_refresh: bool,
    #[serde(skip)]
    pub is_loading: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PullRequestsMode {
    #[default]
    Normal,
    Viewing,
}

impl PullRequestsState {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pull_requests: Vec::new(),
            selected_index: None,
            mode: PullRequestsMode::Normal,
            error_message: None,
            last_refresh: None,
            needs_initial_refresh: true,
            is_loading: false,
        }
    }

    pub fn select_next(&mut self) {
        if self.pull_requests.is_empty() {
            return;
        }

        self.selected_index = match self.selected_index {
            Some(index) => Some((index + 1).min(self.pull_requests.len() - 1)),
            None => Some(0),
        };
    }

    pub fn select_previous(&mut self) {
        if self.pull_requests.is_empty() {
            return;
        }

        self.selected_index = self
            .selected_index
            .map_or(Some(0), |index| Some(index.saturating_sub(1)));
    }

    pub const fn select_first(&mut self) {
        if !self.pull_requests.is_empty() {
            self.selected_index = Some(0);
        }
    }

    pub const fn select_last(&mut self) {
        if !self.pull_requests.is_empty() {
            self.selected_index = Some(self.pull_requests.len() - 1);
        }
    }

    #[must_use]
    pub fn get_selected_pull_request(&self) -> Option<&PullRequest> {
        self.selected_index
            .and_then(|index| self.pull_requests.get(index))
    }

    pub const fn mark_needs_refresh(&mut self) {
        self.needs_initial_refresh = true;
    }

    #[must_use]
    pub fn should_refresh(&self) -> bool {
        if self.needs_initial_refresh {
            return true;
        }

        let Some(last_refresh) = self.last_refresh else {
            return true;
        };

        last_refresh.elapsed().as_secs() >= REFRESH_INTERVAL_SECONDS
    }

    pub fn mark_refreshed(&mut self) {
        self.last_refresh = Some(Instant::now());
        self.needs_initial_refresh = false;
        self.is_loading = false;
    }

    pub fn set_pull_requests(&mut self, pull_requests: Vec<PullRequest>) {
        self.pull_requests = pull_requests;
        self.error_message = None;

        if self.selected_index.is_none() && !self.pull_requests.is_empty() {
            self.selected_index = Some(0);
        }

        if let Some(index) = self.selected_index
            && index >= self.pull_requests.len()
        {
            self.selected_index = if self.pull_requests.is_empty() {
                None
            } else {
                Some(self.pull_requests.len() - 1)
            };
        }
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
        self.is_loading = false;
    }
}

impl Default for PullRequestsState {
    fn default() -> Self {
        Self::new()
    }
}
