use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;

mod config;
mod models;

use crate::config::DATABASE_CONFIG;

static DB_POOL_CELL: OnceCell<PgPool> = OnceCell::const_new();

#[allow(dead_code)]
async fn db_pool<'a>() -> &'a PgPool {
    DB_POOL_CELL
        .get_or_init(|| async {
            PgPoolOptions::new()
                .max_connections(DATABASE_CONFIG.max_connections)
                .connect(&DATABASE_CONFIG.url)
                .await
                .expect("Could not create DB pool.")
        })
        .await
}
