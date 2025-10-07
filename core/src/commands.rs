use std::net::IpAddr;

use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::NaiveDate;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use sdk::constants::ERROR_ALREADY_EXISTS;
use sdk::generate_random_string;

use crate::config::USERS_CONFIG;
use crate::db_pool;
use crate::inputs::{LoginInput, RegisterInput};
use crate::jobs_storage::jobs_storage;
use crate::models::{Session, User};

pub async fn authenticate_user<'a>(input: &LoginInput) -> Result<User<'a>, ValidationErrors> {
    input.validate()?;

    let db_pool = db_pool().await;

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE disabled_at IS NULL AND (LOWER(username) = $1 OR LOWER(email) = $1)
        LIMIT 1",
        input.username_or_email.to_lowercase()
    )
    .fetch_one(db_pool)
    .await
    .map_err(|_| ValidationErrors::new())?;

    if user.verify_password(&input.password) {
        Ok(user)
    } else {
        Err(ValidationErrors::new())
    }
}

pub async fn can_insert_user() -> bool {
    get_users_count().await.unwrap_or_default() < USERS_CONFIG.limit.into()
}

pub async fn finish_session(session: &Session<'_>) -> sqlx::Result<()> {
    let db_pool = db_pool().await;

    sqlx::query!(
        "UPDATE sessions SET finished_at = current_timestamp WHERE finished_at IS NULL AND id = $1",
        session.id
    )
    .execute(db_pool)
    .await
    .map(|_| ())
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

pub async fn get_session_by_id<'a>(id: Uuid) -> sqlx::Result<Session<'a>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Session,
        "SELECT * FROM sessions WHERE finished_at IS NULL AND id = $1 LIMIT 1",
        id
    )
    .fetch_one(db_pool)
    .await
}

pub async fn get_session_by_token<'a>(token: &str) -> sqlx::Result<Session<'a>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Session,
        "SELECT * FROM sessions WHERE finished_at IS NULL AND token = $1 LIMIT 1",
        token
    )
    .fetch_one(db_pool)
    .await
}

pub async fn get_user_by_id<'a>(id: Uuid) -> sqlx::Result<User<'a>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 LIMIT 1", id)
        .fetch_one(db_pool)
        .await
}

pub async fn get_user_by_session_token<'a>(token: &str) -> sqlx::Result<User<'a>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = (SELECT user_id FROM sessions WHERE finished_at IS NULL AND token = $1 LIMIT 1)
        LIMIT 1",
        token
    )
    .fetch_one(db_pool)
    .await
}

async fn get_users_count() -> sqlx::Result<i64> {
    let db_pool = db_pool().await;

    sqlx::query!(r#"SELECT COUNT(*) as "count!" FROM users WHERE disabled_at IS NULL"#)
        .fetch_one(db_pool)
        .await
        .map(|row| row.count)
}

pub async fn insert_session<'a>(user: &User<'_>, user_agent: &str, ip_addr: IpAddr) -> sqlx::Result<Session<'a>> {
    let db_pool = db_pool().await;

    let token = generate_random_string(USERS_CONFIG.session_token_length);

    let result = sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id, token, user_agent) VALUES ($1, $2, $3) RETURNING *",
        user.id,    // $1
        token,      // $2
        user_agent  // $3
    )
    .fetch_one(db_pool)
    .await;

    match result {
        Ok(session) => {
            jobs_storage().await.push_new_session(&session, ip_addr).await;

            Ok(session)
        }
        Err(err) => Err(err),
    }
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

    let result = sqlx::query_as!(
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
    .await;

    match result {
        Ok(user) => {
            jobs_storage().await.push_new_user(&user).await;

            Ok(user)
        }
        Err(_) => Err(ValidationErrors::new()),
    }
}

pub async fn update_session_location<'a>(
    session: &Session<'_>,
    country_alpha2: &str,
    region: &str,
    city: &str,
) -> sqlx::Result<Session<'a>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Session,
        "UPDATE sessions SET country_alpha2 = $2, region = $3, city = $4 WHERE finished_at IS NULL AND id = $1
        RETURNING *",
        session.id,     // $1
        country_alpha2, // $2
        region,         // $3
        city            // $4
    )
    .fetch_one(db_pool)
    .await
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

pub(crate) fn verify_password(encrypted_password: &str, password: &str) -> bool {
    let argon2 = Argon2::default();

    let Ok(password_hash) = PasswordHash::new(encrypted_password) else {
        return false;
    };

    argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
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
