use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;
use validator::ValidationError;

pub static ERROR_ALREADY_EXISTS: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("already-exists").with_message(Cow::Borrowed("Already exists")));
pub static ERROR_IS_INVALID: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("invalid").with_message(Cow::Borrowed("Is invalid")));

pub static REGEX_USERNAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[-_.]?([[:alnum:]]+[-_.]?)+\z").unwrap());

pub const CACHE_PREFIX_GET_ACCESS_TOKEN_BY_CODE: &str = "get_access_token_by_code";
pub const CACHE_PREFIX_GET_ACCESS_TOKEN_BY_REFRESH_CODE: &str = "get_access_token_by_refresh_code";
pub const CACHE_PREFIX_GET_APPLICATION_BY_ID: &str = "get_application_by_id";
pub const CACHE_PREFIX_GET_AUTHORIZATION_BY_CODE: &str = "get_authorization_by_code";
pub const CACHE_PREFIX_GET_AUTHORIZATION_BY_ID: &str = "get_authorization_by_id";
pub const CACHE_PREFIX_GET_SESSION_BY_ID: &str = "get_session_by_id";
pub const CACHE_PREFIX_GET_USER_BY_ID: &str = "get_user_by_id";
pub const CACHE_PREFIX_GET_USER_BY_USERNAME_OR_EMAIL: &str = "get_user_by_username_or_email";
pub const CACHE_PREFIX_GET_USER_ID_BY_EMAIL: &str = "get_user_id_by_email";
pub const CACHE_PREFIX_GET_USER_ID_BY_USERNAME: &str = "get_user_id_by_username";
