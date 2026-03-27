use chrono::NaiveDate;
use leptos::prelude::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use identity_core::commands;
#[cfg(feature = "ssr")]
use identity_core::enums::ConfirmationAction;
#[cfg(feature = "ssr")]
use identity_core::params::*;

use crate::presenters::UserPresenter;

#[cfg(feature = "ssr")]
use crate::server_fns::KEY_SESSION_ID;

use super::{ActionResult, ServerFnResult};

#[cfg(feature = "ssr")]
use super::*;

#[server]
pub async fn confirm_email(confirmation_code: String) -> ActionResult {
    require_authentication().await?;

    let user = extract_user().await?;

    commands::confirm_user_email(&user, ConfirmationParams { confirmation_code }).await?;

    Ok(())
}

#[server]
pub async fn create_user(
    username: String,
    email: String,
    password: String,
    full_name: String,
    birthdate: Option<NaiveDate>,
    country_code: String,
) -> ActionResult {
    require_no_authentication().await?;

    let user = commands::insert_user(UserParams {
        username,
        email,
        password,
        full_name,
        birthdate,
        country_code,
    })
    .await?;

    let client_ip = extract_client_ip().await?;

    let user_session = commands::insert_session(&user, client_ip).await?;

    let tower_session = extract_tower_session().await?;

    tower_session.insert(KEY_SESSION_ID, user_session.id).await?;

    Ok(())
}

#[server]
pub async fn current_user() -> ServerFnResult<UserPresenter> {
    let user = extract_user().await.or_unauthorized()?;

    Ok(user.into())
}

#[server]
pub async fn reset_password(confirmation_id: Uuid, confirmation_code: String, new_password: String) -> ActionResult {
    require_no_authentication().await?;

    commands::reset_user_password(ResetPasswordParams {
        confirmation_id,
        confirmation_code,
        new_password,
    })
    .await?;

    Ok(())
}

#[server]
pub async fn send_email_confirmation() -> ActionResult {
    require_authentication().await?;

    let user = extract_user().await?;

    if user.email_is_confirmed() {
        return Err(ActionError::default());
    }

    commands::insert_confirmation(&user, ConfirmationAction::Email).await?;

    Ok(())
}

#[server]
pub async fn send_password_reset_confirmation(username_or_email: String) -> ActionResult<Uuid> {
    require_no_authentication().await?;

    let user = commands::get_user_by_username_or_email(&username_or_email).await?;

    let confirmation = commands::insert_confirmation(&user, ConfirmationAction::PasswordReset).await?;

    Ok(confirmation.id)
}

#[server]
pub async fn update_password(current_password: String, new_password: String) -> ActionResult {
    require_authentication().await?;

    let user = extract_user().await?;

    commands::update_user_password(
        &user,
        PasswordParams {
            current_password,
            new_password,
        },
    )
    .await?;

    Ok(())
}

#[server]
pub async fn update_email(email: String, password: String) -> ActionResult {
    require_authentication().await?;

    let user = extract_user().await?;

    commands::update_user_email(&user, EmailParams { email, password }).await?;

    Ok(())
}

#[server]
pub async fn update_profile(
    display_name: String,
    full_name: String,
    birthdate: Option<NaiveDate>,
    country_code: String,
) -> ActionResult {
    require_authentication().await?;

    let user = extract_user().await?;

    commands::update_user_profile(
        &user,
        ProfileParams {
            display_name,
            full_name,
            birthdate,
            country_code,
        },
    )
    .await?;

    Ok(())
}
