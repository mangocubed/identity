use sdk::app::storage::{LocalStorage, SessionStorage, StorageBacking};
use sdk::app::{remove_request_bearer, set_request_bearer};

use crate::constants::{KEY_REDIRECT_TO, KEY_SESSION};
use crate::presenters::SessionPresenter;

pub fn delete_redirect_to() {
    SessionStorage::set(KEY_REDIRECT_TO.to_owned(), &None::<()>);
}

pub fn delete_session() {
    LocalStorage::set(KEY_SESSION.to_owned(), &None::<()>);
    remove_request_bearer();
}

pub fn get_redirect_to() -> String {
    SessionStorage::get(&KEY_REDIRECT_TO.to_owned()).unwrap_or("/".to_owned())
}

pub fn get_session() -> Option<SessionPresenter> {
    LocalStorage::get(&KEY_SESSION.to_owned())
}

pub fn set_redirect_to(url: &String) {
    SessionStorage::set(KEY_REDIRECT_TO.to_owned(), url);
}

pub fn set_session(value: &SessionPresenter) {
    LocalStorage::set(KEY_SESSION.to_owned(), value);
    set_request_bearer(&value.token);
}
