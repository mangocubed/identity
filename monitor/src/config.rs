use std::sync::LazyLock;

use envconfig::Envconfig;

pub static IP_GEO_CONFIG: LazyLock<IpGeoConfig> = LazyLock::new(|| IpGeoConfig::init_from_env().unwrap());

#[derive(Envconfig)]
pub struct IpGeoConfig {
    #[envconfig(from = "IP_GEO_API_KEY", default = "")]
    pub api_key: String,
}
