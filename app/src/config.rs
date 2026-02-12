use std::sync::LazyLock;

use envconfig::Envconfig;

pub static SESSION_CONFIG: LazyLock<SessionConfig> = LazyLock::new(|| SessionConfig::init_from_env().unwrap());

#[derive(Envconfig)]
pub struct SessionConfig {
    #[envconfig(from = "SESSION_DOMAIN", default = "localhost")]
    pub domain: String,
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
