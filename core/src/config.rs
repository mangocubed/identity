use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use sdk::config::extract_config_from_env;

pub(crate) static DATABASE_CONFIG: LazyLock<DatabaseConfig> = LazyLock::new(|| extract_config_from_env("DATABASE_"));
pub static IP_GEOLOCATION_CONFIG: LazyLock<IpGeolocationConfig> =
    LazyLock::new(|| extract_config_from_env("IP_GEOLOCATION_"));
pub static MAILER_CONFIG: LazyLock<MailerConfig> = LazyLock::new(|| extract_config_from_env("MAILER_"));
pub(crate) static MONITOR_CONFIG: LazyLock<MonitorConfig> = LazyLock::new(|| extract_config_from_env("MONITOR_"));
pub(crate) static USERS_CONFIG: LazyLock<UsersConfig> = LazyLock::new(|| extract_config_from_env("USERS_"));

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
pub struct IpGeolocationConfig {
    pub api_key: String,
}

impl Default for IpGeolocationConfig {
    fn default() -> Self {
        Self { api_key: "".to_owned() }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MailerConfig {
    pub enable: bool,
    pub sender_address: String,
    pub smtp_address: String,
    pub smtp_password: String,
    pub smtp_security: String,
    pub smtp_username: String,
    pub support_email_address: String,
}

impl Default for MailerConfig {
    fn default() -> Self {
        Self {
            enable: false,
            sender_address: "Mango³ dev <no-reply@localhost>".to_owned(),
            smtp_address: "localhost".to_owned(),
            smtp_password: "".to_owned(),
            smtp_security: "none".to_owned(),
            smtp_username: "".to_owned(),
            support_email_address: "support@localhost".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UsersConfig {
    pub session_token_length: u8,
    pub limit: u8,
}

impl Default for UsersConfig {
    fn default() -> Self {
        Self {
            session_token_length: 64,
            limit: 10,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct MonitorConfig {
    pub redis_url: String,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        let db_number = if cfg!(test) { "10" } else { "0" };

        Self {
            redis_url: format!("redis://127.0.0.1:6379/{db_number}"),
        }
    }
}
