use sdk::data_storage::{DataStorage, data_storage};
use sdk::serv_fn::{remove_serv_fn_header, set_serv_fn_header};

use crate::constants::{HEADER_AUTHORIZATION, KEY_REDIRECT_TO, KEY_SESSION_TOKEN};

pub fn delete_redirect_to() {
    data_storage().delete(KEY_REDIRECT_TO);
}

pub fn delete_session_token() {
    data_storage().delete(KEY_SESSION_TOKEN);
    remove_serv_fn_header(HEADER_AUTHORIZATION);
}

pub fn get_redirect_to() -> String {
    data_storage().get(KEY_REDIRECT_TO).unwrap_or("/".to_owned())
}

#[cfg(not(feature = "server"))]
pub fn get_session_token() -> Option<String> {
    data_storage().get(KEY_SESSION_TOKEN)
}

pub fn set_redirect_to(url: &str) {
    data_storage().set(KEY_REDIRECT_TO, url);
}

pub fn set_session_token(token: &str) {
    data_storage().set(KEY_SESSION_TOKEN, token);
    set_serv_fn_header(HEADER_AUTHORIZATION, &format!("Bearer {token}"));
}
