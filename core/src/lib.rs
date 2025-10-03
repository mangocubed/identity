#[cfg(feature = "server")]
use sqlx::PgPool;
#[cfg(feature = "server")]
use sqlx::postgres::PgPoolOptions;
#[cfg(feature = "server")]
use tokio::sync::OnceCell;

pub mod inputs;

#[cfg(feature = "server")]
pub mod commands;
#[cfg(feature = "server")]
mod config;
#[cfg(feature = "server")]
mod constants;
#[cfg(feature = "server")]
pub mod models;

#[cfg(test)]
mod test_utils;

#[cfg(feature = "server")]
use crate::config::DATABASE_CONFIG;

#[cfg(feature = "server")]
static DB_POOL_CELL: OnceCell<PgPool> = OnceCell::const_new();

#[cfg(feature = "server")]
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
