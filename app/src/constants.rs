use std::sync::LazyLock;

use dioxus::prelude::{Asset, asset, manganis};

pub static SOURCE_CODE_URL: LazyLock<String> =
    LazyLock::new(|| format!("{}/tree/{}", env!("CARGO_PKG_REPOSITORY"), env!("GIT_REV_SHORT")));

pub const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
pub const STYLE_CSS: Asset = asset!("assets/style.css");

pub const KEY_REDIRECT_TO: &str = "_redirect_to";
pub const KEY_SESSION: &str = "_session";

pub const PATH_API_AUTHORIZE: &str = "/api/authorize";
