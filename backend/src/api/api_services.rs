use crate::api::token;
use crate::email::test::send_test_email;
use actix_web::web;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    HttpResponse,
};
use config::Config;
use log::debug;
use database::repository::db_connector::Database;
use serde::{Deserialize, Serialize};
use common::models::types::TeamResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailInfo {
    pub email: String,
    pub name: String,
}

//https://actix.rs/docs/databases/


#[get("/sprints")]
pub async fn get_sprints(
    data: Data<(Database, Config, Config)>,
) -> HttpResponse {
   HttpResponse::NotImplemented().body("Not Ready")
}

#[get("/submit/team")]
pub async fn post_team(
    data: Data<(Database, Config, Config)>,
    body: web::Json<TeamResponse>
) -> HttpResponse {
    debug!("body.client_meeting: {}", body.client_meeting);
    HttpResponse::NotImplemented().body("Not Ready")
}

// TODO: remove using for dev
#[get("/send_test_email")]
pub async fn send_email(data: Data<(Database, Config, Config)>) -> HttpResponse {
    // TODO: remove after dev or require to be admin
    let email_out = send_test_email(
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

// TODO: remove using for dev
#[post("/send_test_email")]
pub async fn send_email_to(
    data: Data<(Database, Config, Config)>,
    email_info: Json<EmailInfo>,
) -> HttpResponse {
    // TODO: remove after dev or require to be admin
    let email_out = send_test_email(
        format!("{} <{}>", email_info.name, email_info.email),
        &data.get_ref().2,
        &data.get_ref().1,
    );
    match email_out {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(message) => HttpResponse::InternalServerError().body(message),
    }
}

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
