use actix_web::web;
use actix_web::{get, post, web::Data, web::Form, web::Json, HttpResponse};
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse, BasicTokenType,
};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, PkceCodeVerifier,
    RedirectUrl, Scope, StandardRevocableToken, TokenUrl,
};
// use oauth2::reqwest::http_client;
use config::Config;
use database::repository::db_connector::Database;
use log::{error, info};
use serde::{Deserialize, Serialize};

/// TODO: look into this
///
/// From example online
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

/// ## OAuth Public Config response struct
///
/// Make it easy to send json
///
/// > TODO: move to common
#[derive(Serialize)]
pub struct OAuthClientConfig {
    client_id: String,
    auth_url: String,
    token_url: String,
}

/// ## simple response struct
///
/// Make it easy to send json for a message instead of text
///
/// > TODO: move to common
#[derive(Serialize)]
pub struct Response {
    message: String,
}

// TODO: setup login

/// config oauth api part of the actix
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("oauth")
            .service(login)
            .service(token)
            .service(give_config)
            .service(give_client_id)
            .service(give_auth_url)
            .service(give_token_url),
    );
}

/// give public parts of the oauth config in json format for http request
#[get("/config")]
pub async fn give_config(app_data: Data<(Database, Config, Config)>) -> HttpResponse {
    let client_id_results: Result<String, _> = app_data.get_ref().1.get("OAUTH_CLIENT_ID");

    match client_id_results {
        Ok(client_id) => {
            let auth_url: Result<String, _> = app_data.get_ref().1.get("OAUTH_AUTH_URL");

            match auth_url {
                Ok(auth_url) => {
                    let token_url: Result<String, _> = app_data.get_ref().1.get("OAUTH_TOKEN_URL");

                    match token_url {
                        Ok(token_url) => HttpResponse::Ok().json(OAuthClientConfig {
                            client_id,
                            auth_url,
                            token_url,
                        }),
                        Err(_) => HttpResponse::InternalServerError().json(Response {
                            message: String::from("Unable to get token url"),
                        }),
                    }
                }
                Err(_) => HttpResponse::InternalServerError().json(Response {
                    message: String::from("Unable to get auth url"),
                }),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(Response {
            message: String::from("Unable to get client id"),
        }),
    }
}

/// give oauth token url in json message for http request
#[get("/tokenurl")]
pub async fn give_token_url(app_data: Data<(Database, Config, Config)>) -> HttpResponse {
    let token_url: Result<String, _> = app_data.get_ref().1.get("OAUTH_TOKEN_URL");

    match token_url {
        Ok(token_url) => HttpResponse::Ok().json(Response { message: token_url }),
        Err(_) => HttpResponse::InternalServerError().json(Response {
            message: String::from("Unable to get token url"),
        }),
    }
}

/// give oauth auth url in json message for http request
#[get("/authurl")]
pub async fn give_auth_url(app_data: Data<(Database, Config, Config)>) -> HttpResponse {
    let auth_url: Result<String, _> = app_data.get_ref().1.get("OAUTH_AUTH_URL");

    match auth_url {
        Ok(auth_url) => HttpResponse::Ok().json(Response { message: auth_url }),
        Err(_) => HttpResponse::InternalServerError().json(Response {
            message: String::from("Unable to get auth url"),
        }),
    }
}

/// give oauth client id in json message for http request
#[get("/clientid")]
pub async fn give_client_id(app_data: Data<(Database, Config, Config)>) -> HttpResponse {
    let client_id: Result<String, _> = app_data.get_ref().1.get("OAUTH_CLIENT_ID");

    match client_id {
        Ok(client_id) => HttpResponse::Ok().json(Response { message: client_id }),
        Err(_) => HttpResponse::InternalServerError().json(Response {
            message: String::from("Unable to get client id"),
        }),
    }
}

/// This is a bad login do not use
#[get("/login")]
pub async fn login(app_data: Data<(Database, Config, Config)>) -> HttpResponse {
    // TODO: add
    let use_ssl: bool = app_data
        .get_ref()
        .1
        .get("SSL_ENABLE")
        .expect("Missing \"SSL_ENABLE\" value");
    let return_url: String = if use_ssl {
        let url: String = app_data
            .get_ref()
            .1
            .get("DOMAIN")
            .expect("Missing \"DOMAIN\" value");
        let port: isize = app_data
            .get_ref()
            .1
            .get("port")
            .expect("Missing Port in Server Config");
        format!("https://{url}:{port}/studentpage")
    } else {
        // TODO: can't use oauth2
        "http://return".to_string()
    };
    let client = make_client(app_data);
    let client = client.set_redirect_uri(
        // TODO: make the url from config files
        RedirectUrl::new(return_url).expect("Invalid redirect URL"),
    );
    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        // .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    // TODO: use csrf_state to check login

    let aru = Response {
        message: authorize_url.to_string(),
    };

    HttpResponse::Ok().json(aru)
}

/// TODO: what is this used for
///
/// From example online
#[post("/token")]
pub async fn token(
    app_data: Data<(Database, Config, Config)>,
    body: Form<TokenBody>,
) -> Json<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>>
{
    let req = body.into_inner();
    //println!("{:?}", req);
    let client = make_client(app_data);

    let client = client.set_redirect_uri(
        RedirectUrl::new(req.redirect_uri).expect("Issue constructing Redirect url"),
    );

    let pkce_verifier = PkceCodeVerifier::new(req.code_verifier);
    let token_result = client
        .exchange_code(AuthorizationCode::new(req.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match token_result {
        Err(err) => {
            error!("{:?}", err.to_string());
            panic!("TODO better error handling here");
        }
        Ok(val) => {
            info!("Tokens received from OAuth provider!");
            Json(val)
        }
    }
}

/// Helper to make a client that often used in this file
fn make_client(
    app_data: Data<(Database, Config, Config)>,
) -> Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenType,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
> {
    let client = BasicClient::new(
        ClientId::new(
            app_data
                .get_ref()
                .1
                .get("OAUTH_CLIENT_ID")
                .expect("Missing \"OAUTH_CLIENT_ID\" value"),
        ),
        Some(ClientSecret::new(
            app_data
                .get_ref()
                .2
                .get("OAUTH_CLIENT_SECRET")
                .expect("Missing \"OAUTH_CLIENT_SECRET\" value"),
        )),
        AuthUrl::new(
            app_data
                .get_ref()
                .1
                .get("OAUTH_AUTH_URL")
                .expect("Missing \"OAUTH_AUTH_URL\" value"),
        )
        .expect("blah"),
        Some(
            TokenUrl::new(
                app_data
                    .get_ref()
                    .1
                    .get("OAUTH_TOKEN_URL")
                    .expect("Missing \"OAUTH_TOKEN_URL\" value"),
            )
            .expect("blah"),
        ),
    );
    client
}
