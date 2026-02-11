use std::sync::LazyLock;
use std::time::Duration;

use envconfig::Envconfig;

pub static CACHE_CONFIG: LazyLock<CacheConfig> = LazyLock::new(|| CacheConfig::init_from_env().unwrap());
pub static DATABASE_CONFIG: LazyLock<DatabaseConfig> = LazyLock::new(|| DatabaseConfig::init_from_env().unwrap());
pub static MONITOR_CONFIG: LazyLock<MonitorConfig> = LazyLock::new(|| MonitorConfig::init_from_env().unwrap());

#[derive(Envconfig)]
pub struct CacheConfig {
    #[envconfig(from = "CACHE_REDIS_URL", default = "redis://127.0.0.1:6379/1")]
    pub redis_url: String,
    #[envconfig(from = "CACHE_TTL", default = "3600")]
    ttl: u16,
}

impl CacheConfig {
    pub fn ttl(&self) -> Duration {
        Duration::from_secs(self.ttl as u64)
    }
}

#[derive(Envconfig)]
pub struct DatabaseConfig {
    #[envconfig(from = "DATABASE_MAX_CONNECTIONS", default = "5")]
    pub max_connections: u32,
    #[envconfig(
        from = "DATABASE_URL",
        default = "postgres://mango3:mango3@127.0.0.1:5432/identity_dev"
    )]
    pub url: String,
}

#[derive(Envconfig)]
pub struct MonitorConfig {
    #[envconfig(from = "MONITOR_REDIS_URL", default = "redis://127.0.0.1:6379/0")]
    pub redis_url: String,
}
