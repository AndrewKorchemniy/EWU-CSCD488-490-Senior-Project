use actix_web::{web::Form, web::Data, post, web::Json};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    // CsrfToken,
    // PkceCodeChallenge,
    RedirectUrl,
    // Scope,
    // TokenResponse,
    TokenUrl,
    PkceCodeVerifier,

};
use oauth2::basic::BasicClient;
// use oauth2::reqwest::http_client;
use serde::{Serialize, Deserialize};
use log::{info, error};
use database::repository::database::Database;
use config::Config;

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

#[post("/token")]
pub async fn token(
    app_data: Data<(Database, Config, Config)>,
    body: Form<TokenBody>
) -> Json<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>> {
    let req = body.into_inner();
    println!("{:?}", req);
    let auth_url;
    let use_ssl:bool = app_data.get_ref().1.get("SSL_ENABLE")
        .expect("Missing \"SSL_ENABLE\" value");
    if use_ssl {
        let url:String = app_data.get_ref().1.get("DOMAIN")
            .expect("Missing \"DOMAIN\" value");
        let port:isize = app_data.get_ref().1.get("port")
            .expect("Missing Port in Server Config");
        auth_url = format!("https://{url}:{port}/studentpage");
    } else {
        // TODO: can't use oauth2
        auth_url = "http://auth".to_string();
    }
    let client = BasicClient::new(
        ClientId::new(app_data.get_ref().2.get("OAUTH_CLIENT_ID")
            .expect("Missing \"OAUTH_CLIENT_ID\" value")),
        Some(ClientSecret::new(app_data.get_ref().2.get("OAUTH_CLIENT_SECRET")
            .expect("Missing \"OAUTH_CLIENT_SECRET\" value"))),
        AuthUrl::new(auth_url).expect("blah"),
        Some(TokenUrl::new(app_data.get_ref().2.get("OAUTH_TOKEN_URL")
            .expect("Missing \"OAUTH_TOKEN_URL\" value")).expect("blah"))
    ).set_redirect_uri(RedirectUrl::new(req.redirect_uri).expect("Issue constructing Redirect url"));

    let pkce_verifier = PkceCodeVerifier::new(req.code_verifier);
    let token_result = client.exchange_code(AuthorizationCode::new(req.code))
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