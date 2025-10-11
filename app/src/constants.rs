use std::sync::LazyLock;

pub static SOURCE_CODE_URL: LazyLock<String> =
    LazyLock::new(|| format!("{}/tree/{}", env!("CARGO_PKG_REPOSITORY"), env!("GIT_REV_SHORT")));

pub const HEADER_AUTHORIZATION: &str = "Authorization";
#[cfg(feature = "server")]
pub const HEADER_REAL_IP: &str = "X-Real-IP";
#[cfg(feature = "server")]
pub const HEADER_USER_AGENT: &str = "User-Agent";
pub const KEY_REDIRECT_TO: &str = "_redirect_to";
pub const KEY_SESSION_TOKEN: &str = "_session_token";
