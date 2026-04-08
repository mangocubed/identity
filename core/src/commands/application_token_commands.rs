use cached::AsyncRedisCache;
use cached::proc_macro::io_cached;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::config::APPLICATION_TOKEN_CONFIG;
use crate::constants::{CACHE_PREFIX_GET_APPLICATION_TOKEN_BY_CODE, CACHE_PREFIX_GET_APPLICATION_TOKEN_BY_ID};
use crate::db_pool;
use crate::models::{Application, ApplicationToken};
use crate::params::ApplicationTokenParams;

use super::{AsyncRedisCacheExt, OrValidationErrors, ValidationResult, async_redis_cache, generate_random_string};

pub async fn all_application_tokens<'a>(application: &Application<'_>) -> sqlx::Result<Vec<ApplicationToken<'a>>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        ApplicationToken,
        "SELECT * FROM application_tokens
        WHERE application_id = $1 AND expires_at > current_timestamp AND revoked_at IS NULL",
        application.id // $1
    )
    .fetch_all(db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<String, ApplicationToken<'_>>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_APPLICATION_TOKEN_BY_CODE).await }"##
)]
pub async fn get_application_token_by_code(code: String) -> sqlx::Result<ApplicationToken<'static>> {
    if code.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    let db_pool = db_pool().await;

    sqlx::query_as!(
        ApplicationToken,
        "SELECT * FROM application_tokens
        WHERE code = $1 AND expires_at > current_timestamp AND revoked_at IS NULL
        LIMIT 1",
        code // $1
    )
    .fetch_one(db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<Uuid, ApplicationToken<'_>>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_APPLICATION_TOKEN_BY_ID).await }"##
)]
pub async fn get_application_token_by_id(id: Uuid) -> sqlx::Result<ApplicationToken<'static>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        ApplicationToken,
        "SELECT * FROM application_tokens
        WHERE id = $1 AND expires_at > current_timestamp AND revoked_at IS NULL
        LIMIT 1",
        id // $1
    )
    .fetch_one(db_pool)
    .await
}

pub async fn insert_application_token<'a>(
    application: &Application<'_>,
    params: ApplicationTokenParams,
) -> ValidationResult<ApplicationToken<'a>> {
    params.validate()?;

    let db_pool = db_pool().await;

    let code = generate_random_string(APPLICATION_TOKEN_CONFIG.min_length..=APPLICATION_TOKEN_CONFIG.max_length);
    let expires_at = params
        .expires_at
        .map(|date| date.and_time(Utc::now().time()).and_utc())
        .unwrap_or_else(|| Utc::now() + APPLICATION_TOKEN_CONFIG.ttl());

    let access_token = sqlx::query_as!(
        ApplicationToken,
        "INSERT INTO application_tokens (application_id, name, code, expires_at) VALUES ($1, $2, $3, $4) RETURNING *",
        application.id, // $1
        params.name,    // $2
        code,           // $3
        expires_at,     // $4
    )
    .fetch_one(db_pool)
    .await
    .or_validation_errors()?;

    Ok(access_token)
}

pub async fn revoke_application_token(application_token: &ApplicationToken<'_>) -> sqlx::Result<()> {
    if application_token.revoked_at.is_some() {
        return Ok(());
    }

    let db_pool = db_pool().await;

    sqlx::query!(
        "UPDATE application_tokens SET revoked_at = current_timestamp WHERE id = $1",
        application_token.id // $1
    )
    .execute(db_pool)
    .await?;

    remove_application_token_cache(application_token).await;

    Ok(())
}

async fn remove_application_token_cache(application_token: &ApplicationToken<'_>) {
    let code = application_token.code.to_string();

    GET_APPLICATION_TOKEN_BY_CODE
        .cache_remove(CACHE_PREFIX_GET_APPLICATION_TOKEN_BY_CODE, &code)
        .await;
}
