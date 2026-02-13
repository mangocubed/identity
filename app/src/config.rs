use std::sync::LazyLock;

use axum_client_ip::ClientIpSource;
use envconfig::Envconfig;

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::init_from_env().unwrap());
pub static SESSION_CONFIG: LazyLock<SessionConfig> = LazyLock::new(|| SessionConfig::init_from_env().unwrap());

#[derive(Envconfig)]
pub struct AppConfig {
    #[envconfig(from = "APP_CLIENT_IP_SOURCE", default = "ConnectInfo")]
    pub client_ip_source: ClientIpSource,
}

#[derive(Envconfig)]
pub struct SessionConfig {
    #[envconfig(from = "SESSION_DOMAIN")]
    pub domain: Option<String>,
    #[envconfig(
        from = "SESSION_PRIVATE_KEY",
        default = "abcdefghijklmnopqrestuvvwxyz0123456789ABCDEFGHIJKLMNOPQRESTUVVWX"
    )]
    pub private_key: String,
    #[envconfig(from = "SESSION_REDIS_URL", default = "redis://127.0.0.1:6379/2")]
    pub redis_url: String,
    #[envconfig(from = "SESSION_SECURE", default = "false")]
    pub secure: bool,
}
