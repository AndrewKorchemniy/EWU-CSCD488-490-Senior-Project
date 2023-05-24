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

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRedirectUrl {
    url: String,
}

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

    let aru = AuthRedirectUrl {
        url: authorize_url.to_string(),
    };

    HttpResponse::Ok().json(aru)
}

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
                .2
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
                .2
                .get("OAUTH_AUTH_URL")
                .expect("Missing \"OAUTH_AUTH_URL\" value"),
        )
        .expect("blah"),
        Some(
            TokenUrl::new(
                app_data
                    .get_ref()
                    .2
                    .get("OAUTH_TOKEN_URL")
                    .expect("Missing \"OAUTH_TOKEN_URL\" value"),
            )
            .expect("blah"),
        ),
    );
    client
}
