#[cfg(feature = "ssr")]
use std::net::IpAddr;

use leptos::prelude::*;
use leptos::server_fn::codec::JsonEncoding;
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[cfg(feature = "ssr")]
use axum_client_ip::ClientIp;
#[cfg(feature = "ssr")]
use http::status::StatusCode;
#[cfg(feature = "ssr")]
use leptos_axum::{ResponseOptions, extract, redirect};
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
use identity_core::commands;
#[cfg(feature = "ssr")]
use identity_core::models::{Session, User};

#[cfg(feature = "ssr")]
use crate::constants::KEY_SESSION_ID;

mod session_server_fns;
mod user_server_fns;

pub use session_server_fns::*;
pub use user_server_fns::*;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ActionError {
    pub fields: ValidationErrors,
}

impl FromServerFnError for ActionError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        serde_json::from_str(&value.to_string()).expect("Failed to deserialize error")
    }
}

#[cfg(feature = "ssr")]
impl From<ServerFnErrorErr> for ActionError {
    fn from(_error: ServerFnErrorErr) -> Self {
        Self::default()
    }
}

#[cfg(feature = "ssr")]
impl From<ServerFnError> for ActionError {
    fn from(_error: ServerFnError) -> Self {
        Self::default()
    }
}

#[cfg(feature = "ssr")]
impl From<sqlx_core::Error> for ActionError {
    fn from(error: sqlx_core::Error) -> Self {
        let resp_opts = expect_context::<ResponseOptions>();

        match error {
            sqlx_core::Error::RowNotFound => resp_opts.set_status(StatusCode::NOT_FOUND),
            sqlx_core::Error::InvalidArgument(_) => resp_opts.set_status(StatusCode::BAD_REQUEST),
            _ => resp_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR),
        }

        Self {
            fields: ValidationErrors::new(),
        }
    }
}

#[cfg(feature = "ssr")]
impl From<tower_sessions::session::Error> for ActionError {
    fn from(_error: tower_sessions::session::Error) -> Self {
        Default::default()
    }
}

#[cfg(feature = "ssr")]
impl From<ValidationErrors> for ActionError {
    fn from(errors: ValidationErrors) -> Self {
        let resp_opts = expect_context::<ResponseOptions>();

        resp_opts.set_status(StatusCode::UNPROCESSABLE_ENTITY);

        Self { fields: errors }
    }
}

type ActionResult<T = ()> = Result<T, ActionError>;

pub type ServerFnResult<T = ()> = Result<T, ServerFnError>;

pub trait ActionResultExt {
    fn get_field_error(&self, name: &str) -> Option<String>;

    fn has_errors(&self) -> bool;

    fn is_success(&self) -> bool;
}

impl ActionResultExt for Option<ActionResult> {
    fn get_field_error(&self, name: &str) -> Option<String> {
        if let Some(Err(error)) = self {
            error
                .fields
                .field_errors()
                .get(name)
                .and_then(|error| error.first())
                .and_then(|error| error.message.clone())
                .map(|message| message.to_string())
        } else {
            None
        }
    }

    fn has_errors(&self) -> bool {
        self.as_ref().is_some_and(|result| result.is_err())
    }

    fn is_success(&self) -> bool {
        self.as_ref().is_some_and(|result| result.is_ok())
    }
}

#[cfg(feature = "ssr")]
trait OrHttpError<T> {
    fn or_unauthorized(self) -> ServerFnResult<T>;
}

#[cfg(feature = "ssr")]
impl<T> OrHttpError<T> for ServerFnResult<T> {
    fn or_unauthorized(self) -> ServerFnResult<T> {
        let resp_opts = expect_context::<ResponseOptions>();

        match self {
            Ok(data) => Ok(data),
            Err(err) => {
                resp_opts.set_status(StatusCode::UNAUTHORIZED);

                Err(err)
            }
        }
    }
}

#[cfg(feature = "ssr")]
async fn extract_client_ip() -> ServerFnResult<IpAddr> {
    let ClientIp(client_ip) = extract::<ClientIp>().await?;

    Ok(client_ip)
}

#[cfg(feature = "ssr")]
async fn extract_tower_session() -> Result<tower_sessions::Session, ServerFnErrorErr> {
    extract::<tower_sessions::Session>().await
}

#[cfg(feature = "ssr")]
async fn extract_session() -> ServerFnResult<Session> {
    let tower_session = extract_tower_session().await?;

    let Some(session_id) = tower_session.get::<Uuid>(KEY_SESSION_ID).await? else {
        return Err(ServerFnError::Request("Unauthorized".to_owned()));
    };

    Ok(commands::get_session_by_id(session_id).await?)
}

#[cfg(feature = "ssr")]
async fn extract_user<'a>() -> ServerFnResult<User<'a>> {
    let session = extract_session().await?;
    let user = session.user().await?;

    Ok(user)
}

#[cfg(feature = "ssr")]
async fn is_authenticated() -> bool {
    extract_session().await.is_ok()
}

#[cfg(feature = "ssr")]
pub async fn require_authentication() -> ServerFnResult {
    if !is_authenticated().await {
        redirect("/login");

        return Err(ServerFnError::Request("Unauthorized".to_owned()));
    }

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn require_no_authentication() -> ServerFnResult {
    if is_authenticated().await {
        redirect("/");

        return Err(ServerFnError::Request("Forbidden".to_owned()));
    }

    Ok(())
}
