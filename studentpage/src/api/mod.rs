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
    // let response = Request::get(&format!("{BASE_API_URI}/oauth/config",))
    //     .send()
    //     .await
    //     .unwrap()
    //     .json::<OAuthClientConfigResponse>()
    //     .await
    //     .unwrap();
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

    OAuthClientConfigResponse {
        client_id: String::new(),
        auth_url: String::new(),
        token_url: String::new(),
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
