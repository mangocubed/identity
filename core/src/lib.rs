use apalis::prelude::TaskSink;
use apalis_redis::RedisStorage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;

mod constants;

pub mod commands;
pub mod config;
pub mod enums;
pub mod jobs;
pub mod models;
pub mod params;

use crate::config::{DATABASE_CONFIG, MONITOR_CONFIG, SENTRY_CONFIG};
use crate::jobs::{FinishedSessionJob, NewConfirmationJob, NewSessionJob, NewUserJob, PasswordChangedJob};
use crate::models::{Confirmation, Session, User};

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

pub fn start_tracing_subscriber() -> Option<sentry::ClientInitGuard> {
    use sentry::integrations::tracing::EventFilter;
    use tracing_subscriber::prelude::*;

    let sentry_guard = if let Some(sentry_dsn) = SENTRY_CONFIG.dsn.as_deref() {
        let guard = sentry::init((
            sentry_dsn,
            sentry::ClientOptions {
                debug: cfg!(debug_assertions),
                enable_logs: true,
                release: Some(env!("CARGO_PKG_VERSION").into()),
                traces_sample_rate: SENTRY_CONFIG.traces_sample_rate,
                send_default_pii: SENTRY_CONFIG.send_default_pii,
                ..Default::default()
            },
        ));

        let sentry_layer = sentry::integrations::tracing::layer()
            .event_filter(|metadata| match *metadata.level() {
                tracing::Level::ERROR => EventFilter::Event | EventFilter::Log,
                tracing::Level::WARN => EventFilter::Breadcrumb | EventFilter::Log,
                _ => EventFilter::Ignore,
            })
            .span_filter(|metadata| matches!(*metadata.level(), tracing::Level::ERROR | tracing::Level::WARN));

        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(sentry_layer)
            .init();

        Some(guard)
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();

        None
    };

    tracing::info!("Tracing subscriber initialized.");

    sentry_guard
}

pub async fn jobs_storage<'a>() -> &'a JobsStorage {
    JOBS_STORAGE_CELL
        .get_or_init(|| async { JobsStorage::new().await })
        .await
}

pub struct JobsStorage {
    pub finished_session: RedisStorage<FinishedSessionJob>,
    pub new_confirmation: RedisStorage<NewConfirmationJob>,
    pub new_session: RedisStorage<NewSessionJob>,
    pub new_user: RedisStorage<NewUserJob>,
    pub password_changed: RedisStorage<PasswordChangedJob>,
}

impl JobsStorage {
    async fn new() -> Self {
        Self {
            finished_session: Self::storage().await,
            new_confirmation: Self::storage().await,
            new_session: Self::storage().await,
            new_user: Self::storage().await,
            password_changed: Self::storage().await,
        }
    }

    async fn storage<T: Serialize + for<'de> Deserialize<'de>>() -> RedisStorage<T> {
        let conn = apalis_redis::connect(MONITOR_CONFIG.redis_url.clone())
            .await
            .expect("Could not connect to Redis Jobs DB");

        RedisStorage::new(conn)
    }

    pub(crate) async fn push_finished_session(&self, session: &Session) {
        self.finished_session
            .clone()
            .push(FinishedSessionJob { session_id: session.id })
            .await
            .expect("Could not store job");
    }

    pub(crate) async fn push_new_confirmation(&self, confirmation: &Confirmation<'_>, code: &str) {
        self.new_confirmation
            .clone()
            .push(NewConfirmationJob {
                confirmation_id: confirmation.id,
                code: code.to_owned(),
            })
            .await
            .expect("Could not store job");
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

    pub(crate) async fn push_password_changed(&self, user: &User<'_>) {
        self.password_changed
            .clone()
            .push(PasswordChangedJob { user_id: user.id })
            .await
            .expect("Could not store job");
    }
}

#[derive(Serialize)]
pub struct Info {
    pub built_at: DateTime<Utc>,
    pub version: String,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            built_at: env!("BUILD_DATETIME").parse().unwrap(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
