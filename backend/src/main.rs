// TODO: Login
// TODO: Individual Report
// TODO: Team Report
// TODO: Email

use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, Result, get, web};
use serde::{Serialize};
use config::Config;
use log::{error, info};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/shutdown")]
async fn shutdown_server() -> impl Responder {
    let response = Response {
        message: "Shutdown everything".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    error!("Invalid page load");
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: config file
        // TODO: add thread to watch config file
    // TODO: logger
    // TODO: database
        // TODO: mySQL
    // TODO: server
        // TODO: api's
        // TODO: load frontend
    let server_config = Config::builder()
        .add_source(config::File::with_name("server.config.toml"))
        .build().expect("Missing Config File");

    if std::env::var_os("RUST_LOG").is_none() {
        //std::env::set_var("RUST_LOG", "actix_web=info");
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("Start of web server");

    // let todo_db = repository::database::Database::new();
    // let app_data = web::Data::new(todo_db);

    let port = server_config.get("port")
        .expect("Missing Port in Server Config");

    info!("Using port: {}", port);

    // TODO: remove duplicate code
    // let app = App::new()
    //     .app_data(app_data.clone())
    //     .configure(api::api::config)
    //     .service(healthcheck)
    //     .default_service(web::route().to(not_found))
    //     .wrap(actix_web::middleware::Logger::default());

    let mut use_ssl:bool = server_config.get("SSL_ENABLE")
        .expect("Missing \"SSL_ENABLE\" value");
    let mut builder= SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    if use_ssl {
        let ssl_certificate_file_path:String = server_config.get("SSL_CERTIFICATE")
            .expect("Missing \"SSL_CERTIFICATE\" value");
        let ssl_key_file_path:String = server_config.get("SSL_KEY")
            .expect("Missing \"SSL_KEY\" value");
        info!("Getting SSL");
        let key_results = builder
            .set_private_key_file(ssl_key_file_path, SslFiletype::PEM);
        match key_results
        {
            Ok(_) => {}
            Err(_) => {
                error!("Can't read ssl key file");
                use_ssl = false;
            }
        }
        let cert_results = builder.set_certificate_chain_file(ssl_certificate_file_path);
        match cert_results
        {
            Ok(_) => {}
            Err(_) => {
                error!("Can't read ssl certificate chain file");
                use_ssl = false;
            }
        }
    }
    if use_ssl {
        info!("Running using ssl (https)");
        HttpServer::new(move || App::new()
            // .app_data(app_data.clone())
            .service(healthcheck)
            .service(Files::new("/studentpage", "./studentpage/dist/").index_file("index.html"))
            .service(Files::new("/adminpage", "./adminpage/dist/").index_file("index.html"))
            .default_service(Files::new("/", "./res/"))
            .wrap(actix_web::middleware::Logger::default()))
            .bind_openssl(format!("0.0.0.0:{}", port), builder)?
            .run()
            .await
    } else {
        info!("Running using no ssl (http)");
        HttpServer::new(move || App::new()
            // .app_data(app_data.clone())
            .service(healthcheck)
            .service(Files::new("/studentpage", "./studentpage/dist/").index_file("index.html"))
            .service(Files::new("/adminpage", "./adminpage/dist/").index_file("index.html"))
            .default_service(Files::new("/", "./res/"))
            .wrap(actix_web::middleware::Logger::default()))
            .bind(("0.0.0.0", port))?
            .run()
            .await
    }
}
