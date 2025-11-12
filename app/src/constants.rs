use std::sync::LazyLock;

pub static SOURCE_CODE_URL: LazyLock<String> =
    LazyLock::new(|| format!("{}/tree/{}", env!("CARGO_PKG_REPOSITORY"), env!("GIT_REV_SHORT")));

pub const KEY_REDIRECT_TO: &str = "_redirect_to";
pub const KEY_SESSION: &str = "_session";

pub const PATH_API_AUTHORIZE: &str = "/api/authorize";
