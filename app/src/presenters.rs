use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "server")]
use identity_core::models::{Session, User};

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct SessionPresenter {
    pub token: String,
    refreshed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl SessionPresenter {
    pub fn should_refresh(&self) -> bool {
        self.refreshed_at.unwrap_or(self.created_at) < Utc::now() - TimeDelta::days(1)
    }
}

#[cfg(feature = "server")]
impl From<Session<'_>> for SessionPresenter {
    fn from(session: Session<'_>) -> Self {
        SessionPresenter {
            token: session.token.to_string(),
            refreshed_at: session.refreshed_at,
            created_at: session.created_at,
        }
    }
}

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
