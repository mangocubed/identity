use cached::AsyncRedisCache;
use cached::proc_macro::io_cached;
use chrono::Utc;

use crate::config::ACCESS_TOKEN_CONFIG;
use crate::constants::*;
use crate::db_pool;
use crate::models::{AccessToken, Application, Authorization, Session};

use super::*;

pub async fn all_access_tokens_by_session(session: &Session) -> sqlx::Result<Vec<AccessToken<'_>>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        AccessToken,
        "SELECT * FROM access_tokens
        WHERE expires_at > current_timestamp AND revoked_at IS NULL AND session_id = $1",
        session.id
    )
    .fetch_all(db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<String, AccessToken<'_>>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_ACCESS_TOKEN_BY_CODE).await }"##
)]
pub async fn get_access_token_by_code(code: String) -> sqlx::Result<AccessToken<'static>> {
    if code.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    let db_pool = db_pool().await;

    sqlx::query_as!(
        AccessToken,
        "SELECT * FROM access_tokens
        WHERE code_expires_at > current_timestamp AND revoked_at IS NULL AND code = $1
        LIMIT 1",
        code // $1
    )
    .fetch_one(db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<String, AccessToken<'_>>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_ACCESS_TOKEN_BY_REFRESH_CODE).await }"##
)]
pub async fn get_access_token_by_refresh_code(refresh_code: String) -> sqlx::Result<AccessToken<'static>> {
    if refresh_code.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    let db_pool = db_pool().await;

    sqlx::query_as!(
        AccessToken,
        "SELECT * FROM access_tokens
        WHERE expires_at > current_timestamp AND revoked_at IS NULL AND refresh_code = $1
        LIMIT 1",
        refresh_code // $1
    )
    .fetch_one(db_pool)
    .await
}

pub async fn insert_access_token<'a>(
    application: &Application<'_>,
    authorization: &Authorization<'_>,
    session: &Session,
) -> sqlx::Result<AccessToken<'a>> {
    if authorization.user_id != session.user_id {
        return Err(sqlx::Error::InvalidArgument("Invalid authorization".to_owned()));
    }

    let db_pool = db_pool().await;

    let user = session.user().await?;
    let code = generate_random_string(ACCESS_TOKEN_CONFIG.min_length..=ACCESS_TOKEN_CONFIG.max_length);
    let refresh_code = generate_random_string(ACCESS_TOKEN_CONFIG.min_length..=ACCESS_TOKEN_CONFIG.max_length);
    let code_expires_at = Utc::now() + ACCESS_TOKEN_CONFIG.code_ttl();
    let expires_at = Utc::now() + ACCESS_TOKEN_CONFIG.ttl();

    let access_token = sqlx::query_as!(
        AccessToken,
        "INSERT INTO access_tokens (
            application_id, authorization_id, session_id, user_id, code, refresh_code, code_expires_at, expires_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        application.id,   // $1
        authorization.id, // $2
        session.id,       // $3
        user.id,          // $4
        code,             // $5
        refresh_code,     // $6
        code_expires_at,  // $7
        expires_at,       // $8
    )
    .fetch_one(db_pool)
    .await?;

    if session.should_refresh() {
        let _ = refresh_session(session).await;
    }

    Ok(access_token)
}

pub async fn revoke_access_token(access_token: &AccessToken<'_>) -> sqlx::Result<()> {
    if access_token.revoked_at.is_some() {
        return Ok(());
    }

    let db_pool = db_pool().await;

    sqlx::query!(
        "UPDATE access_tokens SET revoked_at = current_timestamp WHERE id = $1",
        access_token.id // $1
    )
    .execute(db_pool)
    .await?;

    remove_access_token_cache(access_token).await;

    Ok(())
}

async fn remove_access_token_cache(access_token: &AccessToken<'_>) {
    let code = access_token.code.to_string();
    let refresh_code = access_token.refresh_code.to_string();

    tokio::join!(
        GET_ACCESS_TOKEN_BY_CODE.cache_remove(CACHE_PREFIX_GET_ACCESS_TOKEN_BY_CODE, &code),
        GET_ACCESS_TOKEN_BY_REFRESH_CODE.cache_remove(CACHE_PREFIX_GET_ACCESS_TOKEN_BY_REFRESH_CODE, &refresh_code),
        GET_USER_BY_ACCESS_TOKEN_CODE.cache_remove(CACHE_PREFIX_GET_USER_BY_ACCESS_TOKEN_CODE, &code)
    );
}
