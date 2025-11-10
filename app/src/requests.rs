use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use sdk::app::{Request, ServerResult};

use crate::constants::*;

#[derive(Deserialize, Serialize)]
pub struct AuthorizeParams {
    pub client_id: Uuid,
}

pub async fn authorize(client_id: Uuid) -> ServerResult<Url> {
    Request::post(PATH_API_AUTHORIZE)
        .json(&AuthorizeParams { client_id })
        .send()
        .await
}
