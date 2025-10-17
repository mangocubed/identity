use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;

pub mod inputs;

pub mod commands;
pub mod config;
mod constants;
pub mod jobs_storage;
pub mod models;

#[cfg(test)]
mod test_utils;

use config::DATABASE_CONFIG;

static DB_POOL_CELL: OnceCell<PgPool> = OnceCell::const_new();

async fn db_pool<'a>() -> &'a PgPool {
    DB_POOL_CELL
        .get_or_init(|| async {
            PgPoolOptions::new()
                .max_connections(DATABASE_CONFIG.max_connections as u32)
                .connect(&DATABASE_CONFIG.url)
                .await
                .expect("Could not create DB pool.")
        })
        .await
}
