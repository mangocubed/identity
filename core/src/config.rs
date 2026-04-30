use std::net::SocketAddr;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::time::Duration;

use envconfig::Envconfig;
use url::Url;

pub static API_CONFIG: LazyLock<ApiConfig> = LazyLock::new(|| ApiConfig::init_from_env().unwrap());
pub(crate) static ACCESS_TOKEN_CONFIG: LazyLock<AccessTokenConfig> =
    LazyLock::new(|| AccessTokenConfig::init_from_env().unwrap());
pub(crate) static APPLICATION_TOKEN_CONFIG: LazyLock<ApplicationTokenConfig> =
    LazyLock::new(|| ApplicationTokenConfig::init_from_env().unwrap());
pub(crate) static AUTHORIZATION_CONFIG: LazyLock<AuthorizationConfig> =
    LazyLock::new(|| AuthorizationConfig::init_from_env().unwrap());
pub(crate) static CONFIRMATION_CONFIG: LazyLock<ConfirmationConfig> =
    LazyLock::new(|| ConfirmationConfig::init_from_env().unwrap());
pub(crate) static DATABASE_CONFIG: LazyLock<DatabaseConfig> =
    LazyLock::new(|| DatabaseConfig::init_from_env().unwrap());
pub(crate) static MONITOR_CONFIG: LazyLock<MonitorConfig> = LazyLock::new(|| MonitorConfig::init_from_env().unwrap());
pub(crate) static STORAGE_CONFIG: LazyLock<StorageConfig> = LazyLock::new(|| StorageConfig::init_from_env().unwrap());

#[derive(Envconfig)]
pub(crate) struct AccessTokenConfig {
    #[envconfig(from = "ACCESS_TOKEN_CODE_TTL_SECS", default = "86400")]
    code_ttl_secs: u64,
    #[envconfig(from = "ACCESS_TOKEN_MIN_LENGTH", default = "64")]
    min_length: u8,
    #[envconfig(from = "ACCESS_TOKEN_MAX_LENGTH", default = "128")]
    max_length: u8,
    #[envconfig(from = "ACCESS_TOKEN_TTL_SECS", default = "2592000")]
    ttl_secs: u64,
}

impl AccessTokenConfig {
    pub fn code_ttl(&self) -> Duration {
        Duration::from_secs(self.code_ttl_secs)
    }

    pub fn length(&self) -> RangeInclusive<u8> {
        self.min_length..=self.max_length
    }

    pub fn ttl(&self) -> Duration {
        Duration::from_secs(self.ttl_secs)
    }
}

#[derive(Envconfig)]
pub struct ApiConfig {
    #[envconfig(from = "API_ADDRESS", default = "127.0.0.1:8005")]
    pub address: SocketAddr,
    #[envconfig(from = "API_URL", default = "http://127.0.0.1:8005")]
    pub(crate) url: Url,
}

#[derive(Envconfig)]
pub(crate) struct ApplicationTokenConfig {
    #[envconfig(from = "APPLICATION_TOKEN_MIN_LENGTH", default = "64")]
    min_length: u8,
    #[envconfig(from = "APPLICATION_TOKEN_MAX_LENGTH", default = "128")]
    max_length: u8,
    #[envconfig(from = "APPLICATION_TOKEN_TTL_SECS", default = "31104000")]
    ttl_secs: u64,
}

impl ApplicationTokenConfig {
    pub fn length(&self) -> RangeInclusive<u8> {
        self.min_length..=self.max_length
    }

    pub fn ttl(&self) -> Duration {
        Duration::from_secs(self.ttl_secs)
    }
}

#[derive(Envconfig)]
pub(crate) struct AuthorizationConfig {
    #[envconfig(from = "AUTHORIZATION_MIN_LENGTH", default = "64")]
    min_length: u8,
    #[envconfig(from = "AUTHORIZATION_MAX_LENGTH", default = "128")]
    max_length: u8,
    #[envconfig(from = "AUTHORIZATION_TTL_SECS", default = "600")]
    ttl_secs: u64,
}

impl AuthorizationConfig {
    pub fn length(&self) -> RangeInclusive<u8> {
        self.min_length..=self.max_length
    }

    pub fn ttl(&self) -> Duration {
        Duration::from_secs(self.ttl_secs)
    }
}

#[derive(Envconfig)]
pub(crate) struct ConfirmationConfig {
    #[envconfig(from = "CONFIRMATION_CODE_LENGTH", default = "6")]
    pub code_length: u8,
}

#[derive(Envconfig)]
pub(crate) struct DatabaseConfig {
    #[envconfig(from = "DATABASE_MAX_CONNECTIONS", default = "5")]
    pub max_connections: u32,
    #[envconfig(
        from = "DATABASE_URL",
        default = "postgres://mango3:mango3@127.0.0.1:5432/identity_dev"
    )]
    pub url: String,
}

#[derive(Envconfig)]
pub(crate) struct MonitorConfig {
    #[envconfig(from = "MONITOR_REDIS_URL", default = "redis://127.0.0.1:6379/1")]
    pub redis_url: String,
}

#[derive(Envconfig)]
pub struct SentryConfig {
    #[envconfig(from = "SENTRY_DSN")]
    pub dsn: Option<String>,
    #[envconfig(from = "SENTRY_TRACES_SAMPLE_RATE", default = "1.0")]
    pub traces_sample_rate: f32,
    #[envconfig(from = "SENTRY_SEND_DEFAULT_PII", default = "true")]
    pub send_default_pii: bool,
}

#[derive(Envconfig)]
pub(crate) struct StorageConfig {
    #[envconfig(
        from = "STORAGE_FONT_PATH",
        default = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
    )]
    pub font_path: PathBuf,
    #[envconfig(from = "STORAGE_PATH", default = "./storage/")]
    pub path: PathBuf,
}
