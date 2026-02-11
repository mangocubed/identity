use chrono::NaiveDate;
use leptos::prelude::*;
use leptos::server_fn::codec::JsonEncoding;
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[cfg(feature = "ssr")]
use http::status::StatusCode;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

#[cfg(feature = "ssr")]
use identity_core::commands;
#[cfg(feature = "ssr")]
use identity_core::params::UserParams;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionError {
    pub fields: ValidationErrors,
}

impl FromServerFnError for ActionError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        serde_json::from_str(&value.to_string()).expect("Failed to deserialize error")
    }
}

type ActionResult<T = ()> = Result<T, ActionError>;

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
trait OrActionError<T> {
    fn or_action_error(self) -> ActionResult<T>;
}

#[cfg(feature = "ssr")]
impl<T> OrActionError<T> for Result<T, ValidationErrors> {
    fn or_action_error(self) -> ActionResult<T> {
        match self {
            Ok(data) => Ok(data),
            Err(fields) => {
                let resp_opts = expect_context::<ResponseOptions>();

                resp_opts.set_status(StatusCode::UNPROCESSABLE_ENTITY);

                Err(ActionError { fields })
            }
        }
    }
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
    let _ = commands::insert_user(UserParams {
        username,
        email,
        password,
        full_name,
        birthdate,
        country_code,
    })
    .await
    .or_action_error()?;

    Ok(())
}
