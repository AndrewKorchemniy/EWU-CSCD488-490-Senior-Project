use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Used for "/api/oauth/config" endpoint.
#[derive(Deserialize, Serialize)]
pub struct OAuthClientConfigResponse {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
}

/// Used for "/api/sprints" endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct SprintResponse {
    pub id: u8,
    pub due_date: NaiveDate,
    pub is_individual_report_submitted: bool,
    pub is_team_report_submitted: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SprintsResponse {
    pub sprints: Vec<SprintResponse>,
}

/// Used for "api/requirements" endpoint.
#[derive(Deserialize, Serialize)]
pub struct RequirementResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct RequirementsResponse {
    pub requirements: Vec<RequirementResponse>,
}

/// Used for "api/submit/team" endpoint.
#[derive(Deserialize, Serialize)]
pub struct TeamResponse {
    pub understand_easy: String,
    pub understand_hard: String,
    pub approach_easy: String,
    pub approach_hard: String,
    pub solve_easy: String,
    pub solve_hard: String,
    pub evaluate_easy: String,
    pub evaluate_hard: String,
    pub completion_percent: String,
    pub pace_succeed: String,
    pub client_meeting: String,
    pub issues_comments: String,
}

impl TeamResponse {
    pub fn validate(&self) -> bool {
        let mut valid = true;
        let min_size_three = |value: &str| -> bool { value.trim().len() >= 3 };

        valid = min_size_three(&self.understand_easy) && valid;
        valid = min_size_three(&self.understand_hard) && valid;
        valid = min_size_three(&self.approach_easy) && valid;
        valid = min_size_three(&self.approach_hard) && valid;
        valid = min_size_three(&self.solve_easy) && valid;
        valid = min_size_three(&self.solve_hard) && valid;
        valid = min_size_three(&self.evaluate_easy) && valid;
        valid = min_size_three(&self.evaluate_hard) && valid;
        valid = min_size_three(&self.pace_succeed) && valid;
        valid = min_size_three(&self.client_meeting) && valid;
        valid
    }
}
