use chrono::{NaiveDate, Utc};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::commands;
use crate::constants::{ERROR_ALREADY_EXISTS, ERROR_IS_INVALID, REGEX_USERNAME};

fn validate_birthdate(value: &NaiveDate) -> Result<(), ValidationError> {
    if *value > Utc::now().date_naive() {
        return Err(ERROR_IS_INVALID.clone());
    }

    Ok(())
}

fn validate_country_code(value: &str) -> Result<(), ValidationError> {
    use rust_iso3166::ALL_ALPHA2;

    if !ALL_ALPHA2.contains(&value) {
        return Err(ERROR_IS_INVALID.clone());
    }

    Ok(())
}

fn validate_email(value: &str) -> Result<(), ValidationError> {
    if crate::block_on(commands::user_email_exists(value)) {
        return Err(ERROR_ALREADY_EXISTS.clone());
    }

    Ok(())
}

fn validate_username(value: &str) -> Result<(), ValidationError> {
    if uuid::Uuid::try_parse(value).is_ok() {
        return Err(ERROR_IS_INVALID.clone());
    }

    if crate::block_on(commands::user_username_exists(value)) {
        return Err(ERROR_ALREADY_EXISTS.clone());
    }

    Ok(())
}

#[derive(Validate)]
pub struct ApplicationParams {
    #[validate(length(min = 1, max = 255, message = "Can't be blank"))]
    pub name: String,
    #[validate(url(message = "Is invalid"))]
    pub redirect_url: String,
}

#[derive(Validate)]
pub struct AuthenticationParams {
    #[validate(length(min = 1, max = 255, message = "Can't be blank"))]
    pub username_or_email: String,
    #[validate(length(min = 1, max = 255, message = "Can't be blank"))]
    pub password: String,
}

#[derive(Validate)]
pub struct ConfirmationParams {
    #[validate(length(min = 1, max = 255, message = "Can't be blank"))]
    pub confirmation_code: String,
}

#[derive(Validate)]
pub struct EmailParams {
    #[validate(
        length(min = 5, max = 255, message = "Must have at least 5 characters"),
        email(message = "Is invalid")
    )]
    pub email: String,
    #[validate(length(min = 1, max = 255, message = "Can't be blank"))]
    pub password: String,
}

#[derive(Validate)]
pub struct PasswordParams {
    #[validate(length(min = 1, max = 255, message = "Can't be blank"))]
    pub current_password: String,
    #[validate(length(min = 6, max = 128, message = "Must have at least 6 characters"))]
    pub new_password: String,
}

#[derive(Validate)]
pub struct ProfileParams {
    #[validate(length(min = 2, max = 255, message = "Must have at least 2 characters"))]
    pub display_name: String,
    #[validate(length(min = 2, max = 255, message = "Must have at least 2 characters"))]
    pub full_name: String,
    #[validate(required(message = "Can't be blank"), custom(function = "validate_birthdate"))]
    pub birthdate: Option<NaiveDate>,
    #[validate(custom(function = "validate_country_code"))]
    pub country_code: String,
}

#[derive(Validate)]
pub struct ResetPasswordParams {
    pub confirmation_id: Uuid,
    #[validate(length(min = 1, max = 255, message = "Can't be blank"))]
    pub confirmation_code: String,
    #[validate(length(min = 6, max = 128, message = "Must have at least 6 characters"))]
    pub new_password: String,
}

#[derive(Validate)]
pub struct UserParams {
    #[validate(
        length(min = 3, max = 16, message = "Must have at least 3 characters"),
        regex(path = *REGEX_USERNAME, message = "Is invalid"),
        custom(function = "validate_username")
    )]
    pub username: String,
    #[validate(
        length(min = 5, max = 255, message = "Must have at least 5 characters"),
        email(message = "Is invalid"),
        custom(function = "validate_email")
    )]
    pub email: String,
    #[validate(length(min = 6, max = 128, message = "Must have at least 6 characters"))]
    pub password: String,
    #[validate(length(min = 2, max = 255, message = "Must have at least 2 characters"))]
    pub full_name: String,
    #[validate(required(message = "Can't be blank"), custom(function = "validate_birthdate"))]
    pub birthdate: Option<NaiveDate>,
    #[validate(custom(function = "validate_country_code"))]
    pub country_code: String,
}
