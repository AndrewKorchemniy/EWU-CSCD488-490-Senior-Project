use chrono::NaiveDate;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

// TODO: use an environment variable instead of a file for the base API URI
const BASE_API_URI: &str = include_str!("base_api_uri.txt");

// TODO: move types to common
#[derive(Deserialize)]
pub struct OAuthClientConfigResponse {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
}

pub async fn api_get_auth_config() -> OAuthClientConfigResponse {
    let response = Request::get(&format!("{BASE_API_URI}/oauth/config",))
        .send()
        .await;

    if response.is_ok() {
        let response = response.unwrap();
        let response = response.json::<OAuthClientConfigResponse>().await;
        if response.is_ok() {
            let response = response.unwrap();
            return response;
        }
    }

    // Return invalid config, which will cause the oauth components to display an error
    OAuthClientConfigResponse {
        client_id: String::new(),
        auth_url: String::new(),
        token_url: String::new(),
    }
}

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

pub async fn api_get_sprints(token: &str) -> Result<SprintsResponse, reqwasm::Error> {
    let response = Request::get(&format!("{BASE_API_URI}/api/sprints",))
        .header("Authorization", token)
        .send()
        .await;

    match response {
        Ok(response) => {
            let response = response.json::<SprintsResponse>().await;
            match response {
                Ok(response) => Ok(response),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

#[allow(dead_code)]
pub async fn api_post_team_report() {
    todo!()
}

#[allow(dead_code)]
pub async fn api_get_individual_report() {
    todo!()
}

#[allow(dead_code)]
pub async fn api_post_individual_report() {
    todo!()
}

#[allow(dead_code)]
pub async fn api_get_requirements() {
    todo!()
}

#[allow(dead_code)]
pub async fn api_post_requirements() {
    todo!()
}

#[allow(dead_code)]
pub async fn api_request_logout() {
    todo!()
}
