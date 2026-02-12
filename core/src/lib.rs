use apalis::prelude::TaskSink;
use apalis_redis::RedisStorage;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;

mod config;
mod constants;
pub mod jobs;
pub mod models;

pub mod commands;
pub mod params;

use crate::config::{DATABASE_CONFIG, MONITOR_CONFIG};
use crate::jobs::{NewSessionJob, NewUserJob};
use crate::models::{Session, User};

static DB_POOL_CELL: OnceCell<PgPool> = OnceCell::const_new();
static JOBS_STORAGE_CELL: OnceCell<JobsStorage> = OnceCell::const_new();

fn block_on<T>(f: impl Future<Output = T>) -> T {
    tokio::task::block_in_place(move || tokio::runtime::Handle::current().block_on(f))
}

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

pub async fn jobs_storage<'a>() -> &'a JobsStorage {
    JOBS_STORAGE_CELL
        .get_or_init(|| async { JobsStorage::new().await })
        .await
}

pub struct JobsStorage {
    pub new_session: RedisStorage<NewSessionJob>,
    pub new_user: RedisStorage<NewUserJob>,
}

impl JobsStorage {
    async fn new() -> Self {
        Self {
            new_session: Self::storage().await,
            new_user: Self::storage().await,
        }
    }

    async fn storage<T: Serialize + for<'de> Deserialize<'de>>() -> RedisStorage<T> {
        let conn = apalis_redis::connect(MONITOR_CONFIG.redis_url.clone())
            .await
            .expect("Could not connect to Redis Jobs DB");

        RedisStorage::new(conn)
    }

    pub(crate) async fn push_new_session(&self, session: &Session) {
        self.new_session
            .clone()
            .push(NewSessionJob { session_id: session.id })
            .await
            .expect("Could not store job");
    }

    pub(crate) async fn push_new_user(&self, user: &User<'_>) {
        self.new_user
            .clone()
            .push(NewUserJob { user_id: user.id })
            .await
            .expect("Could not store job");
    }
}
