use serde::Deserialize;
use url::Url;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenGrantType {
    AuthorizationCode,
    RefreshToken,
}

#[derive(Deserialize)]
pub struct RevokeParams {
    pub client_id: Uuid,
    pub token: String,
}

#[derive(Deserialize)]
pub struct TokenParams {
    pub grant_type: TokenGrantType,
    pub client_id: Uuid,
    pub code: Option<String>,
    pub redirect_uri: Option<Url>,
    pub refresh_token: Option<String>,
    pub code_verifier: Option<String>,
}

#[derive(Deserialize)]
pub struct AvatarImageParams {
    pub size: Option<u32>,
}
