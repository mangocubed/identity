use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "server")]
use identity_core::models::User;

#[derive(Deserialize, Serialize)]
pub struct UserPresenter {
    id: Uuid,
    pub username: String,
    pub email: String,
    pub email_is_confirmed: bool,
    pub display_name: String,
    pub initials: String,
}

#[cfg(feature = "server")]
impl From<User<'_>> for UserPresenter {
    fn from(user: User<'_>) -> Self {
        UserPresenter {
            id: user.id,
            username: user.username.to_string(),
            email: user.email.to_string(),
            email_is_confirmed: user.email_is_confirmed(),
            display_name: user.display_name.to_string(),
            initials: user.initials(),
        }
    }
}
