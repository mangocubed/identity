#[cfg(feature = "server")]
use std::net::{IpAddr, SocketAddr};

use dioxus::prelude::*;
use serde_json::Value;
use url::Url;
use uuid::Uuid;

#[cfg(feature = "server")]
use axum::extract::ConnectInfo;
#[cfg(feature = "server")]
use chrono::{TimeDelta, Utc};
#[cfg(feature = "server")]
use http::HeaderMap;

use sdk::app::{ActionResult, ServFnResult};

#[cfg(feature = "server")]
use sdk::app::{ActionError, ActionSuccess, HeaderMapExt};
#[cfg(feature = "server")]
use sdk::constants::{HEADER_USER_AGENT, HEADER_X_REAL_IP};

#[cfg(feature = "server")]
use identity_core::commands;
#[cfg(feature = "server")]
use identity_core::enums::ConfirmationAction;
#[cfg(feature = "server")]
use identity_core::models::{Session, User};

use crate::presenters::{SessionPresenter, UserPresenter, UserProfilePresenter};

#[cfg(feature = "server")]
fn extract_client_ip_addr(headers: &HeaderMap, connect_info: ConnectInfo<SocketAddr>) -> IpAddr {
    let real_ip = headers
        .get(HEADER_X_REAL_IP)
        .and_then(|ip| ip.to_str().ok())
        .and_then(|ip| ip.parse().ok());

    if let Some(real_ip) = real_ip {
        return real_ip;
    }

    connect_info.0.ip()
}

#[cfg(feature = "server")]
fn extract_user_agent(headers: &HeaderMap) -> String {
    headers
        .get(HEADER_USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("Unknown")
        .to_owned()
}

#[cfg(feature = "server")]
async fn extract_session<'a>(headers: &HeaderMap) -> Result<Session<'a>, HttpError> {
    let bearer = headers.bearer()?;

    commands::get_session_by_token(bearer.token())
        .await
        .or_forbidden("Forbidden")
}

#[cfg(feature = "server")]
async fn extract_user<'a>(headers: &HeaderMap) -> Result<User<'a>, HttpError> {
    let bearer = headers.bearer()?;

    commands::get_user_by_session_token(bearer.token())
        .await
        .or_else(|_| HttpError::unauthorized("Unauthorized"))
}

#[cfg(feature = "server")]
async fn require_no_session(headers: &HeaderMap) -> Result<(), HttpError> {
    headers.require_app_token()?;

    if extract_session(headers).await.is_err() {
        Ok(())
    } else {
        HttpError::forbidden("Forbidden")
    }
}

#[post("/api/authorize", headers: HeaderMap)]
pub async fn authorize(client_id: Uuid) -> ServFnResult<Url> {
    headers.require_app_token()?;

    let (application, session, user) = tokio::try_join!(
        async {
            commands::get_application_by_id(client_id)
                .await
                .or_not_found("Not Found")
        },
        extract_session(&headers),
        extract_user(&headers),
    )?;

    let authorization =
        commands::insert_or_refresh_authorization(&application, &user, &session, Utc::now() + TimeDelta::hours(1))
            .await
            .or_internal_server_error("Internal Server Error")?;

    let mut redirect_url = application.redirect_url();

    redirect_url.set_query(Some(&format!(
        "token={}&expires_at={}",
        authorization.token, authorization.expires_at
    )));

    Ok(redirect_url)
}

#[put("/api/change-password", headers: HeaderMap)]
pub async fn change_password(input: Value) -> ActionResult {
    headers.require_app_token()?;

    let user = extract_user(&headers).await?;

    let result = commands::update_user_password(&user, &serde_json::from_value(input)?).await;

    match result {
        Ok(_) => Ok(ActionSuccess::new("Password changed successfully", Value::Null)),
        Err(errors) => Err(ActionError::new("Failed to change password", Some(errors))),
    }
}

#[put("/api/confirm-email", headers: HeaderMap)]
pub async fn confirm_email(input: Value) -> ActionResult {
    headers.require_app_token()?;

    let user = extract_user(&headers).await?;

    let result = commands::confirm_user_email(&user, &serde_json::from_value(input)?).await;

    match result {
        Ok(_) => Ok(ActionSuccess::new("Email confirmed successfully", Value::Null)),
        Err(errors) => Err(ActionError::new("Failed to confirm email", Some(errors))),
    }
}

#[get("/api/current-user", headers: HeaderMap)]
pub async fn current_user() -> Result<UserPresenter> {
    headers.require_app_token()?;

    let user = extract_user(&headers).await?;

    Ok(UserPresenter::from(user))
}

#[get("/api/current-user/profile", headers: HeaderMap)]
pub async fn current_user_profile() -> Result<UserProfilePresenter> {
    headers.require_app_token()?;

    let user = extract_user(&headers).await?;

    Ok(UserProfilePresenter::from(user))
}

#[get("/api/can-register", headers: HeaderMap)]
pub async fn can_register() -> Result<bool, HttpError> {
    require_no_session(&headers).await?;

    Ok(commands::can_insert_user().await)
}

#[post("/api/login", headers: HeaderMap, connect_info: ConnectInfo<SocketAddr>)]
pub async fn login(input: Value) -> ActionResult {
    require_no_session(&headers).await?;

    let user = commands::authenticate_user(&serde_json::from_value(input)?)
        .await
        .map_err(|errors| ActionError::new("Failed to authenticate user", Some(errors)))?;

    let user_agent = extract_user_agent(&headers);
    let ip_addr = extract_client_ip_addr(&headers, connect_info);

    let result = commands::insert_session(&user, &user_agent, ip_addr).await;

    match result {
        Ok(session) => Ok(ActionSuccess::new(
            "User authenticated successfully",
            serde_json::to_value(SessionPresenter::from(session))?,
        )),
        Err(_) => Err(ActionError::new("Failed to authenticate user", None)),
    }
}

#[delete("/api/logout", headers: HeaderMap)]
pub async fn logout() -> Result<()> {
    headers.require_app_token()?;

    let session = extract_session(&headers).await?;

    commands::finish_session(&session)
        .await
        .or_internal_server_error("Internal server error")?;

    Ok(())
}

#[put("/api/refresh-session", headers: HeaderMap)]
pub async fn refresh_session() -> Result<SessionPresenter> {
    headers.require_app_token()?;

    let session = extract_session(&headers).await?;

    let session = commands::refresh_session(&session)
        .await
        .or_internal_server_error("Internal server error")?;

    Ok(session.into())
}

#[post("/api/register", headers: HeaderMap, connect_info: ConnectInfo<SocketAddr>)]
pub async fn register(input: Value) -> ActionResult {
    require_no_session(&headers).await?;

    let result = commands::insert_user(&serde_json::from_value(input)?).await;

    match result {
        Ok(user) => {
            let user_agent = extract_user_agent(&headers);
            let ip_addr = extract_client_ip_addr(&headers, connect_info);

            let session = commands::insert_session(&user, &user_agent, ip_addr)
                .await
                .or_internal_server_error("Internal server error")?;

            Ok(ActionSuccess::new(
                "User created successfully",
                serde_json::to_value(SessionPresenter::from(session))?,
            ))
        }
        Err(errors) => Err(ActionError::new("Failed to create user", Some(errors))),
    }
}

#[put("/api/reset-password", headers: HeaderMap)]
pub async fn reset_password(input: Value) -> ActionResult {
    require_no_session(&headers).await?;

    let result = commands::reset_user_password(&serde_json::from_value(input)?).await;

    match result {
        Ok(_) => ActionSuccess::ok("Password updated successfully", Value::Null),
        Err(errors) => ActionError::err("Failed to reset password", Some(errors)),
    }
}

#[post("/api/send-email-confirmation", headers: HeaderMap)]
pub async fn send_email_confirmation() -> Result<()> {
    headers.require_app_token()?;

    let user = extract_user(&headers).await?;

    if user.email_is_confirmed() {
        return HttpError::bad_request("Email already confirmed")?;
    }

    commands::insert_confirmation(&user, ConfirmationAction::Email)
        .await
        .or_bad_request("Failed to send email confirmation")?;

    Ok(())
}

#[post("/api/send-password-reset-confirmation", headers: HeaderMap)]
pub async fn send_password_reset_confirmation(input: Value) -> ActionResult {
    require_no_session(&headers).await?;

    let Some(Value::String(username_or_email)) = input.get("username_or_email") else {
        return ActionError::err("Username or email is invalid", None);
    };

    let user = commands::get_user_by_username_or_email(username_or_email)
        .await
        .map_err(|_| ActionError::new("Username or email is invalid", None))?;

    let result = commands::insert_confirmation(&user, ConfirmationAction::PasswordReset).await;

    match result {
        Ok(confirmation) => ActionSuccess::ok("Password reset confirmation sent", confirmation.id.to_string().into()),
        Err(_) => ActionError::err("Failed to send password reset confirmation", None),
    }
}

#[put("/api/update-email", headers: HeaderMap)]
pub async fn update_email(input: Value) -> ActionResult {
    headers.require_app_token()?;

    let user = extract_user(&headers).await?;

    let result = commands::update_user_email(&user, &serde_json::from_value(input)?).await;

    match result {
        Ok(_) => Ok(ActionSuccess::new("Email updated successfully", Value::Null)),
        Err(errors) => Err(ActionError::new("Failed to update email", Some(errors))),
    }
}

#[put("/api/update-profile", headers: HeaderMap)]
pub async fn update_profile(input: Value) -> ActionResult {
    headers.require_app_token()?;

    let user = extract_user(&headers).await?;

    let result = commands::update_user_profile(&user, &serde_json::from_value(input)?).await;

    match result {
        Ok(_) => Ok(ActionSuccess::new("Profile updated successfully", Value::Null)),
        Err(errors) => Err(ActionError::new("Failed to update profile", Some(errors))),
    }
}
