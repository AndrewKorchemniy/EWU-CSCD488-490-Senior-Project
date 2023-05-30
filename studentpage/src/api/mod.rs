use reqwasm::http::Request;
use reqwasm::Error;
use common::models::types::*;
use serde_json::json;

/// Gets the server's OAuth configuration.
/// See APIDOC for more information.
pub async fn api_get_auth_config() -> OAuthClientConfigResponse {
    let response = Request::get("/api/oauth/config").send().await;

    if response.is_ok() {
        let response = response.unwrap();
        let response = response.json::<OAuthClientConfigResponse>().await;
        if response.is_ok() {
            let response = response.unwrap();
            return response;
        }
    }

    // Return an invalid config, which will cause the oauth components to display an error
    OAuthClientConfigResponse {
        client_id: String::new(),
        auth_url: String::new(),
        token_url: String::new(),
    }
}

/// Gets a list of all sprints in the database for a student.
/// See APIDOC for more information.
pub async fn api_get_sprints(token: &str) -> Result<SprintsResponse, Error> {
    let response = Request::get("/api/sprints")
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

/// Gets the project requirements of a team from the database.
/// See APIDOC for more information.
pub async fn api_get_requirements(token: &str) -> Result<RequirementsResponse, Error> {
    let _response = Request::get("/api/requirements")
        .header("Authorization", token)
        .send()
        .await;

    // match response {
    //     Ok(response) => {
    //         let response = response.json::<RequirementsResponse>().await;
    //         match response {
    //             Ok(response) => Ok(response),
    //             Err(err) => Err(err),
    //         }
    //     }
    //     Err(err) => Err(err),
    // }

    // TODO: Remove this once the api is ready.
    let testing = RequirementResponse {
        id: 1,
        title: String::from("Example One"),
        description: String::from("Example description"),
    };
    let testing2 = RequirementResponse {
        id: 2,
        title: String::from("Example Two"),
        description: String::from("Example description"),
    };

    let testing3 = RequirementsResponse {
        requirements: vec![testing, testing2],
    };

    return Ok(testing3);
}

/// Posts a new team project requirement to the database.
/// See APIDOC for more information.
#[allow(dead_code)]
pub async fn api_post_new_requirement(title: String, description: String, token: &str) {
    let body = json!({
        "title": title,
        "description": description,
    });
    let _response = Request::post("/api/submit/new_requirement")
        .header("Authorization", token)
        .body(body.to_string())
        .send()
        .await;
    // TODO: Handle errors gracefully by displaying an error message to the user.
}

/// Request a requirement to be deleted from the database.
/// Returns the new list of requirements.
/// See APIDOC for more information.
pub async fn api_post_delete_requirement(
    id: i32,
    token: &str,
) -> Result<RequirementsResponse, Error> {
    let body = json!({
        "id": id,
    });
    let response = Request::post("/api/submit/delete_requirement")
        .header("Authorization", token)
        .body(body.to_string())
        .send()
        .await;

    match response {
        Ok(response) => {
            let response = response.json::<RequirementsResponse>().await;
            match response {
                Ok(response) => Ok(response),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
    // TODO: Handle errors gracefully by displaying an error message to the user.
}

#[allow(dead_code)]
pub async fn api_post_logout() {
    todo!()
}
