use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use identity_core::models::{Application, User};

#[derive(Clone, Deserialize, Serialize)]
pub struct ApplicationPresenter {
    pub id: Uuid,
    pub name: String,
    pub is_trusted: bool,
}

#[cfg(feature = "ssr")]
impl From<Application<'_>> for ApplicationPresenter {
    fn from(application: Application<'_>) -> Self {
        ApplicationPresenter {
            id: application.id,
            name: application.name.to_string(),
            is_trusted: application.is_trusted(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserPresenter {
    id: Uuid,
    pub username: String,
    pub email: String,
    pub email_is_confirmed: bool,
    pub display_name: String,
    pub full_name: String,
    pub birthdate: NaiveDate,
    pub country_code: String,
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
            email: user.email.to_string(),
            email_is_confirmed: user.email_is_confirmed(),
            display_name: user.display_name.to_string(),
            full_name: user.full_name.to_string(),
            birthdate: user.birthdate,
            country_code: user.country_code.to_string(),
            initials: user.initials(),
            avatar_image_url: user.avatar_image_url(),
        }
    }
}
