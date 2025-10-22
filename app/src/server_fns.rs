use dioxus::prelude::*;

#[cfg(feature = "server")]
use headers::authorization::Bearer;
#[cfg(feature = "server")]
use http::HeaderMap;

#[cfg(feature = "server")]
use identity_core::commands;
#[cfg(feature = "server")]
use identity_core::models::Session;

#[cfg(feature = "server")]
pub fn extract_bearer(headers: &HeaderMap) -> Result<Bearer, HttpError> {
    sdk::app::extract_bearer(headers).or_else(|_| HttpError::unauthorized("Unauthorized"))
}

#[cfg(feature = "server")]
pub async fn require_app_token<'a>(headers: &HeaderMap) -> Result<(), HttpError> {
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
async fn require_no_session<'a>(headers: &HeaderMap) -> Result<(), HttpError> {
    require_app_token(headers).await?;

    if extract_session(headers).await.is_err() {
        Ok(())
    } else {
        Err(HttpError::forbidden("Forbidden")?)
    }
}

#[get("/api/can-register", headers: HeaderMap)]
pub async fn can_register() -> Result<bool> {
    require_no_session(&headers).await?;

    Ok(commands::can_insert_user().await)
}
