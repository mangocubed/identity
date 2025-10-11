use std::borrow::Cow;

use chrono::{DateTime, NaiveDate, Utc};
use url::Url;
use uuid::Uuid;

use crate::commands::{get_user_by_id, verify_password};

pub struct Application<'a> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub redirect_url: Cow<'a, str>,
    pub encrypted_secret: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Application<'_> {
    pub fn redirect_url(&self) -> Url {
        Url::parse(&self.redirect_url).expect("Could not get Redirect URL")
    }

    pub fn verify_secret(&self, secret: &str) -> bool {
        verify_password(&self.encrypted_secret, secret)
    }
}

pub struct Authorization<'a> {
    pub id: Uuid,
    pub application_id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub token: Cow<'a, str>,
    pub previous_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub refreshed_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Authorization<'_> {
    pub async fn user(&self) -> User<'_> {
        get_user_by_id(self.user_id).await.expect("Could not get user")
    }
}

pub struct Session<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: Cow<'a, str>,
    pub user_agent: Cow<'a, str>,
    pub country_alpha2: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub finished_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Session<'_> {
    pub fn location(&self) -> String {
        let Some(country) = self.country_alpha2.as_ref().and_then(|c| rust_iso3166::from_alpha2(c)) else {
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

    pub async fn user(&self) -> User<'_> {
        get_user_by_id(self.user_id).await.expect("Could not get user")
    }
}

pub struct User<'a> {
    pub id: Uuid,
    pub username: Cow<'a, str>,
    pub email: Cow<'a, str>,
    pub(crate) encrypted_password: Cow<'a, str>,
    pub display_name: Cow<'a, str>,
    pub full_name: Cow<'a, str>,
    pub birthdate: NaiveDate,
    pub language_code: Cow<'a, str>,
    pub country_alpha2: Cow<'a, str>,
    pub disabled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User<'_> {
    pub fn initials(&self) -> String {
        self.display_name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .collect::<String>()
            .to_uppercase()
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify_password(&self.encrypted_password, password)
    }
}
