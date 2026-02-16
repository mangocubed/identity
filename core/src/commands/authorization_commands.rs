use chrono::Utc;
use url::Url;

use crate::config::AUTHORIZATION_CONFIG;
use crate::db_pool;
use crate::models::{Application, Authorization, Session};

use super::generate_random_string;

pub async fn insert_or_refresh_authorization<'a>(
    application: &Application<'_>,
    session: &Session,
    redirect_url: Url,
    code_challenge: &str,
) -> sqlx::Result<Authorization<'a>> {
    if session.finished_at.is_some() {
        return Err(sqlx::Error::InvalidArgument("Invalid session".to_owned()));
    }

    if redirect_url.to_string() != application.redirect_url {
        return Err(sqlx::Error::InvalidArgument("Invalid redirect URL".to_owned()));
    }

    let db_pool = db_pool().await;

    let code = generate_random_string(AUTHORIZATION_CONFIG.code_min_length..=AUTHORIZATION_CONFIG.code_max_length);
    let expires_at = Utc::now() + AUTHORIZATION_CONFIG.ttl();

    sqlx::query_as!(
        Authorization,
        "INSERT INTO authorizations AS a (application_id, session_id, redirect_url, code, code_challenge, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (application_id, session_id) DO UPDATE
        SET redirect_url = $3, code = $4, code_challenge = $5, expires_at = $6, revoked_at = NULL
        RETURNING *",
        application.id,        // $1
        session.id,            // $2
        redirect_url.as_ref(), // $3
        code,                  // $4
        code_challenge,        // $5
        expires_at,            // $6
    )
    .fetch_one(db_pool)
    .await
}
