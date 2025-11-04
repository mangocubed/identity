use std::net::IpAddr;

use apalis::prelude::Storage;
use apalis_redis::RedisStorage;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::config::MONITOR_CONFIG;
use crate::models::{Confirmation, Session, User};

static JOBS_STORAGE_CELL: OnceCell<JobsStorage> = OnceCell::const_new();

pub async fn jobs_storage<'a>() -> &'a JobsStorage {
    JOBS_STORAGE_CELL
        .get_or_init(|| async { JobsStorage::new().await })
        .await
}

#[derive(Clone, Debug)]
pub struct JobsStorage {
    pub finished_session: RedisStorage<FinishedSession>,
    pub new_confirmation: RedisStorage<NewConfirmation>,
    pub new_session: RedisStorage<NewSession>,
    pub new_user: RedisStorage<NewUser>,
    pub password_changed: RedisStorage<PasswordChanged>,
}

impl JobsStorage {
    async fn storage<T: Serialize + for<'de> Deserialize<'de>>() -> RedisStorage<T> {
        let conn = apalis_redis::connect(MONITOR_CONFIG.redis_url.clone())
            .await
            .expect("Could not connect to Redis Jobs DB");
        RedisStorage::new(conn)
    }

    async fn new() -> Self {
        Self {
            finished_session: Self::storage().await,
            new_confirmation: Self::storage().await,
            new_session: Self::storage().await,
            new_user: Self::storage().await,
            password_changed: Self::storage().await,
        }
    }

    pub(crate) async fn push_finished_session(&self, session: &Session<'_>) {
        self.finished_session
            .clone()
            .push(FinishedSession { session_id: session.id })
            .await
            .expect("Could not store job");
    }

    pub(crate) async fn push_new_confirmation(&self, confirmation: &Confirmation<'_>, code: &str) {
        self.new_confirmation
            .clone()
            .push(NewConfirmation {
                confirmation_id: confirmation.id,
                code: code.to_owned(),
            })
            .await
            .expect("Could not store job");
    }

    pub(crate) async fn push_new_session(&self, session: &Session<'_>, ip_addr: IpAddr) {
        self.new_session
            .clone()
            .push(NewSession {
                session_id: session.id,
                ip_addr,
            })
            .await
            .expect("Could not store job");
    }

    pub(crate) async fn push_new_user(&self, user: &User<'_>) {
        self.new_user
            .clone()
            .push(NewUser { user_id: user.id })
            .await
            .expect("Could not store job");
    }

    pub(crate) async fn push_password_changed(&self, user: &User<'_>) {
        self.password_changed
            .clone()
            .push(PasswordChanged { user_id: user.id })
            .await
            .expect("Could not store job");
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FinishedSession {
    pub session_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewConfirmation {
    pub confirmation_id: Uuid,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewSession {
    pub session_id: Uuid,
    pub ip_addr: IpAddr,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PasswordChanged {
    pub user_id: Uuid,
}
