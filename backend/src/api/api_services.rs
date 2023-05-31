use crate::api::token;
use crate::email::test::send_test_email;
use actix_web::web;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    HttpResponse,
};
use common::models::todo::Todo;// TODO: remove todo example
use config::Config;
use database::repository::db_connector::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailInfo {
    pub email: String,
    pub name: String,
}

//https://actix.rs/docs/databases/

// TODO: remove todo example
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

// TODO: remove todo example
#[get("/todos")]
pub async fn get_todos(data: Data<(Database, Config, Config)>) -> HttpResponse {
    let todos = data.get_ref().0.get_todos();
    HttpResponse::Ok().json(todos)
}

// TODO: remove todo example
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

// TODO: remove todo example
#[put("/todos/{id}")]
pub async fn update_todo_by_id(
    data: Data<(Database, Config, Config)>,
    id: web::Path<String>,
    _updated_todo: web::Json<Todo>,
) -> HttpResponse {
    let todo = data
        .get_ref()
        .0
        .delete_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

// TODO: remove todo example
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
            .service(send_email)// TODO: remove after dev or require to be admin
            .service(send_email_to)// TODO: remove after dev or require to be admin
            .service(create_todo)// TODO: remove todo example
            .service(get_todos)// TODO: remove todo example
            .service(get_todo_by_id)// TODO: remove todo example
            .service(update_todo_by_id)// TODO: remove todo example
            .service(delete_todo_by_id),// TODO: remove todo example
    );
}
