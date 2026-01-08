use chrono::NaiveDate;
use validator::{Validate, ValidationErrors};

use crate::db_pool;
use crate::inputs::{LoginInput, UserProfileInput};
use crate::models::User;

pub async fn authenticate_user<'a>(input: &LoginInput) -> Result<User<'a>, ValidationErrors> {
    input.validate()?;

    let user = get_user_by_username_or_email(&input.username_or_email)
        .await
        .map_err(|_| ValidationErrors::new())?;

    if user.verify_password(&input.password) {
        Ok(user)
    } else {
        Err(ValidationErrors::new())
    }
}

pub async fn get_user_by_username_or_email<'a>(username_or_email: &str) -> sqlx::Result<User<'a>> {
    if username_or_email.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    let db_pool = db_pool().await;

    sqlx::query_as!(
        User,
        "SELECT * FROM users
        WHERE disabled_at IS NULL AND (LOWER(username) = $1 OR (email_confirmed_at IS NOT NULL AND LOWER(email) = $1))
        LIMIT 1",
        username_or_email.to_lowercase()
    )
    .fetch_one(db_pool)
    .await
}

pub async fn update_user_profile(user: &User<'_>, input: &UserProfileInput) -> Result<(), ValidationErrors> {
    input.validate()?;

    let db_pool = db_pool().await;
    let birthdate = NaiveDate::parse_from_str(&input.birthdate, "%Y-%m-%d").unwrap();

    let result = sqlx::query_as!(
        User,
        r#"UPDATE users SET full_name = $2, birthdate = $3, country_alpha2 = $4 WHERE disabled_at IS NULL AND id = $1"#,
        user.id,              // $1
        input.full_name,      // $2
        birthdate,            // $3
        input.country_alpha2  // $4
    )
    .execute(db_pool)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationErrors::new()),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[tokio::test]
    async fn should_authenticate_user() {
        let password = fake_password();
        let user = insert_test_user(Some(&password)).await;
        let input = LoginInput {
            username_or_email: user.username.to_string(),
            password: password.clone(),
        };

        let result = authenticate_user(&input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_authenticate_user_with_invalid_username() {
        let password = fake_password();
        let _ = insert_test_user(Some(&password)).await;
        let input = LoginInput {
            username_or_email: fake_username(),
            password: password.clone(),
        };

        let result = authenticate_user(&input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_authenticate_user_with_invalid_password() {
        let user = insert_test_user(None).await;
        let input = LoginInput {
            username_or_email: user.username.to_string(),
            password: fake_password(),
        };

        let result = authenticate_user(&input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_update_user_profile() {
        let user = insert_test_user(None).await;
        let input = UserProfileInput {
            full_name: fake_name(),
            birthdate: fake_birthdate().to_string(),
            country_alpha2: fake_country_alpha2(),
        };

        let result = update_user_profile(&user, &input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_update_user_profile_with_invalid_input() {
        let user = insert_test_user(None).await;
        let input = UserProfileInput {
            full_name: String::new(),
            birthdate: String::new(),
            country_alpha2: String::new(),
        };

        let result = update_user_profile(&user, &input).await;

        assert!(result.is_err());
    }
}
