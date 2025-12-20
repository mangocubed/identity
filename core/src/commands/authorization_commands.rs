use crate::db_pool;
use crate::models::{Authorization, Session};

pub async fn all_authorizations_by_session<'a>(session: &Session<'_>) -> sqlx::Result<Vec<Authorization<'a>>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Authorization,
        "SELECT * FROM authorizations WHERE session_id = $1 AND revoked_at IS NULL",
        session.id
    )
    .fetch_all(db_pool)
    .await
}

pub async fn revoke_authorization<'a>(authorization: &Authorization<'_>) -> sqlx::Result<Authorization<'a>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Authorization,
        "UPDATE authorizations AS a SET revoked_at = current_timestamp WHERE id = $1 AND revoked_at IS NULL
        RETURNING *",
        authorization.id, // $1
    )
    .fetch_one(db_pool)
    .await
}
