use std::net::IpAddr;

use cached::AsyncRedisCache;
use cached::proc_macro::io_cached;
use uuid::Uuid;

use crate::constants::CACHE_PREFIX_GET_SESSION_BY_ID;
use crate::models::{Session, User};
use crate::{db_pool, jobs_storage};

use super::{AsyncRedisCacheExt, async_redis_cache};

pub async fn finish_session(session: &Session) -> sqlx::Result<()> {
    let db_pool = db_pool().await;

    sqlx::query!(
        "UPDATE sessions SET finished_at = current_timestamp WHERE finished_at IS NULL AND id = $1",
        session.id
    )
    .execute(db_pool)
    .await?;

    remove_session_cache(session).await;

    jobs_storage().await.push_finished_session(session).await;

    Ok(())
}

pub async fn get_finished_session_by_id(id: Uuid) -> sqlx::Result<Session> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Session,
        "SELECT * FROM sessions WHERE (expires_at <= current_timestamp OR finished_at IS NOT NULL) AND id = $1 LIMIT 1",
        id
    )
    .fetch_one(db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<Uuid, Session>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_SESSION_BY_ID).await }"##
)]
pub async fn get_session_by_id(id: Uuid) -> sqlx::Result<Session> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Session,
        "SELECT * FROM sessions WHERE expires_at > current_timestamp AND finished_at IS NULL AND id = $1 LIMIT 1",
        id
    )
    .fetch_one(db_pool)
    .await
}

pub async fn insert_session(user: &User<'_>, ip_address: IpAddr) -> sqlx::Result<Session> {
    let db_pool = db_pool().await;

    let result = sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id, ip_address) VALUES ($1, $2) RETURNING *",
        user.id,                // $1
        ip_address.to_string(), // $2
    )
    .fetch_one(db_pool)
    .await;

    match result {
        Ok(session) => {
            jobs_storage().await.push_new_session(&session).await;

            Ok(session)
        }
        Err(err) => Err(err),
    }
}

pub async fn refresh_session(session: &Session) -> sqlx::Result<()> {
    let db_pool = db_pool().await;

    sqlx::query!(
        "UPDATE sessions
        SET refreshed_at = current_timestamp, expires_at = current_timestamp + INTERVAL '30 days'
        WHERE id = $1",
        session.id, // $1
    )
    .execute(db_pool)
    .await?;

    remove_session_cache(session).await;

    Ok(())
}

pub async fn update_session_location(
    session: &Session,
    country_code: &str,
    region: &str,
    city: &str,
) -> sqlx::Result<Session> {
    let db_pool = db_pool().await;

    let session = sqlx::query_as!(
        Session,
        "UPDATE sessions SET country_code = $2, region = $3, city = $4 WHERE finished_at IS NULL AND id = $1 RETURNING *",
        session.id,   // $1
        country_code, // $2
        region,       // $3
        city          // $4
    )
    .fetch_one(db_pool)
    .await?;

    remove_session_cache(&session).await;

    Ok(session)
}

async fn remove_session_cache(session: &Session) {
    GET_SESSION_BY_ID
        .cache_remove(CACHE_PREFIX_GET_SESSION_BY_ID, &session.id)
        .await;
}
