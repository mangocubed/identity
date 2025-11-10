#[cfg(feature = "server")]
use std::net::{IpAddr, SocketAddr};

use dioxus::prelude::*;
use serde_json::Value;

#[cfg(feature = "server")]
use axum::extract::ConnectInfo;
#[cfg(feature = "server")]
use headers::authorization::Bearer;
#[cfg(feature = "server")]
use http::HeaderMap;

use sdk::app::ActionResult;

#[cfg(feature = "server")]
use sdk::app::{ActionError, ActionSuccess};
#[cfg(feature = "server")]
use sdk::constants::{HEADER_USER_AGENT, HEADER_X_REAL_IP};

#[cfg(feature = "server")]
use identity_core::commands;
#[cfg(feature = "server")]
use identity_core::models::{Session, User};

use crate::presenters::UserPresenter;

#[cfg(feature = "server")]
pub fn extract_bearer(headers: &HeaderMap) -> Result<Bearer, HttpError> {
    sdk::app::extract_bearer(headers).or_else(|_| HttpError::unauthorized("Unauthorized"))
}

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
pub async fn require_app_token(headers: &HeaderMap) -> Result<(), HttpError> {
    sdk::app::require_app_token(headers)
        .await
        .or_else(|_| HttpError::forbidden("Forbidden"))
}

#[cfg(feature = "server")]
async fn extract_session<'a>(headers: &HeaderMap) -> Result<Session<'a>, HttpError> {
    let bearer = extract_bearer(headers)?;

    commands::get_session_by_token(bearer.token())
        .await
        .or_forbidden("Forbidden")
}

#[cfg(feature = "server")]
async fn extract_user<'a>(headers: &HeaderMap) -> Result<User<'a>, HttpError> {
    let bearer = extract_bearer(headers)?;

    commands::get_user_by_session_token(bearer.token())
        .await
        .or_else(|_| HttpError::unauthorized("Unauthorized"))
}

#[cfg(feature = "server")]
async fn require_no_session(headers: &HeaderMap) -> Result<(), HttpError> {
    require_app_token(headers).await?;

    if extract_session(headers).await.is_err() {
        Ok(())
    } else {
        Err(HttpError::forbidden("Forbidden")?)
    }
}

#[put("/api/change-password", headers: HeaderMap)]
pub async fn change_password(input: Value) -> ActionResult {
    require_app_token(&headers).await?;

    let user = extract_user(&headers).await?;

    let result = commands::update_user_password(&user, &serde_json::from_value(input)?).await;

    match result {
        Ok(_) => Ok(ActionSuccess::new("Password changed successfully", Value::Null)),
        Err(errors) => Err(ActionError::new("Failed to change password", Some(errors))),
    }
}

#[put("/api/confirm-email", headers: HeaderMap)]
pub async fn confirm_email(input: Value) -> ActionResult {
    require_app_token(&headers).await?;

    let user = extract_user(&headers).await?;

    let result = commands::confirm_user_email(&user, &serde_json::from_value(input)?).await;

    match result {
        Ok(_) => Ok(ActionSuccess::new("Email confirmed successfully", Value::Null)),
        Err(errors) => Err(ActionError::new("Failed to confirm email", Some(errors))),
    }
}

#[get("/api/current-user", headers: HeaderMap)]
pub async fn current_user() -> Result<UserPresenter> {
    require_app_token(&headers).await?;

    let user = extract_user(&headers).await?;

    Ok(UserPresenter::from(user))
}

#[get("/api/can-register", headers: HeaderMap)]
pub async fn can_register() -> Result<bool> {
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
            session.token.into(),
        )),
        Err(_) => Err(ActionError::new("Failed to authenticate user", None)),
    }
}

#[delete("/api/logout", headers: HeaderMap)]
pub async fn logout() -> Result<()> {
    let session = extract_session(&headers).await?;

    commands::finish_session(&session)
        .await
        .or_internal_server_error("Internal server error")?;

    Ok(())
}

#[post("/api/register", headers: HeaderMap, connect_info: ConnectInfo<SocketAddr>)]
pub async fn register(input: Value) -> ActionResult {
    require_no_session(&headers).await?;

    let result = commands::insert_user(&serde_json::from_value(input)?).await;

    match result {
        Ok(user) => {
            let user_agent = extract_user_agent(&headers);
            let ip_addr = extract_client_ip_addr(&headers, connect_info);

            let result = commands::insert_session(&user, &user_agent, ip_addr).await;

            let session_token = result.ok().map(|session| session.token);

            Ok(ActionSuccess::new("User created successfully", session_token.into()))
        }
        Err(errors) => Err(ActionError::new("Failed to create user", Some(errors))),
    }
}

#[post("/api/send-email-confirmation", headers: HeaderMap)]
pub async fn send_email_confirmation() -> Result<()> {
    use identity_core::enums::ConfirmationAction;

    require_app_token(&headers).await?;

    let user = extract_user(&headers).await?;

    if user.email_is_confirmed() {
        return HttpError::bad_request("Email already confirmed")?;
    }

    commands::insert_confirmation(&user, ConfirmationAction::Email)
        .await
        .or_bad_request("Failed to send email confirmation")?;

    Ok(())
}

#[put("/api/update-email", headers: HeaderMap)]
pub async fn update_email(input: Value) -> ActionResult {
    require_app_token(&headers).await?;

    let user = extract_user(&headers).await?;

    let result = commands::update_user_email(&user, &serde_json::from_value(input)?).await;

    match result {
        Ok(_) => Ok(ActionSuccess::new("Email updated successfully", Value::Null)),
        Err(errors) => Err(ActionError::new("Failed to update email", Some(errors))),
    }
}
