use crate::api::token;
use crate::email;
use actix_web::web;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};
use common::models::status_report;
use common::models::types::TeamResponse;
use config::Config;
use database::repository::db_connector::Database;
use log::debug;
use serde::{Deserialize, Serialize};

/// TODO: remove using for dev or require to be admin
///
/// TODO: if keep than move to common
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailInfo {
    pub email: String,
    pub name: String,
}

//https://actix.rs/docs/databases/

/// # get sprites from db
///
/// > TODO: finish
#[get("/sprints")]
pub async fn get_sprints(_data: Data<(Database, Config, Config)>) -> HttpResponse {
    HttpResponse::NotImplemented().body("Not Ready")
}

/// # submit a team report to the db
///
/// > TODO: finish
#[post("/submit/team")]
pub async fn post_team(
    data: Data<(Database, Config, Config)>,
    body: web::Json<TeamResponse>,
) -> HttpResponse {
    let completion_value;
    if body.completion_percent.is_empty() {
        debug!("Bad Request: completion_percent is empty");
        completion_value = 0;
    } else {
        completion_value = match body.completion_percent.parse() {
            Ok(value) => value,
            Err(error_message) => {
                debug!(
                    "Bad Request: completion_percent is unable to be parse ({}) ({})",
                    body.completion_percent, error_message
                );
                return HttpResponse::BadRequest().body("completion_percent is unable to be parse");
            }
        }
    }
    let update_value = status_report::TeamReport {
        teams: "eagles".to_string(), // TODO: fix
        sprint_num: 1,
        understand_easiest: body.understand_easy.clone(),
        understand_hardest: body.understand_hard.clone(),
        approach_easiest: body.approach_easy.clone(),
        approach_hardest: body.approach_hard.clone(),
        solve_easiest: body.solve_easy.clone(),
        solve_hardest: body.solve_hard.clone(),
        evaluate_easiest: body.evaluate_easy.clone(),
        evaluate_hardest: body.evaluate_hard.clone(),
        completion: completion_value,
        // TODO: pace_succeed?
        contact: "student@ewu.edu".to_string(), // TODO: fix
        comments: body.issues_comments.clone(),
    };
    data.get_ref().0.update_team_report(update_value.clone());
    // TODO: add email confirmation
    let email_out = email::team::send_confirmation_email(
        "Not Stu <nelof50340@soremap.com>".to_string(),
        update_value,
        &data.get_ref().2,
        &data.get_ref().1,
    );
    match email_out {
        Ok(message) => HttpResponse::Ok().body(format!("Done (Confirmation {})", message)),
        Err(message) => {
            HttpResponse::InternalServerError().body(format!("Done (Error {})", message))
        }
    }
    // debug!("body.client_meeting: {}", body.client_meeting);
    // HttpResponse::NotImplemented().body("Not Ready")
}

/// TODO: remove using for dev or require to be admin
#[get("/send_test_email")]
pub async fn send_email(data: Data<(Database, Config, Config)>) -> HttpResponse {
    // TODO: remove after dev or require to be admin
    let email_out = email::test::send_test_email(
        format!(
            "admin <{}>",
            data.get_ref()
                .1
                .get::<String>("admin_email")
                .expect("Missing admin email")
        ),
        &data.get_ref().2,
        &data.get_ref().1,
    );
    match email_out {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(message) => HttpResponse::InternalServerError().body(message),
    }
}

/// TODO: remove using for dev or require to be admin
#[post("/send_test_email")]
pub async fn send_email_to(
    data: Data<(Database, Config, Config)>,
    email_info: Json<EmailInfo>,
) -> HttpResponse {
    // TODO: remove after dev or require to be admin
    let email_out = email::test::send_test_email(
        format!("{} <{}>", email_info.name, email_info.email),
        &data.get_ref().2,
        &data.get_ref().1,
    );
    match email_out {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(message) => HttpResponse::InternalServerError().body(message),
    }
}

/// config api part of the actix service
///
/// > NOTE: also run oauth actix config
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(token::config)
            .service(send_email) // TODO: remove after dev or require to be admin
            .service(send_email_to) // TODO: remove after dev or require to be admin
            .service(get_sprints)
            .service(post_team),
    );
}
