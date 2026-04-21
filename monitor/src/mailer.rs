use toolbox::config::MAILER_CONFIG;
use toolbox::mailer::send_email;

use identity_core::enums::ConfirmationAction;
use identity_core::models::{Confirmation, Session, User};

pub async fn send_new_confirmation_email(confirmation: &Confirmation<'_>, code: &str) -> anyhow::Result<()> {
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

pub async fn send_new_session_email(session: &Session) -> anyhow::Result<()> {
    let user = session.user().await?;

    let message = format!(
        "Hello @{},

Someone has started a new session from:

Location: {}

If you recognize this action, you can ignore this message.

If not, please contact us at the following email address: {}",
        user.username,
        session.location(),
        MAILER_CONFIG.support_email_address,
    );

    send_email(&user.email, "New session started", &message).await
}

pub async fn send_password_changed_email(user: &User<'_>) -> anyhow::Result<()> {
    let message = format!(
        "Hello @{},

Your password has been changed.

If you recognize this action, you can ignore this message.

If not, please contact us at the following email address: {}",
        user.username, MAILER_CONFIG.support_email_address
    );

    send_email(&user.email, "Password changed", &message).await
}

pub async fn send_welcome_email(user: &User<'_>) -> anyhow::Result<()> {
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

    pub async fn send_new_user_email(user: &User<'_>) -> anyhow::Result<()> {
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
