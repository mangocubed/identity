use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use cached::AsyncRedisCache;
use cached::proc_macro::io_cached;
use chrono::Utc;
use sha2::{Digest, Sha256};
use url::Url;
use uuid::Uuid;

use toolbox::cache::redis_cache_store;

use crate::config::AUTHORIZATION_CONFIG;
use crate::constants::{CACHE_PREFIX_GET_AUTHORIZATION_BY_CODE, CACHE_PREFIX_GET_AUTHORIZATION_BY_ID};
use crate::db_pool;
use crate::models::{Application, Authorization, Session};

use super::generate_random_string;

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<String, Authorization<'_>>",
    create = r##"{ redis_cache_store(CACHE_PREFIX_GET_AUTHORIZATION_BY_CODE).await }"##
)]
pub async fn get_authorization_by_code(code: String) -> sqlx::Result<Authorization<'static>> {
    if code.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    let db_pool = db_pool().await;

    sqlx::query_as!(
        Authorization,
        "SELECT * FROM authorizations
        WHERE expires_at > current_timestamp AND revoked_at IS NULL AND code = $1
        LIMIT 1",
        code // $1
    )
    .fetch_one(db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<Uuid, Authorization<'_>>",
    create = r##"{ redis_cache_store(CACHE_PREFIX_GET_AUTHORIZATION_BY_ID).await }"##
)]
pub async fn get_authorization_by_id(id: Uuid) -> sqlx::Result<Authorization<'static>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Authorization,
        "SELECT * FROM authorizations WHERE revoked_at IS NULL AND id = $1 LIMIT 1",
        id // $1
    )
    .fetch_one(db_pool)
    .await
}

pub async fn insert_or_refresh_authorization<'a>(
    application: &Application<'_>,
    session: &Session,
    redirect_url: Url,
    code_challenge: &str,
) -> sqlx::Result<Authorization<'a>> {
    if redirect_url != application.redirect_url() {
        return Err(sqlx::Error::InvalidArgument("Invalid redirect URL".to_owned()));
    }

    let db_pool = db_pool().await;

    let user = session.user().await?;
    let code = generate_random_string(AUTHORIZATION_CONFIG.min_length..=AUTHORIZATION_CONFIG.max_length);
    let expires_at = Utc::now() + AUTHORIZATION_CONFIG.ttl();

    sqlx::query_as!(
        Authorization,
        "INSERT INTO authorizations AS a (application_id, session_id, user_id, redirect_url, code, code_challenge, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (application_id, user_id) DO UPDATE
        SET session_id = $2, redirect_url = $4, code = $5, code_challenge = $6, expires_at = $7, revoked_at = NULL
        RETURNING *",
        application.id,        // $1
        session.id,            // $2
        user.id,               // $3
        redirect_url.as_ref(), // $4
        code,                  // $5
        code_challenge,        // $6
        expires_at,            // $7
    )
    .fetch_one(db_pool)
    .await
}

pub fn verify_authorization_code_challenge(authorization: &Authorization<'_>, code_verifier: &str) -> bool {
    let mut hasher = Sha256::new();

    hasher.update(code_verifier.as_bytes());

    let code_verifier_hash = hasher.finalize();
    let code_verifier_base64 = BASE64_URL_SAFE_NO_PAD.encode(code_verifier_hash);

    authorization.code_challenge == code_verifier_base64
}
