use common::models::types::OAuthClientConfigResponse;
use reqwasm::http::Request;

pub async fn api_get_auth_config() -> OAuthClientConfigResponse {
    let response = Request::get("/oauth/config").send().await;

    if response.is_ok() {
        let response = response.unwrap();
        let response = response.json::<OAuthClientConfigResponse>().await;
        if response.is_ok() {
            let response = response.unwrap();
            return response;
        }
    }

    // Return an invalid config, which will cause the app to display an error
    OAuthClientConfigResponse {
        client_id: String::new(),
        auth_url: String::new(),
        token_url: String::new(),
    }
}

#[allow(dead_code)]
pub async fn api_request_logout() {
    todo!()
}

#[allow(dead_code)]
pub async fn api_post_classes() {
    todo!()
}

#[allow(dead_code)]
pub async fn api_post_students() {
    todo!()
}

#[allow(dead_code)]
pub async fn api_post_teams() {
    todo!()
}

#[allow(dead_code)]
pub async fn api_get_sprint() {
    todo!()
}
