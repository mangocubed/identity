use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use sdk::config::extract_config_from_env;

pub(crate) static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| extract_config_from_env("APP_"));
pub(crate) static DATABASE_CONFIG: LazyLock<DatabaseConfig> = LazyLock::new(|| extract_config_from_env("DATABASE_"));
pub(crate) static USERS_CONFIG: LazyLock<UsersConfig> = LazyLock::new(|| extract_config_from_env("USERS_"));

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub token: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            token: "identity_dev".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DatabaseConfig {
    pub max_connections: u8,
    pub url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        let db_suffix = if cfg!(test) { "test" } else { "dev" };

        Self {
            max_connections: 5,
            url: format!("postgres://mango3:mango3@127.0.0.1:5432/identity_{db_suffix}"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UsersConfig {
    pub access_token_length: u8,
    pub limit: u8,
}

impl Default for UsersConfig {
    fn default() -> Self {
        Self {
            access_token_length: 64,
            limit: 10,
        }
    }
}
