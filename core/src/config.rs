use std::sync::LazyLock;

use envconfig::Envconfig;

pub static DATABASE_CONFIG: LazyLock<DatabaseConfig> = LazyLock::new(|| DatabaseConfig::init_from_env().unwrap());

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
