use dotenv::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::utils::notifications::warn;

pub fn send(to: &str) -> bool {
    dotenv().ok();

    let email = Message::builder()
        .from(
            "Project Rapport <noreply@projectrapport.rs>"
                .parse()
                .unwrap(),
        )
        .to(format!("Supervisor <{}>",to).parse().unwrap())
        .subject("Attention Required: Negative Feedback Threshold Reached")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(
            "This is to alert you that consecutive negative feedback has exceeded the set threshold!",
        ))
        .unwrap();

    let username = std::env::var("MAIL_USERNAME");
    let password = std::env::var("MAIL_PASSWORD");

    match (username, password) {
        (Ok(username), Ok(password)) => {
            let creds = Credentials::new(username, password);
            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();
            mailer.send(&email).is_ok()
        }
        _ => {
            warn("Mail credentials missing!\nTo send emails, we need your email username and password. Please set the environment variables MAIL_USERNAME and MAIL_PASSWORD.");
            false
        }
    }
}
