use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use identity_core::models::User;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserPresenter {
    id: Uuid,
    pub username: String,
    pub initials: String,
}

#[cfg(feature = "ssr")]
impl From<User<'_>> for UserPresenter {
    fn from(user: User<'_>) -> Self {
        UserPresenter {
            id: user.id,
            username: user.username.to_string(),
            initials: user.initials(),
        }
    }
}
