use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use identity_core::models::User;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserPresenter {
    id: Uuid,
    pub username: String,
    pub display_name: String,
    pub initials: String,
    avatar_image_url: Url,
}

impl UserPresenter {
    pub fn avatar_image_url(&self, size: u32) -> Url {
        let mut avatar_image_url = self.avatar_image_url.clone();

        avatar_image_url.set_query(Some(&format!("size={}", size)));

        avatar_image_url
    }
}

#[cfg(feature = "ssr")]
impl From<User<'_>> for UserPresenter {
    fn from(user: User<'_>) -> Self {
        UserPresenter {
            id: user.id,
            username: user.username.to_string(),
            display_name: user.display_name.to_string(),
            initials: user.initials(),
            avatar_image_url: user.avatar_image_url(),
        }
    }
}
