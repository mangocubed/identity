use chrono::NaiveDate;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use identity_core::commands;
#[cfg(feature = "ssr")]
use identity_core::params::UserParams;

use crate::presenters::UserPresenter;

#[cfg(feature = "ssr")]
use crate::server_fns::KEY_SESSION_ID;

use super::{ActionResult, ServerFnResult};

#[cfg(feature = "ssr")]
use super::{OrHttpError, extract_client_ip, extract_tower_session, extract_user, require_no_authentication};

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
