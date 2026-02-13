use leptos::prelude::*;

#[cfg(feature = "ssr")]
use identity_core::commands;
#[cfg(feature = "ssr")]
use identity_core::params::AuthenticationParams;

#[cfg(feature = "ssr")]
use crate::constants::KEY_SESSION_ID;

use super::ActionResult;

#[cfg(feature = "ssr")]
use super::*;

#[server]
pub async fn create_session(username_or_email: String, password: String) -> ActionResult {
    require_no_authentication().await?;

    let user = commands::authenticate_user(AuthenticationParams {
        username_or_email,
        password,
    })
    .await?;

    let client_ip = extract_client_ip().await?;

    let user_session = commands::insert_session(&user, client_ip).await?;

    let tower_session = extract_tower_session().await?;

    tower_session.insert(KEY_SESSION_ID, user_session.id).await?;

    Ok(())
}

#[server]
pub async fn finish_session() -> ActionResult {
    require_authentication().await?;

    let session = extract_session().await?;

    commands::finish_session(&session).await?;

    let tower_session = extract_tower_session().await?;

    tower_session.remove::<String>(KEY_SESSION_ID).await?;

    Ok(())
}
