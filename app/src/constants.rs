use std::sync::LazyLock;

pub static SOURCE_CODE_URL: LazyLock<String> =
    LazyLock::new(|| format!("{}/tree/{}", env!("CARGO_PKG_REPOSITORY"), env!("GIT_REV_SHORT")));

pub const KEY_REDIRECT_TO: &str = "_redirect_to";
pub const KEY_SESSION_TOKEN: &str = "_session_token";

pub const PATH_API_AUTHORIZE: &str = "/api/authorize";
pub const PATH_API_CURRENT_USER: &str = "/api/current-user";
pub const PATH_API_LOGIN: &str = "/api/login";
pub const PATH_API_LOGOUT: &str = "/api/logout";
pub const PATH_API_REGISTER: &str = "/api/register";
