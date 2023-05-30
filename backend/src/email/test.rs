use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use config::Config;

use log::{debug, error, info};

pub fn send_test_email(to_who: String, secret: &Config, server: &Config) -> Result<String, String> {
pub fn send_test_email(to_who: String, secret: &Config, server: &Config) -> Result<String, String> {
    info!("Sending test email");
    let email_result = Message::builder()
        // "NoBody <nobody@domain.tld>"
        .from(format!("Status Reports Server <{}>", server.get::<String>("smtp_email").expect("Missing sender email")).parse().unwrap())
        .reply_to(format!("no-reply <{}>", server.get::<String>("smtp_email").expect("Missing sender email")).parse().unwrap())
        // .to(format!("{} <{}>", to_who, server.get::<String>("admin_email").expect("Missing admin email")).parse().unwrap())
        .to(to_who.parse().unwrap())
        .subject("Test Email from Status Reports Backend")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"));
    let email;
    match email_result {
        Ok(em) => {
            email = em;
        }
        Err(e) => {
            error!("Unable to make email: {:?}", e);
            return Err(format!("Could not make email: {:?}", e));
        }
    };

    debug!("email made");

    let username = secret.get("smtp_username").expect("Missing email username");
    let password = secret.get("smtp_password").expect("Missing email password");

    let creds = Credentials::new(username, password);

    let server_url:String = server.get("smtp_server").expect("Missing email server url");

    // Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay(&*server_url)
        .unwrap()
        .credentials(creds)
        .build();

    debug!("mailer ready");

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            info!("Email sent");
            Ok("Email sent successfully!".to_string())
        },
        Err(e) => {
            error!("Email not sent");
            Err(format!("Could not send email: {:?}", e))
        },
    }
}