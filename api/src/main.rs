use std::borrow::Cow;
use std::net::SocketAddr;

use axum::Json;
use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Result};
use axum::routing::{delete, get, put};
use axum_extra::TypedHeader;
use chrono::{Duration, Utc};
use headers::authorization::{Authorization, Bearer};
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use tracing::Level;
use uuid::Uuid;

use sdk::constants::{RESPONSE_BAD_REQUEST, RESPONSE_OK, RESPONSE_UNAUTHORIZED};

use identity_core::commands;

#[derive(Deserialize)]
struct AuthParams<'a> {
    client_id: Uuid,
    client_secret: Cow<'a, str>,
    token: Cow<'a, str>,
}

async fn delete_revoke_auth(Json(params): Json<AuthParams<'_>>) -> impl IntoResponse {
    let (application, authorization) = tokio::try_join!(
        commands::authenticate_application(params.client_id, &params.client_secret),
        commands::get_authorization_by_token(&params.token),
    )
    .map_err(|_| RESPONSE_BAD_REQUEST)?;

    if application.id != authorization.application_id {
        return Err(RESPONSE_BAD_REQUEST);
    }

    let _ = commands::revoke_authorization(&authorization).await;

    Ok((StatusCode::NO_CONTENT, "\"No Content\""))
}

async fn get_verify_auth(Json(params): Json<AuthParams<'_>>) -> impl IntoResponse {
    let (application, authorization) = tokio::try_join!(
        commands::authenticate_application(params.client_id, &params.client_secret),
        commands::get_authorization_by_token(&params.token),
    )
    .map_err(|_| RESPONSE_BAD_REQUEST)?;

    if application.id != authorization.application_id {
        return Err(RESPONSE_BAD_REQUEST);
    }

    Ok(RESPONSE_OK)
}

async fn get_user_info(TypedHeader(bearer): TypedHeader<Authorization<Bearer>>) -> Result<impl IntoResponse> {
    let authorization = commands::get_authorization_by_token(bearer.token())
        .await
        .map_err(|_| RESPONSE_UNAUTHORIZED)?;

    let user = authorization.user().await;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "display_name": user.display_name,
            "initials": user.initials(),
            "full_name": user.full_name,
            "birthdate": user.birthdate,
            "language_code": user.language_code,
            "country_alpha2": user.country_alpha2,
            "created_at": user.created_at,
            "updated_at": user.updated_at,
        })),
    ))
}

async fn put_refresh_auth(Json(params): Json<AuthParams<'_>>) -> Result<impl IntoResponse> {
    let (application, authorization) = tokio::try_join!(
        commands::authenticate_application(params.client_id, &params.client_secret),
        commands::get_authorization_by_token(&params.token),
    )
    .map_err(|_| RESPONSE_BAD_REQUEST)?;

    if application.id != authorization.application_id {
        return Err(RESPONSE_BAD_REQUEST.into());
    }

    let result = commands::refresh_authorization(&authorization, Utc::now() + Duration::days(30)).await;

    match result {
        Ok(authorization) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({
                "token": authorization.token,
                "expires_at": authorization.expires_at,
                "refreshed_at": authorization.refreshed_at,
            })),
        )),
        Err(_) => Err(RESPONSE_BAD_REQUEST.into()),
    }
}

#[tokio::main]
async fn main() {
    let tracing_level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    };

    tracing_subscriber::fmt().with_max_level(tracing_level).init();

    let trace_layer = TraceLayer::new_for_http();

    let router = Router::new()
        .route("/auth/refresh", put(put_refresh_auth))
        .route("/auth/revoke", delete(delete_revoke_auth))
        .route("/user-info", get(get_user_info))
        .route("/auth/verify", get(get_verify_auth))
        .layer(trace_layer);

    let address = std::env::var("API_ADDRESS").unwrap_or("127.0.0.1:8082".to_owned());

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
