use dioxus::prelude::*;
use uuid::Uuid;

use crate::layouts::{LoginLayout, UserLayout};
use crate::pages::*;

#[derive(Clone, Routable)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Routes {
    #[layout(UserLayout)]
        #[route("/")]
        HomePage {},
        #[route("/authorize?:client_id")]
        AuthorizePage { client_id: Uuid },
    #[end_layout]

    #[layout(LoginLayout)]
        #[route("/login")]
        LoginPage {},
        #[route("/register")]
        RegisterPage {},
}

impl Routes {
    pub fn home() -> Self {
        Self::HomePage {}
    }

    pub fn login() -> Self {
        Self::LoginPage {}
    }

    pub fn register() -> Self {
        Self::RegisterPage {}
    }
}

#[cfg(feature = "server")]
pub mod priv_api {
    use std::borrow::Cow;

    use axum::Json;
    use axum::extract::Form;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum_extra::TypedHeader;
    use chrono::{Duration, Utc};
    use headers::authorization::{Authorization, Bearer};
    use serde::Deserialize;
    use uuid::Uuid;

    use identity_core::commands;

    #[derive(Deserialize)]
    pub struct AuthParams<'a> {
        client_id: Uuid,
        client_secret: Cow<'a, str>,
        token: Cow<'a, str>,
    }

    pub async fn get_verify_auth(TypedHeader(bearer): TypedHeader<Authorization<Bearer>>) -> impl IntoResponse {
        let exists = commands::authorization_exists(bearer.token()).await;

        if exists {
            (StatusCode::OK, "OK")
        } else {
            (StatusCode::UNAUTHORIZED, "INVALID")
        }
    }

    pub async fn get_user_info(TypedHeader(bearer): TypedHeader<Authorization<Bearer>>) -> impl IntoResponse {
        let Ok(authorization) = commands::get_authorization_by_token(bearer.token()).await else {
            return Err((StatusCode::UNAUTHORIZED, "UNAUTHORIZED"));
        };

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

    pub async fn post_refresh_auth(Form(params): Form<AuthParams<'_>>) -> impl IntoResponse {
        let Ok((application, authorization)) = futures::future::try_join(
            commands::authenticate_application(params.client_id, &params.client_secret),
            commands::get_authorization_by_token(&params.token),
        )
        .await
        else {
            return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
        };

        if application.id != authorization.application_id {
            return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
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
            Err(_) => Err((StatusCode::BAD_REQUEST, "BAD REQUEST")),
        }
    }
}
