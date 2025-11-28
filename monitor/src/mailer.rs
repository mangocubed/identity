use apalis::prelude::BoxDynError;
use lettre::message::header::ContentType;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use lettre::{Message, transport::smtp::authentication::Credentials};

use identity_core::config::MAILER_CONFIG;
use identity_core::enums::ConfirmationAction;
use identity_core::models::{Confirmation, Session, User};

use crate::ApalisError;

impl<T> ApalisError<T> for Result<T, lettre::transport::smtp::Error> {
    fn or_apalis_error(self) -> Result<T, apalis::prelude::Error> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(apalis::prelude::Error::from(Box::new(err) as BoxDynError)),
        }
    }
}

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
    .or_apalis_error()?;

    Ok(())
}

pub async fn send_new_confirmation_email(
    confirmation: &Confirmation<'_>,
    code: &str,
) -> Result<(), apalis::prelude::Error> {
    let user = confirmation.user().await;

    let message = format!(
        "Hello {},

Use this code to {}:

{}

If you don't recognize this action, you can ignore this message.",
        user.username,
        match confirmation.action {
            ConfirmationAction::Email => "confirm your email",
            ConfirmationAction::Login => "confirm your login",
            ConfirmationAction::PasswordReset => "reset your password",
        },
        code,
    );

    send_email(&user.email, "Confirmation code", &message).await
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

pub async fn send_welcome_email(user: &User<'_>) -> Result<(), apalis::prelude::Error> {
    let message = format!(
        "Hello @{},

        Welcome to Mango³.

        If you have any questions, please contact us at the following email address: {}",
        user.username, MAILER_CONFIG.support_email_address
    );

    send_email(&user.email, "Welcome to Mango³", &message).await
}

pub mod admin_emails {
    use super::*;

    pub async fn send_new_user_email(user: &User<'_>) -> Result<(), apalis::prelude::Error> {
        let message = format!(
            "Hello,

Someone has created a new user account with the following username: @{}",
            user.username
        );

        send_email(
            &MAILER_CONFIG.support_email_address,
            "(Admin) New user account created",
            &message,
        )
        .await
    }
}
