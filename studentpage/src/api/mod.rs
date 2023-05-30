use chrono::NaiveDate;
use reqwasm::http::Request;
use reqwasm::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

// TODO: use an environment variable instead of a file for the base API URI
const BASE_API_URI: &str = include_str!("base_api_uri.txt");

// TODO: move types to common
#[derive(Deserialize)]
pub struct OAuthClientConfigResponse {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
}

/// Gets the server's OAuth configuration.
/// See APIDOC for more information.
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

    // Return an invalid config, which will cause the oauth components to display an error
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

/// Gets a list of all sprints in the database for a student.
/// See APIDOC for more information.
pub async fn api_get_sprints(token: &str) -> Result<SprintsResponse, Error> {
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

/// Gets the project requirements of a team from the database.
/// See APIDOC for more information.
pub async fn api_get_requirements(token: &str) -> Result<RequirementsResponse, Error> {
    let _response = Request::get(&format!("{BASE_API_URI}/api/requirements",))
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
    let _response = Request::post(&format!("{BASE_API_URI}/api/submit/new_requirement",))
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
    let response = Request::post(&format!("{BASE_API_URI}/api/submit/delete_requirement",))
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
