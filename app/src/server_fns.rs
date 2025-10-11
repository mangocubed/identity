#[cfg(feature = "server")]
use std::net::IpAddr;

use dioxus::prelude::*;
use url::Url;
use uuid::Uuid;

use sdk::serv_fn::{FormResult, ServFnClient, ServFnResult};

#[cfg(feature = "server")]
use sdk::serv_fn::{FormError, FormSuccess, ServFnError, extract_bearer, extract_header, require_app_token};

use identity_core::inputs::{LoginInput, RegisterInput};

#[cfg(feature = "server")]
use identity_core::models::{Session, User};

#[cfg(feature = "server")]
use identity_core::commands;

use crate::presenters::UserPresenter;

#[cfg(feature = "server")]
use crate::constants::{HEADER_REAL_IP, HEADER_USER_AGENT};

#[cfg(feature = "server")]
async fn extract_client_ip_addr() -> ServFnResult<IpAddr> {
    use std::net::SocketAddr;

    use axum::extract::ConnectInfo;

    let real_ip = extract_header(HEADER_REAL_IP).await?.map(|ip| ip.parse());

    if let Some(Ok(real_ip)) = real_ip {
        return Ok(real_ip);
    }

    let ConnectInfo(addr) = extract::<ConnectInfo<SocketAddr>, _>()
        .await
        .map_err(|_| ServFnError::internal_server())?;

    Ok(addr.ip())
}

#[cfg(feature = "server")]
async fn extract_session<'a>() -> ServFnResult<Option<Session<'a>>> {
    if let Some(bearer) = extract_bearer().await? {
        Ok(commands::get_session_by_token(bearer.token()).await.ok())
    } else {
        Ok(None)
    }
}

#[cfg(feature = "server")]
async fn extract_user<'a>() -> ServFnResult<Option<User<'a>>> {
    if let Some(bearer) = extract_bearer().await? {
        Ok(commands::get_user_by_session_token(bearer.token()).await.ok())
    } else {
        Ok(None)
    }
}

#[cfg(feature = "server")]
async fn extract_user_agent() -> ServFnResult<String> {
    let user_agent = extract_header(HEADER_USER_AGENT).await?.unwrap_or("Unknown".to_owned());

    Ok(user_agent)
}

#[cfg(feature = "server")]
pub async fn is_logged_in() -> ServFnResult<bool> {
    require_app_token().await?;

    Ok(extract_session().await?.is_some())
}

#[cfg(feature = "server")]
async fn require_login() -> ServFnResult<()> {
    if is_logged_in().await? {
        Ok(())
    } else {
        Err(ServFnError::unauthorized().into())
    }
}

#[cfg(feature = "server")]
async fn require_no_login() -> ServFnResult<()> {
    if !is_logged_in().await? {
        Ok(())
    } else {
        Err(ServFnError::forbidden().into())
    }
}

#[server(client = ServFnClient)]
pub async fn attempt_to_authorize(client_id: Uuid) -> ServFnResult<Url> {
    use chrono::{Duration, Utc};

    require_login().await?;

    let application = commands::get_application_by_id(client_id)
        .await
        .map_err(|_| ServFnError::not_found())?;

    let session = extract_session().await?.expect("Could not get session");
    let user = extract_user().await?.expect("Could not get user");

    let authorization =
        commands::insert_or_refresh_authorization(&application, &user, &session, Utc::now() + Duration::hours(1))
            .await
            .expect("Could not get authorization");

    let mut redirect_url = application.redirect_url();

    redirect_url.set_query(Some(&format!(
        "token={}&expires_at={}",
        authorization.token, authorization.expires_at
    )));

    Ok(redirect_url)
}

#[server(client = ServFnClient)]
pub async fn attempt_to_login(input: LoginInput) -> FormResult {
    require_no_login().await.map_err(FormError::from)?;

    let user = {
        let result = commands::authenticate_user(&input).await;

        match result {
            Ok(user) => user,
            Err(errors) => {
                return Err(FormError::new("Failed to authenticate user", Some(errors)).into());
            }
        }
    };

    let user_agent = extract_user_agent().await.map_err(FormError::from)?;
    let ip_addr = extract_client_ip_addr().await.map_err(FormError::from)?;

    let result = commands::insert_session(&user, &user_agent, ip_addr).await;

    match result {
        Ok(session) => Ok(FormSuccess::new(
            "User authenticated successfully",
            session.token.into(),
        )),
        Err(_) => Err(FormError::new("Failed to authenticate user", None).into()),
    }
}

#[server(client = ServFnClient)]
pub async fn attempt_to_logout() -> ServFnResult {
    require_login().await?;

    let Some(session) = extract_session().await? else {
        return Ok(());
    };

    let _ = commands::finish_session(&session).await;

    Ok(())
}

#[server(client = ServFnClient)]
pub async fn attempt_to_register(input: RegisterInput) -> FormResult {
    require_no_login().await.map_err(FormError::from)?;

    let result = commands::insert_user(&input).await;

    match result {
        Ok(user) => {
            let user_agent = extract_user_agent().await.map_err(FormError::from)?;
            let ip_addr = extract_client_ip_addr().await.map_err(FormError::from)?;

            let result = commands::insert_session(&user, &user_agent, ip_addr).await;

            let session_token = result.ok().map(|session| session.token);

            Ok(FormSuccess::new("User created successfully", session_token.into()))
        }
        Err(errors) => Err(FormError::new("Failed to create user", Some(errors)).into()),
    }
}

#[server(client = ServFnClient)]
pub async fn can_register_user() -> ServFnResult<bool> {
    require_no_login().await?;

    Ok(commands::can_insert_user().await)
}

#[server(client = ServFnClient)]
pub async fn get_current_user() -> ServFnResult<Option<UserPresenter>> {
    require_app_token().await?;

    let Some(user) = extract_user().await? else {
        return Ok(None);
    };

    Ok(Some(user.into()))
}
