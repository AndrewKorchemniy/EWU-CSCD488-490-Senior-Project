use chrono::{Days, Local};
use common::models::types::*;
use reqwasm::http::Request;
use reqwasm::Error;
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
    let _response = Request::get("/api/sprints")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    // match response {
    //     Ok(response) => {
    //         let response = response.json::<SprintsResponse>().await;
    //         match response {
    //             Ok(response) => Ok(response),
    //             Err(err) => Err(err),
    //         }
    //     }
    //     Err(err) => Err(err),
    // }

    // TODO: Uncomment the above code block and
    // remove the following once the api is ready.
    let sprint_1 = SprintResponse {
        id: 1,
        due_date: Local::now()
            .naive_local()
            .checked_sub_days(Days::new(14))
            .unwrap()
            .date(),
        is_individual_report_submitted: true,
        is_team_report_submitted: true,
    };

    let sprint_2 = SprintResponse {
        id: 2,
        due_date: Local::now()
            .naive_local()
            .checked_sub_days(Days::new(7))
            .unwrap()
            .date(),
        is_individual_report_submitted: false,
        is_team_report_submitted: false,
    };

    let sprint_3 = SprintResponse {
        id: 3,
        due_date: Local::now().naive_local().date(),
        is_individual_report_submitted: false,
        is_team_report_submitted: false,
    };

    let sprint_4 = SprintResponse {
        id: 4,
        due_date: Local::now()
            .naive_local()
            .checked_add_days(Days::new(7))
            .unwrap()
            .date(),
        is_individual_report_submitted: false,
        is_team_report_submitted: false,
    };

    let sprints = SprintsResponse {
        sprints: vec![sprint_1, sprint_2, sprint_3, sprint_4],
    };

    Ok(sprints)
}

/// Posts a team report to the database for a team.
/// See APIDOC for more information.
pub async fn api_post_team_report(token: &str, body: TeamResponse) -> Result<String, Error> {
    let response = Request::post("/api/submit/team")
        .header("Authorization", &format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await;

    match response {
        Ok(response) => {
            let response = response.text().await;
            match response {
                Ok(response) => Ok(response),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
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
        .header("Authorization", &format!("Bearer {}", token))
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

    // TODO: Uncomment the above code block and
    // remove the following once the api is ready.
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
        .header("Authorization", &format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await;
    // TODO: Handle errors gracefully by displaying an error message to the user.
}

/// Request a requirement to be deleted from the database.
/// See APIDOC for more information.
pub async fn api_post_delete_requirement(id: i32, token: &str) -> Result<String, Error> {
    let body = json!({
        "id": id,
    });
    let response = Request::post("/api/submit/delete_requirement")
        .header("Authorization", token)
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await;

    match response {
        Ok(response) => {
            let response = response.text().await;
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
