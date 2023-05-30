use crate::api::token;
use actix_web::web;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    HttpResponse,
};
use common::models::todo::Todo;
use config::Config;
use database::repository::db_connector::Database;
use crate::email::test::send_test_email;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailInfo {
    pub email: String,
    pub name: String
}

// https://actix.rs/docs/databases/

#[post("/todos")]
pub async fn create_todo(
    data: Data<(Database, Config, Config)>,
    new_todo: Json<Todo>,
) -> HttpResponse {
    let todo = data.get_ref().0.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(data: Data<(Database, Config, Config)>) -> HttpResponse {
    let todos = data.get_ref().0.get_todos();
    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(
    data: Data<(Database, Config, Config)>,
    id: web::Path<String>,
) -> HttpResponse {
    let todo = data.get_ref().0.get_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(
    data: Data<(Database, Config, Config)>,
    id: web::Path<String>,
    updated_todo: web::Json<Todo>,
) -> HttpResponse {
    let todo = data
        .get_ref()
        .0
        .update_todo_by_id(&id, updated_todo.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(
    data: Data<(Database, Config, Config)>,
    id: web::Path<String>,
) -> HttpResponse {
    let todo = data.get_ref().0.delete_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

// TODO: remove using for dev
#[get("/send_test_email")]
pub async fn send_email(
    data: Data<(Database, Config, Config)>,
) -> HttpResponse {
    let email_out = send_test_email(
        format!("admin <{}>", data.get_ref().1.get::<String>("admin_email").expect("Missing admin email")), &data.get_ref().2, &data.get_ref().1);
    match email_out {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(message) => HttpResponse::InternalServerError().body(message),
    }
}

// TODO: remove using for dev
#[post("/send_test_email")]
pub async fn send_email_to(
    data: Data<(Database, Config, Config)>,
    email_info: Json<EmailInfo>
) -> HttpResponse {
    let email_out = send_test_email(format!("{} <{}>", email_info.name, email_info.email), &data.get_ref().2, &data.get_ref().1);
    match email_out {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(message) => HttpResponse::InternalServerError().body(message),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(token::config)
            .service(send_email)
            .service(send_email_to)
            .service(create_todo)
            .service(get_todos)
            .service(get_todo_by_id)
            .service(update_todo_by_id)
            .service(delete_todo_by_id),
    );
}
