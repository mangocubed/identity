use axum::body::Body;
use axum::extract::{Path, Query};
use axum::response::{IntoResponse, Result};
use axum::{Form, Json};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use chrono::{DateTime, Utc};
use http::StatusCode;
use http::header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE};
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use identity_core::models::User;
use identity_core::{Info, commands};

use crate::constants::*;
use crate::params::{AvatarImageParams, RevokeParams, TokenGrantType, TokenParams};

type AuthorizationBearer = TypedHeader<Authorization<Bearer>>;

trait OrHttpError<T> {
    #[allow(clippy::result_large_err)]
    fn or_bad_request(self) -> Result<T>;

    #[allow(clippy::result_large_err, dead_code)]
    fn or_forbidden(self) -> Result<T>;

    #[allow(clippy::result_large_err)]
    fn or_internal_server_error(self) -> Result<T>;

    #[allow(clippy::result_large_err)]
    fn or_not_found(self) -> Result<T>;

    #[allow(clippy::result_large_err)]
    fn or_unauthorized(self) -> Result<T>;
}

impl<T> OrHttpError<T> for Option<T> {
    fn or_bad_request(self) -> Result<T> {
        self.ok_or_else(|| ERROR_BAD_REQUEST.into())
    }

    fn or_forbidden(self) -> Result<T> {
        self.ok_or_else(|| ERROR_FORBIDDEN.into())
    }

    fn or_internal_server_error(self) -> Result<T> {
        self.ok_or_else(|| ERROR_INTERNAL_SERVER_ERROR.into())
    }

    fn or_not_found(self) -> Result<T> {
        self.ok_or_else(|| ERROR_NOT_FOUND.into())
    }

    fn or_unauthorized(self) -> Result<T> {
        self.ok_or_else(|| ERROR_UNAUTHORIZED.into())
    }
}

impl<T, E> OrHttpError<T> for Result<T, E> {
    fn or_bad_request(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(ERROR_BAD_REQUEST.into()),
        }
    }

    fn or_forbidden(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(ERROR_FORBIDDEN.into()),
        }
    }

    fn or_internal_server_error(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(ERROR_INTERNAL_SERVER_ERROR.into()),
        }
    }

    fn or_not_found(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(ERROR_NOT_FOUND.into()),
        }
    }

    fn or_unauthorized(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(ERROR_UNAUTHORIZED.into()),
        }
    }
}

#[derive(Serialize)]
pub struct UserJson {
    id: Uuid,
    username: String,
    email: String,
    display_name: String,
    initials: String,
    language_code: String,
    country_code: String,
    avatar_image_url: Url,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl From<User<'_>> for UserJson {
    fn from(user: User<'_>) -> Self {
        Self {
            id: user.id,
            username: user.username.to_string(),
            email: user.email.to_string(),
            display_name: user.display_name.to_string(),
            initials: user.initials(),
            language_code: user.language_code.to_string(),
            country_code: user.country_code.to_string(),
            avatar_image_url: user.avatar_image_url(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

async fn require_token(bearer: Option<AuthorizationBearer>) -> Result<impl IntoResponse> {
    let Some(TypedHeader(Authorization(bearer))) = bearer else {
        return Err(ERROR_UNAUTHORIZED.into());
    };

    let code = bearer.token().to_owned();

    commands::get_user_by_access_token_code(code.clone())
        .await
        .map(|_| ())
        .or(commands::get_application_token_by_code(code).await.map(|_| ()))
        .or_unauthorized()
}

pub async fn get_authorized(authorization: Option<AuthorizationBearer>) -> Result<impl IntoResponse> {
    require_token(authorization)
        .await
        .map(|_| (StatusCode::OK, "\"Authorized\""))
}

pub async fn get_current_user(authorization: Option<AuthorizationBearer>) -> Result<impl IntoResponse> {
    let Some(TypedHeader(Authorization(bearer))) = authorization else {
        return Err(ERROR_UNAUTHORIZED.into());
    };

    let code = bearer.token().to_owned();

    let user = commands::get_user_by_access_token_code(code).await.or_unauthorized()?;

    Ok(Json(UserJson::from(user)))
}

pub async fn get_index() -> impl IntoResponse {
    Json(Info::default())
}

pub async fn get_user(
    bearer: Option<AuthorizationBearer>,
    Path(username_or_id): Path<String>,
) -> Result<impl IntoResponse> {
    require_token(bearer).await?;

    let user = commands::get_user_by_username_or_id(&username_or_id)
        .await
        .or_not_found()?;

    Ok(Json(UserJson::from(user)))
}

pub async fn get_user_avatar_image(
    Path(username_or_id): Path<String>,
    Query(params): Query<AvatarImageParams>,
) -> Result<impl IntoResponse> {
    let size = params.size.unwrap_or(128);

    if size > 512 {
        return Err(ERROR_BAD_REQUEST.into());
    }

    let user = commands::get_user_by_username_or_id(&username_or_id)
        .await
        .or_not_found()?;

    let avatar_image = user.avatar_image(size).or_internal_server_error()?;

    let content_length = avatar_image.len();
    let body = Body::from(avatar_image);

    let headers = [
        (CONTENT_TYPE, "image/jpeg".to_owned()),
        (CONTENT_LENGTH, content_length.to_string()),
        (
            CONTENT_DISPOSITION,
            format!("inline; filename=\"{}_{}x{}.jpg\"", username_or_id, size, size),
        ),
    ];

    Ok((headers, body))
}

pub async fn post_oauth_revoke(Form(params): Form<RevokeParams>) -> Result<impl IntoResponse> {
    let access_token = commands::get_access_token_by_code(params.token)
        .await
        .or_bad_request()?;

    if params.client_id != access_token.application_id {
        return Err(ERROR_BAD_REQUEST.into());
    }

    let _ = commands::revoke_access_token(&access_token).await;

    Ok(Json(serde_json::json!({})))
}

pub async fn post_oauth_token(Form(params): Form<TokenParams>) -> Result<impl IntoResponse> {
    let (application, authorization, session) = match params.grant_type {
        TokenGrantType::AuthorizationCode => {
            let authorization_code = params.code.or_bad_request()?;

            let authorization = commands::get_authorization_by_code(authorization_code)
                .await
                .or_bad_request()?;

            let code_verifier = params.code_verifier.or_bad_request()?;

            if params.client_id != authorization.application_id
                || params.redirect_uri != Some(authorization.redirect_url())
                || !authorization.verify_code_challenge(&code_verifier)
            {
                return Err(ERROR_BAD_REQUEST.into());
            }

            let application = authorization.application().await.or_internal_server_error()?;
            let session = authorization.session().await.or_bad_request()?;

            (application, authorization, session)
        }
        TokenGrantType::RefreshToken => {
            let refresh_code = params.refresh_token.or_bad_request()?;

            let current_access_token = commands::get_access_token_by_refresh_code(refresh_code)
                .await
                .or_bad_request()?;

            if params.client_id != current_access_token.application_id {
                return Err(ERROR_BAD_REQUEST.into());
            }

            let application = current_access_token.application().await.or_internal_server_error()?;
            let authorization = current_access_token.authorization().await.or_bad_request()?;
            let session = current_access_token.session().await.or_bad_request()?;

            (application, authorization, session)
        }
    };

    let access_token = commands::insert_access_token(&application, &authorization, &session)
        .await
        .or_bad_request()?;

    Ok(Json(serde_json::json!({
        "access_token": access_token.code,
        "token_type": "Bearer",
        "expires_in": access_token.code_expires_in().num_seconds(),
        "refresh_token": access_token.refresh_code,
    })))
}
