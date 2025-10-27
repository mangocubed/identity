use lettre::message::header::ContentType;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use lettre::{Message, transport::smtp::authentication::Credentials};

use identity_core::config::MAILER_CONFIG;
use identity_core::models::{Session, User};

async fn send_email(to: &str, subject: &str, body: &str) -> Result<(), apalis::prelude::Error> {
    if !MAILER_CONFIG.enable {
        return Ok(());
    }

    let message = Message::builder()
        .from(
            MAILER_CONFIG
                .sender_address
                .parse()
                .expect("Could not parse mailer sender address"),
        )
        .to(to.parse().expect("Could not parse recipient address"))
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .expect("Could not build message");

    let credentials = Credentials::new(
        MAILER_CONFIG.smtp_username.to_owned(),
        MAILER_CONFIG.smtp_password.to_owned(),
    );

    match MAILER_CONFIG.smtp_security.as_str() {
        "tls" => AsyncSmtpTransport::<Tokio1Executor>::relay(&MAILER_CONFIG.smtp_address),
        "starttls" => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&MAILER_CONFIG.smtp_address),
        _ => Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
            MAILER_CONFIG.smtp_address.clone(),
        )),
    }
    .expect("Could not get SMTP transport builder")
    .credentials(credentials)
    .build()
    .send(message)
    .await
    .expect("Could not send email");

    Ok(())
}

pub async fn send_new_user_email(user: &User<'_>) -> Result<(), apalis::prelude::Error> {
    let message = format!(
        "Hello @{},

        Welcome to Mango³.

        If you have any questions, please contact us at the following email address: {}",
        user.username, MAILER_CONFIG.support_email_address
    );

    send_email(&user.email, "Welcome to Mango³", &message).await
}

pub async fn send_new_session_email(session: &Session<'_>) -> Result<(), apalis::prelude::Error> {
    let user = session.user().await;

    let message = format!(
        "Hello @{},

Someone has started a new session from:

Application: {}
Location: {}

If you recognize this action, you can ignore this message.

If not, please contact us at the following email address: {}",
        user.username,
        session.user_agent,
        session.location(),
        MAILER_CONFIG.support_email_address,
    );

    send_email(&user.email, "New session started", &message).await
}

pub async fn send_password_changed_email(user: &User<'_>) -> Result<(), apalis::prelude::Error> {
    let message = format!(
        "Hello @{},

Your password has been changed.

If you recognize this action, you can ignore this message.

If not, please contact us at the following email address: {}",
        user.username, MAILER_CONFIG.support_email_address
    );

    send_email(&user.email, "Password changed", &message).await
}
