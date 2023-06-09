/// # Main to project backend also know as the server

// TODO: Login
// TODO: Individual Report
// TODO: Team Report
// TODO: Email

use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use config::Config;
use log::{error, info};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Serialize;
use std::process;
use std::process::Command;

use database::repository;

mod api;
mod email;

/// ## simple response struct
///
/// Make it easy to send json for a message instead of text
///
/// > TODO: move to common
#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

/// ## health api (get)
///
/// easy way to see if the server is healthy
///
/// ### TODO:
///
///  - check on config files
///  - check on database
///  - check on email server
///  - check on logging
///  - any other parts of the server
///
#[get("/health")]
async fn healthcheck() -> impl Responder {
    // TODO: check on config files
    // TODO: check on database
    // TODO: check on email server
    // TODO: check on logging
    // TODO: any other parts of the server
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

/// ## shutdown api (get)
///
/// > TODO: remove or admin only allow to call
///
/// Run a kill command to current actix thread.
///
/// NOTE: when any of the actix thread are killed then actix will shutdown
#[get("/shutdown")]
async fn shutdown_server() -> impl Responder {
    // TODO: check if admin
    let kill_results = Command::new("kill")
        .args(["-2".to_string(), process::id().to_string()])
        .spawn();
    match kill_results {
        Ok(_) => {
            let response = Response {
                message: "Shutdown everything".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let response = Response {
                message: "Can't Shutdown everything".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

/// ## Not found (404)
///
/// Make a result in json for when the page is not found (http 404 code)
async fn not_found() -> Result<HttpResponse> {
    error!("Invalid page load");
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

/// ## Start of backend code
///
/// Run the backend of the project.
/// Used to server the pages and
/// provide a rest API to verify info and save/get info from database.
///
/// ### Parts of the code
///
/// - Get the config files (server.config, secret.config)
/// - Setup Logger
/// - Call setup db
/// - Setup SSL
/// - Setup actix HTTP server
/// - Start actix server
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config file
    let server_config = Config::builder()
        .add_source(config::File::with_name("server.config"))
        .build()
        .expect("Missing Server Config File");
    let secret_config = Config::builder()
        .add_source(config::File::with_name("secret.config"))
        .build()
        .expect("Missing Secret Config File");

    // Logger
    if std::env::var_os("RUST_LOG").is_none() {
        //std::env::set_var("RUST_LOG", "actix_web=info");
        std::env::set_var("RUST_LOG", "info");
        // std::env::set_var("RUST_LOG", "debug");
    }
    // TODO: add log file (Look like we need to change what logger we use)
    env_logger::init();

    info!("Start of web server");

    // database
    let todo_db = repository::db_connector::Database::new();
    let app_data = web::Data::new((todo_db, server_config.clone(), secret_config.clone()));

    let port = server_config
        .get("port")
        .expect("Missing Port in Server Config");

    info!("Using port: {}", port);

    // SSL
    let mut use_ssl: bool = server_config
        .get("SSL_ENABLE")
        .expect("Missing \"SSL_ENABLE\" value");
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Unable to setup ssl builder (NOTE: needed with and without ssl)");
    if use_ssl {
        let ssl_certificate_file_path: String = server_config
            .get("SSL_CERTIFICATE")
            .expect("Missing \"SSL_CERTIFICATE\" value");
        let ssl_key_file_path: String = server_config
            .get("SSL_KEY")
            .expect("Missing \"SSL_KEY\" value");
        info!("Getting SSL");
        let key_results = builder.set_private_key_file(ssl_key_file_path, SslFiletype::PEM);
        match key_results {
            Ok(_) => {
                let cert_results = builder.set_certificate_chain_file(ssl_certificate_file_path);
                match cert_results {
                    Ok(_) => {}
                    Err(_) => {
                        error!("Can't read ssl certificate chain file");
                        use_ssl = false;
                    }
                }
            }
            Err(_) => {
                error!("Can't read ssl key file");
                use_ssl = false;
            }
        }
    }

    // Http Server
    let server_builder = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::api_services::config)
            .service(healthcheck)
            .service(shutdown_server) // TODO: remove after dev or require to be admin
            .service(
                Files::new("/studentpage", "./studentpage/dist/")
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = NamedFile::open_async("./studentpage/dist/index.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    })),
            )
            .service(
                Files::new("/adminpage", "./adminpage/dist/")
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = NamedFile::open_async("./adminpage/dist/index.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    })),
            )
            // TODO: redirect root to student page
            .default_service(Files::new("/", "./res/").default_handler(web::route().to(not_found)))
            .wrap(actix_web::middleware::Logger::default())
    });
    let server = if use_ssl {
        info!("Running using ssl (https)");
        server_builder.bind_openssl(format!("0.0.0.0:{}", port), builder)?
    } else {
        info!("Running using no ssl (http)");
        server_builder.bind(("0.0.0.0", port))?
    };
    server.run().await
}
