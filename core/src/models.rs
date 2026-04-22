use std::borrow::Cow;
use std::fmt::Display;
use std::path::PathBuf;

use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::commands;
use crate::config::{API_CONFIG, STORAGE_CONFIG};
use crate::enums::ConfirmationAction;

#[derive(Clone, Deserialize, Serialize)]
pub struct AccessToken<'a> {
    pub id: Uuid,
    pub application_id: Uuid,
    pub authorization_id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub code: Cow<'a, str>,
    pub refresh_code: Cow<'a, str>,
    pub code_expires_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub refreshed_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for AccessToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl AccessToken<'_> {
    pub async fn application<'a>(&self) -> sqlx::Result<Application<'a>> {
        commands::get_application_by_id(self.application_id).await
    }

    pub async fn authorization<'a>(&self) -> sqlx::Result<Authorization<'a>> {
        commands::get_authorization_by_id(self.authorization_id).await
    }

    pub async fn session(&self) -> sqlx::Result<Session> {
        commands::get_session_by_id(self.session_id).await
    }

    pub fn code_expires_in(&self) -> TimeDelta {
        self.code_expires_at - Utc::now()
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Application<'a> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub redirect_url: Cow<'a, str>,
    pub trusted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for Application<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Application<'_> {
    pub fn is_trusted(&self) -> bool {
        self.trusted_at.is_some()
    }

    pub fn redirect_url(&self) -> Url {
        Url::parse(&self.redirect_url).expect("Could not get Redirect URL")
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ApplicationToken<'a> {
    pub id: Uuid,
    pub application_id: Uuid,
    pub name: Cow<'a, str>,
    pub code: Cow<'a, str>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for ApplicationToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Authorization<'a> {
    pub id: Uuid,
    pub application_id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub code: Cow<'a, str>,
    pub code_challenge: Cow<'a, str>,
    pub redirect_url: Cow<'a, str>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for Authorization<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Authorization<'_> {
    pub async fn application<'a>(&self) -> sqlx::Result<Application<'a>> {
        commands::get_application_by_id(self.application_id).await
    }

    pub fn full_redirect_url(&self) -> Url {
        let mut url = self.redirect_url();

        url.set_query(Some(&format!("code={}", self.code)));

        url
    }

    pub fn redirect_url(&self) -> Url {
        Url::parse(&self.redirect_url).expect("Could not get Redirect URL")
    }

    pub async fn session(&self) -> sqlx::Result<Session> {
        commands::get_session_by_id(self.session_id).await
    }

    pub async fn user(&self) -> sqlx::Result<User<'_>> {
        commands::get_user_by_id(self.user_id).await
    }

    pub fn verify_code_challenge(&self, code_verifier: &str) -> bool {
        commands::verify_authorization_code_challenge(self, code_verifier)
    }
}

#[derive(Clone)]
pub struct Confirmation<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: ConfirmationAction,
    pub(crate) encrypted_code: Cow<'a, str>,
    pub pending_attempts: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Confirmation<'_> {
    pub async fn user(&self) -> User<'_> {
        commands::get_user_by_id(self.user_id)
            .await
            .expect("Could not get user")
    }

    pub fn verify_code(&self, code: &str) -> bool {
        commands::verify_password(&self.encrypted_code, code)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub ip_address: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub refreshed_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Session {
    pub async fn access_tokens(&self) -> sqlx::Result<Vec<AccessToken<'_>>> {
        commands::all_access_tokens_by_session(self).await
    }

    pub fn location(&self) -> String {
        let Some(country) = self.country_code.as_ref().and_then(|c| rust_iso3166::from_alpha2(c)) else {
            return "Unknown".to_owned();
        };

        let mut location = country.name.to_owned();

        if let Some(region) = &self.region {
            location += &format!(", {region}");
        }

        if let Some(city) = &self.city {
            location += &format!(", {city}");
        }

        location
    }

    pub fn should_refresh(&self) -> bool {
        self.refreshed_at.unwrap_or(self.created_at) < Utc::now() - TimeDelta::days(1)
    }

    pub async fn user<'a>(&self) -> sqlx::Result<User<'a>> {
        commands::get_user_by_id(self.user_id).await
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct User<'a> {
    pub id: Uuid,
    pub username: Cow<'a, str>,
    pub email: Cow<'a, str>,
    pub email_confirmed_at: Option<DateTime<Utc>>,
    pub(crate) encrypted_password: Cow<'a, str>,
    pub full_name: Cow<'a, str>,
    pub display_name: Cow<'a, str>,
    pub birthdate: NaiveDate,
    pub language_code: Cow<'a, str>,
    pub country_code: Cow<'a, str>,
    pub disabled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for User<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl User<'_> {
    pub fn avatar_image(&self, size: u32) -> anyhow::Result<Vec<u8>> {
        let avatar_image_path = self.avatar_image_path(size);

        if !avatar_image_path.exists() {
            let avatar_image = commands::generate_text_icon(&self.username, size)?;

            std::fs::create_dir_all(
                avatar_image_path
                    .parent()
                    .ok_or_else(|| anyhow::anyhow!("Failed to create directory"))?,
            )?;

            avatar_image.save(&avatar_image_path)?;
        }

        Ok(std::fs::read(&avatar_image_path)?)
    }

    pub fn avatar_image_path(&self, size: u32) -> PathBuf {
        STORAGE_CONFIG
            .path
            .join(format!("user_avatar_images/{size}x{size}/{}.jpg", self.id))
    }

    pub fn avatar_image_url(&self) -> Url {
        API_CONFIG.url.join(&format!("users/{}/avatar-image", self.id)).unwrap()
    }

    pub fn email_is_confirmed(&self) -> bool {
        self.email_confirmed_at.is_some()
    }

    pub fn initials(&self) -> String {
        self.username[0..2].to_uppercase()
    }

    pub(crate) fn verify_password(&self, password: &str) -> bool {
        commands::verify_password(&self.encrypted_password, password)
    }
}
