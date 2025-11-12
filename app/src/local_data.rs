use sdk::app::{DataStorage, data_storage, remove_request_bearer, set_request_bearer};

use crate::constants::{KEY_REDIRECT_TO, KEY_SESSION};
use crate::presenters::SessionPresenter;

pub fn delete_redirect_to() {
    data_storage().delete(KEY_REDIRECT_TO);
}

pub fn delete_session() {
    data_storage().delete(KEY_SESSION);
    remove_request_bearer();
}

pub fn get_redirect_to() -> String {
    data_storage().get(KEY_REDIRECT_TO).unwrap_or("/".to_owned())
}

pub fn get_session() -> Option<SessionPresenter> {
    data_storage().get(KEY_SESSION)
}

pub fn set_redirect_to(url: String) {
    data_storage().set(KEY_REDIRECT_TO, &url);
}

pub fn set_session(value: SessionPresenter) {
    data_storage().set(KEY_SESSION, &value);
    set_request_bearer(&value.token);
}
