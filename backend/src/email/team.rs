use common::models::status_report::TeamReport;
use config::Config;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use log::{debug, error, info};

pub fn send_confirmation_email(
    to_who: String,
    report_data: TeamReport,
    secret: &Config,
    server: &Config,
) -> Result<String, String> {
    info!("Sending team report confirmation email {}", to_who);
    let email_result = Message::builder()
        // "NoBody <nobody@domain.tld>"
        .from(
            format!(
                "Status Reports Server <{}>",
                server
                    .get::<String>("smtp_email")
                    .expect("Missing sender email")
            )
            .parse()
            .unwrap(),
        )
        .reply_to(
            format!(
                "no-reply <{}>",
                server
                    .get::<String>("smtp_email")
                    .expect("Missing sender email")
            )
            .parse()
            .unwrap(),
        )
        // .to(format!("{} <{}>", to_who, server.get::<String>("admin_email").expect("Missing admin email")).parse().unwrap())
        .to(to_who.parse().unwrap())
        .subject("Team Report Confirmation Email")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(format!(
            "A team report was successfully submitted by {to_who}.\n\
            Sprint {} Team Report:\n\
            Understand - Easiest: {}\n\
            Understand - Hardest: {}\n\
            Approach - Easiest: {}\n\
            Approach - Hardest: {}\n\
            Solve - Easiest: {}\n\
            Solve - Hardest: {}\n\
            Evaluate - Easiest: {}\n\
            Evaluate - Hardest: {}\n\
            Completion: {}\n\
            Contact: {}\n\
            Comments: {}",
            report_data.sprint_num,
            report_data.understand_easiest,
            report_data.understand_hardest,
            report_data.approach_easiest,
            report_data.approach_hardest,
            report_data.solve_easiest,
            report_data.solve_hardest,
            report_data.evaluate_easiest,
            report_data.evaluate_hardest,
            report_data.completion,
            report_data.contact,
            report_data.comments
        )));
    let email = match email_result {
        Ok(em) => em,
        Err(e) => {
            error!("Unable to make email: {:?}", e);
            return Err(format!("Could not make email: {:?}", e));
        }
    };

    debug!("email made");

    let username = secret.get("smtp_username").expect("Missing email username");
    let password = secret.get("smtp_password").expect("Missing email password");

    let creds = Credentials::new(username, password);

    let server_url: String = server.get("smtp_server").expect("Missing email server url");

    // Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay(&server_url)
        .unwrap()
        .credentials(creds)
        .build();

    debug!("mailer ready");

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            info!("Email sent");
            Ok("Email sent successfully!".to_string())
        }
        Err(e) => {
            error!("Email not sent");
            Err(format!("Could not send email: {:?}", e))
        }
    }
}
