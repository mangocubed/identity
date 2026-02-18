use axum::body::Body;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Form, Json};
use http::StatusCode;
use http::header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE};
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

use identity_core::commands;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum TokenGrantType {
    AuthorizationCode,
    RefreshToken,
}

#[derive(Deserialize)]
pub struct RevokeParams {
    client_id: Uuid,
    token: String,
}

#[derive(Deserialize)]
pub struct TokenParams {
    grant_type: TokenGrantType,
    client_id: Uuid,
    code: Option<String>,
    redirect_uri: Option<Url>,
    refresh_token: Option<String>,
    code_verifier: Option<String>,
}

#[derive(Deserialize)]
pub struct AvatarImageQuery {
    size: Option<u32>,
}

pub async fn get_user_avatar_image(
    Path(user_id): Path<Uuid>,
    Query(params): Query<AvatarImageQuery>,
) -> impl IntoResponse {
    let size = params.size.unwrap_or(128);

    if size > 512 {
        return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
    }

    let Ok(user) = commands::get_user_by_id(user_id).await else {
        return Err((StatusCode::NOT_FOUND, "NOT FOUND"));
    };

    let Ok(avatar_image) = user.avatar_image(size) else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR"));
    };

    let content_length = avatar_image.len();
    let body = Body::from(avatar_image);

    let headers = [
        (CONTENT_TYPE, "image/jpeg".to_owned()),
        (CONTENT_LENGTH, content_length.to_string()),
        (
            CONTENT_DISPOSITION,
            format!("inline; filename=\"{}_{}x{}.jpg\"", user_id, size, size),
        ),
    ];

    Ok((headers, body))
}

pub async fn post_oauth_revoke(Form(params): Form<RevokeParams>) -> impl IntoResponse {
    let Ok(access_token) = commands::get_access_token_by_code(params.token).await else {
        return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
    };

    let Ok(authorization) = access_token.authorization().await else {
        return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
    };

    if params.client_id != authorization.application_id {
        return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
    }

    let _ = commands::revoke_access_token(&access_token).await;

    Ok(Json(serde_json::json!({})))
}

pub async fn post_oauth_token(Form(params): Form<TokenParams>) -> impl IntoResponse {
    let authorization = match params.grant_type {
        TokenGrantType::AuthorizationCode => {
            let Some(authorization_code) = params.code else {
                return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
            };

            let Ok(authorization) = commands::get_authorization_by_code(authorization_code).await else {
                return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
            };

            let Some(code_verifier) = params.code_verifier else {
                return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
            };

            if params.client_id != authorization.application_id
                || params.redirect_uri != Some(authorization.redirect_url())
                || !authorization.verify_code_challenge(&code_verifier)
            {
                return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
            }

            authorization
        }
        TokenGrantType::RefreshToken => {
            let Some(refresh_code) = params.refresh_token else {
                return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
            };

            let Ok(current_access_token) = commands::get_access_token_by_refresh_code(refresh_code).await else {
                return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
            };

            let Ok(authorization) = current_access_token.authorization().await else {
                return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
            };

            if params.client_id != authorization.application_id {
                return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
            }

            authorization
        }
    };

    let Ok(access_token) = commands::insert_access_token(&authorization).await else {
        return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
    };

    Ok(Json(serde_json::json!({
        "access_token": access_token.code,
        "token_type": "Bearer",
        "expires_in": access_token.code_expires_in().num_seconds(),
        "refresh_token": access_token.refresh_code,
    })))
}
