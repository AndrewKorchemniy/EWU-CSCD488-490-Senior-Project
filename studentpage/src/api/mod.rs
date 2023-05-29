use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

const BASE_API_URI: &str = include_str!("base_api_uri.txt");

#[derive(Deserialize, Serialize)]
pub struct OAuthClientConfigResponse {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
}

// TODO: remove since it does not work
pub async fn api_get_auth_config() -> OAuthClientConfigResponse {
    let response = Request::get(&format!("{BASE_API_URI}/oauth/config",))
        .send()
        .await
        .unwrap()
        .json::<OAuthClientConfigResponse>()
        .await
        .unwrap();

    OAuthClientConfigResponse {
        client_id: response.client_id,
        auth_url: response.auth_url,
        token_url: response.token_url,
    }
}

#[allow(dead_code)]
pub async fn api_get_sprints() {
    todo!()
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
