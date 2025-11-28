use validator::{Validate, ValidationErrors};

use crate::db_pool;
use crate::inputs::LoginInput;
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
}
