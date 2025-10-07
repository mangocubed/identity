use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use validator::{Validate, ValidationError};

#[cfg(feature = "server")]
use sdk::constants::ERROR_IS_INVALID;

#[cfg(feature = "server")]
use crate::constants::REGEX_USERNAME;

#[cfg(feature = "server")]
fn validate_birthdate(value: &str) -> Result<(), ValidationError> {
    use chrono::{NaiveDate, Utc};

    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        if date > Utc::now().date_naive() {
            return Err(ERROR_IS_INVALID.clone());
        }
    } else {
        return Err(ERROR_IS_INVALID.clone());
    }

    Ok(())
}

#[cfg(feature = "server")]
fn validate_country_alpha2(value: &str) -> Result<(), ValidationError> {
    use rust_iso3166::ALL_ALPHA2;

    if !ALL_ALPHA2.contains(&value) {
        return Err(ERROR_IS_INVALID.clone());
    }

    Ok(())
}

#[cfg(feature = "server")]
fn validate_username(value: &str) -> Result<(), ValidationError> {
    if uuid::Uuid::try_parse(value).is_ok() {
        return Err(ERROR_IS_INVALID.clone());
    }

    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Validate))]
pub struct LoginInput {
    #[cfg_attr(feature = "server", validate(length(min = 1, max = 256, message = "Can't be blank")))]
    pub username_or_email: String,
    #[cfg_attr(feature = "server", validate(length(min = 1, max = 256, message = "Can't be blank")))]
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Validate))]
pub struct RegisterInput {
    #[cfg_attr(
        feature = "server",
        validate(
            length(min = 3, max = 16, message = "Must have at least 3 characters"),
            regex(path = *REGEX_USERNAME, message = "Is invalid"),
            custom(function = "validate_username")
        )
    )]
    pub username: String,
    #[cfg_attr(
        feature = "server",
        validate(
            length(min = 5, max = 256, message = "Must have at least 5 characters"),
            email(message = "Is invalid")
        )
    )]
    pub email: String,
    #[cfg_attr(
        feature = "server",
        validate(length(min = 6, max = 128, message = "Must have at least 6 characters"))
    )]
    pub password: String,
    #[cfg_attr(
        feature = "server",
        validate(length(min = 2, max = 256, message = "Must have at least 2 characters"))
    )]
    pub full_name: String,
    #[cfg_attr(feature = "server", validate(custom(function = "validate_birthdate")))]
    pub birthdate: String,
    #[cfg_attr(feature = "server", validate(custom(function = "validate_country_alpha2")))]
    pub country_alpha2: String,
}
