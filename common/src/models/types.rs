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
