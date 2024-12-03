use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub async fn send_email(
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    let smtp_host = env::var("SMTP_HOST")?;
    let smtp_port = env::var("SMTP_PORT")?.parse::<u16>()?;
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;
    let from_email = env::var("FROM_EMAIL")?;

    // Create email
    let email = Message::builder()
        .from(from_email.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    // Create SMTP transport
    let creds = Credentials::new(smtp_username, smtp_password);
    let mailer = SmtpTransport::relay(&smtp_host)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    // Send email
    mailer.send(&email)?;
    Ok(())
}
