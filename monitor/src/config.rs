use std::sync::LazyLock;

use envconfig::Envconfig;

pub static MAILER_CONFIG: LazyLock<MailerConfig> = LazyLock::new(|| MailerConfig::init_from_env().unwrap());
pub static IP_GEO_CONFIG: LazyLock<IpGeoConfig> = LazyLock::new(|| IpGeoConfig::init_from_env().unwrap());

#[derive(Envconfig)]
pub struct IpGeoConfig {
    #[envconfig(from = "IP_GEO_API_KEY", default = "")]
    pub api_key: String,
}

#[derive(Envconfig)]
pub struct MailerConfig {
    #[envconfig(from = "MAILER_ENABLE", default = "false")]
    pub enable: bool,
    #[envconfig(from = "MAILER_SENDER_ADDRESS", default = "Mango³ <no-reply@localhost>")]
    pub sender_address: String,
    #[envconfig(from = "MAILER_SMTP_ADDRESS", default = "localhost")]
    pub smtp_address: String,
    #[envconfig(from = "MAILER_SMTP_PASSWORD", default = "")]
    pub smtp_password: String,
    #[envconfig(from = "MAILER_SMTP_SECURITY", default = "none")]
    pub smtp_security: String,
    #[envconfig(from = "MAILER_SMTP_USERNAME", default = "")]
    pub smtp_username: String,
    #[envconfig(from = "MAILER_SUPPORT_EMAIL_ADDRESS", default = "support@localhost")]
    pub support_email_address: String,
}
