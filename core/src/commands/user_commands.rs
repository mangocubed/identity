use cached::AsyncRedisCache;
use cached::proc_macro::io_cached;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::constants::*;
use crate::models::User;
use crate::params::{AuthenticationParams, PasswordParams, ProfileParams, UserParams};
use crate::{db_pool, jobs_storage};

use super::{AsyncRedisCacheExt, OrValidationErrors, ValidationResult, async_redis_cache, encrypt_password};

pub async fn authenticate_user<'a>(params: AuthenticationParams) -> Result<User<'a>, ValidationErrors> {
    params.validate()?;

    let user = get_user_by_username_or_email(&params.username_or_email)
        .await
        .or_validation_errors()?;

    if user.verify_password(&params.password) {
        Ok(user)
    } else {
        Err(Default::default())
    }
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<String, User<'_>>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_USER_BY_ACCESS_TOKEN_CODE).await }"##
)]
pub async fn get_user_by_access_token_code(code: String) -> sqlx::Result<User<'static>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        User,
        "SELECT us.* FROM users AS us, access_tokens AS at
        WHERE
            us.id = at.user_id AND us.disabled_at IS NULL AND at.code_expires_at > current_timestamp
            AND at.revoked_at IS NULL AND at.code = $1
        LIMIT 1",
        code, // $1
    )
    .fetch_one(db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<Uuid, User<'_>>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_USER_BY_ID).await }"##
)]
pub async fn get_user_by_id(id: Uuid) -> sqlx::Result<User<'static>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE disabled_at IS NULL AND id = $1 LIMIT 1",
        id
    )
    .fetch_one(db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ username.to_lowercase() }"#,
    ty = "AsyncRedisCache<String, User<'_>>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_USER_BY_USERNAME).await }"##
)]
async fn get_user_by_username(username: &str) -> sqlx::Result<User<'static>> {
    if username.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    let db_pool = db_pool().await;

    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE disabled_at IS NULL AND LOWER(username) = $1 LIMIT 1",
        username.to_lowercase()
    )
    .fetch_one(db_pool)
    .await
}

pub async fn get_user_by_username_or_id(username_or_id: &str) -> sqlx::Result<User<'static>> {
    if let Ok(id) = username_or_id.parse::<Uuid>() {
        get_user_by_id(id).await
    } else {
        get_user_by_username(username_or_id).await
    }
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ username_or_email.to_lowercase() }"#,
    ty = "AsyncRedisCache<String, User<'_>>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_USER_BY_USERNAME_OR_EMAIL).await }"##
)]
async fn get_user_by_username_or_email(username_or_email: &str) -> sqlx::Result<User<'static>> {
    if username_or_email.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    let db_pool = db_pool().await;

    sqlx::query_as!(
        User,
        "SELECT * FROM users
        WHERE disabled_at IS NULL AND (LOWER(username) = $1 OR LOWER(email) = $1)
        LIMIT 1",
        username_or_email.to_lowercase()
    )
    .fetch_one(db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ email.to_lowercase() }"#,
    ty = "AsyncRedisCache<String, Uuid>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_USER_ID_BY_EMAIL).await }"##
)]
async fn get_user_id_by_email(email: &str) -> sqlx::Result<Uuid> {
    if email.is_empty() {
        return Err(sqlx::Error::InvalidArgument("email".to_owned()));
    }

    let db_pool = db_pool().await;

    sqlx::query!(
        r#"SELECT id AS "id!" FROM users WHERE LOWER(email) = $1 LIMIT 1"#,
        email.to_lowercase() // $1
    )
    .fetch_one(db_pool)
    .await
    .map(|record| record.id)
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ username.to_lowercase() }"#,
    ty = "AsyncRedisCache<String, Uuid>",
    create = r##"{ async_redis_cache(CACHE_PREFIX_GET_USER_ID_BY_USERNAME).await }"##
)]
async fn get_user_id_by_username(username: &str) -> sqlx::Result<Uuid> {
    if username.is_empty() {
        return Err(sqlx::Error::InvalidArgument("username".to_owned()));
    }

    let db_pool = db_pool().await;

    sqlx::query!(
        r#"SELECT id AS "id!" FROM users WHERE LOWER(username) = $1 LIMIT 1"#,
        username.to_lowercase()
    )
    .fetch_one(db_pool)
    .await
    .map(|record| record.id)
}

pub async fn insert_user<'a>(params: UserParams) -> ValidationResult<User<'a>> {
    params.validate()?;

    let db_pool = db_pool().await;
    let display_name = params.full_name.split(' ').next().unwrap();
    let encrypted_password = encrypt_password(&params.password);

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (
            username,
            email,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            country_code
        ) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        params.username,             // $1
        params.email.to_lowercase(), // $2
        encrypted_password,          // $3
        display_name,                // $4
        params.full_name,            // $5
        params.birthdate,            // $6
        params.country_code,         // $7
    )
    .fetch_one(db_pool)
    .await
    .or_validation_errors()?;

    jobs_storage().await.push_new_user(&user).await;

    Ok(user)
}

async fn remove_user_cache(user: &User<'_>) {
    let username = user.username.to_lowercase();
    let email = user.email.to_lowercase();

    tokio::join!(
        GET_USER_BY_ID.cache_remove(CACHE_PREFIX_GET_USER_BY_ID, &user.id),
        GET_USER_BY_USERNAME.cache_remove(CACHE_PREFIX_GET_USER_BY_USERNAME, &username),
        GET_USER_BY_USERNAME_OR_EMAIL.cache_remove(CACHE_PREFIX_GET_USER_BY_USERNAME_OR_EMAIL, &username),
        GET_USER_BY_USERNAME_OR_EMAIL.cache_remove(CACHE_PREFIX_GET_USER_BY_USERNAME_OR_EMAIL, &email),
        GET_USER_ID_BY_EMAIL.cache_remove(CACHE_PREFIX_GET_USER_ID_BY_EMAIL, &email),
        GET_USER_ID_BY_USERNAME.cache_remove(CACHE_PREFIX_GET_USER_ID_BY_USERNAME, &username),
    );
}

pub async fn update_user_password(user: &User<'_>, params: PasswordParams) -> Result<(), ValidationErrors> {
    params.validate()?;

    let mut validation_errors = ValidationErrors::new();

    if !user.verify_password(&params.current_password) {
        validation_errors.add("current_password", ERROR_IS_INVALID.clone());

        return Err(validation_errors);
    }

    if params.current_password == params.new_password {
        validation_errors.add("new_password", ERROR_PASSWORD_MUST_CHANGE.clone());

        return Err(validation_errors);
    }

    let db_pool = db_pool().await;

    sqlx::query_as!(
        User,
        r#"UPDATE users SET encrypted_password = $2 WHERE disabled_at IS NULL AND id = $1"#,
        user.id,                                // $1
        encrypt_password(&params.new_password), // $2
    )
    .execute(db_pool)
    .await
    .map_err(|_| validation_errors)?;

    jobs_storage().await.push_password_changed(user).await;

    remove_user_cache(user).await;

    Ok(())
}

pub async fn update_user_profile(user: &User<'_>, params: ProfileParams) -> Result<(), ValidationErrors> {
    params.validate()?;

    let db_pool = db_pool().await;

    sqlx::query_as!(
        User,
        r#"UPDATE users SET display_name = $2, full_name = $3, birthdate = $4, country_code = $5
        WHERE disabled_at IS NULL AND id = $1"#,
        user.id,             // $1
        params.display_name, // $2
        params.full_name,    // $3
        params.birthdate,    // $4
        params.country_code  // $5
    )
    .execute(db_pool)
    .await
    .map_err(|_| ValidationErrors::new())?;

    remove_user_cache(user).await;

    Ok(())
}

pub(crate) async fn user_email_exists(email: &str) -> bool {
    get_user_id_by_email(email).await.is_ok()
}

pub(crate) async fn user_username_exists(username: &str) -> bool {
    get_user_id_by_username(username).await.is_ok()
}
