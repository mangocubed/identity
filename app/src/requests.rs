use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use uuid::Uuid;

use sdk::app::{ActionResult, ClientRequest, ServerResult};

use crate::constants::*;
use crate::presenters::UserPresenter;

#[derive(Deserialize, Serialize)]
pub struct AuthorizeParams {
    pub client_id: Uuid,
}

pub async fn authorize(client_id: Uuid) -> ServerResult<Url> {
    ClientRequest::post(PATH_API_AUTHORIZE)
        .json(&AuthorizeParams { client_id })
        .send()
        .await
}

pub async fn can_register() -> ServerResult<bool> {
    ClientRequest::get(PATH_API_CAN_REGISTER).send().await
}

pub async fn current_user() -> ServerResult<UserPresenter> {
    ClientRequest::get(PATH_API_CURRENT_USER).send().await
}

pub async fn login(input: Value) -> ActionResult {
    ClientRequest::post(PATH_API_LOGIN).json(&input).send().await
}

pub async fn logout() -> ServerResult {
    ClientRequest::delete(PATH_API_LOGOUT).send().await
}

pub async fn register(input: Value) -> ActionResult {
    ClientRequest::post(PATH_API_REGISTER).json(&input).send().await
}
