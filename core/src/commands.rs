use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use chrono::NaiveDate;
use validator::{Validate, ValidationErrors};

use sdk::constants::ERROR_ALREADY_EXISTS;

use crate::config::{APP_CONFIG, USERS_CONFIG};
use crate::db_pool;
use crate::inputs::RegisterInput;
use crate::models::User;

async fn can_insert_user() -> bool {
    get_users_count().await.unwrap_or_default() < USERS_CONFIG.limit.into()
}

async fn email_exists(value: &str) -> bool {
    let db_pool = db_pool().await;

    sqlx::query!(
        "SELECT id FROM users WHERE LOWER(email) = $1 LIMIT 1",
        value.to_lowercase() // $1
    )
    .fetch_one(db_pool)
    .await
    .is_ok()
}

fn encrypt_password(value: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(value.as_bytes(), &salt).unwrap().to_string()
}

async fn get_users_count() -> sqlx::Result<i64> {
    let db_pool = db_pool().await;

    sqlx::query!(r#"SELECT COUNT(*) as "count!" FROM users WHERE disabled_at IS NULL"#)
        .fetch_one(db_pool)
        .await
        .map(|row| row.count)
}

pub async fn insert_user<'a>(input: &RegisterInput) -> Result<User<'a>, ValidationErrors> {
    if !can_insert_user().await {
        return Err(ValidationErrors::new());
    }

    input.validate()?;

    let mut validation_errors = ValidationErrors::new();

    if username_exists(&input.username).await {
        validation_errors.add("username", ERROR_ALREADY_EXISTS.clone());
    }

    if email_exists(&input.email).await {
        validation_errors.add("email", ERROR_ALREADY_EXISTS.clone());
    }

    if !validation_errors.is_empty() {
        return Err(validation_errors);
    }

    let db_pool = db_pool().await;
    let display_name = input.full_name.split(' ').next().unwrap();
    let birthdate = NaiveDate::parse_from_str(&input.birthdate, "%Y-%m-%d").unwrap();

    sqlx::query_as!(
        User,
        "INSERT INTO users (
            username,
            email,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            country_alpha2
        ) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        input.username,                    // $1
        input.email.to_lowercase(),        // $2
        encrypt_password(&input.password), // $3
        display_name,                      // $4
        input.full_name,                   // $5
        birthdate,                         // $6
        input.country_alpha2,              // $7
    )
    .fetch_one(db_pool)
    .await
    .map_err(|_| ValidationErrors::new())
}

async fn username_exists(value: &str) -> bool {
    let db_pool = db_pool().await;

    sqlx::query!(
        "SELECT id FROM users WHERE LOWER(username) = $1 LIMIT 1",
        value.to_lowercase()
    )
    .fetch_one(db_pool)
    .await
    .is_ok()
}

pub fn verify_app_token(app_token: &str) -> bool {
    app_token == APP_CONFIG.token
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[tokio::test]
    async fn should_find_existing_email() {
        let user = insert_test_user(None).await;

        assert!(email_exists(&user.email).await);
    }

    #[tokio::test]
    async fn should_not_find_inexistent_email() {
        let email = fake_email();
        assert!(!email_exists(&email).await);
    }

    #[tokio::test]
    async fn should_find_an_existing_username() {
        let user = insert_test_user(None).await;

        assert!(username_exists(&user.username).await);
    }

    #[tokio::test]
    async fn should_not_find_inexistent_username() {
        let username = fake_username();

        assert!(!username_exists(&username).await);
    }

    #[tokio::test]
    async fn should_insert_a_user() {
        let input = RegisterInput {
            username: fake_username(),
            email: fake_email(),
            password: fake_password(),
            full_name: fake_name(),
            birthdate: fake_birthdate(),
            country_alpha2: fake_country_alpha2(),
        };

        let result = insert_user(&input).await;

        assert!(result.is_ok());

        let user = result.unwrap();

        assert_eq!(user.username, input.username);
        assert_eq!(user.email, input.email);
        assert_eq!(user.full_name, input.full_name);
        assert_eq!(user.birthdate.to_string(), input.birthdate);
        assert_eq!(user.country_alpha2, input.country_alpha2);
    }

    #[tokio::test]
    async fn should_not_insert_a_user_with_existent_username() {
        let user = insert_test_user(None).await;
        let input = RegisterInput {
            username: user.username.to_string(),
            email: fake_email(),
            password: fake_password(),
            full_name: fake_name(),
            birthdate: fake_birthdate(),
            country_alpha2: fake_country_alpha2(),
        };

        let result = insert_user(&input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_insert_a_user_with_existent_email() {
        let user = insert_test_user(None).await;
        let input = RegisterInput {
            username: fake_username(),
            email: user.email.to_string(),
            password: fake_password(),
            full_name: fake_name(),
            birthdate: fake_birthdate(),
            country_alpha2: fake_country_alpha2(),
        };

        let result = insert_user(&input).await;

        assert!(result.is_err());
    }
}
