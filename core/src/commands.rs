use std::fmt::Display;

use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use cached::proc_macro::io_cached;
use cached::{AsyncRedisCache, IOCachedAsync};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::sync::OnceCell;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::config::CACHE_CONFIG;
use crate::constants::{CACHE_PREFIX_GET_USER_ID_BY_EMAIL, CACHE_PREFIX_GET_USER_ID_BY_USERNAME, CACHE_PREFIX_GET_USER_BY_ID};
use crate::models::User;
use crate::params::UserParams;
use crate::{db_pool, jobs_storage};

trait OrValidationErrors<T> {
    fn or_validation_errors(self) -> Result<T, ValidationErrors>;
}

impl<T> OrValidationErrors<T> for Result<T, sqlx::Error> {
    fn or_validation_errors(self) -> Result<T, ValidationErrors> {
        self.map_err(|_| ValidationErrors::new())
    }
}

trait AsyncRedisCacheExt<K> {
    async fn cache_remove(&self, prefix: &str, key: &K);
}

impl<K, V> AsyncRedisCacheExt<K> for OnceCell<AsyncRedisCache<K, V>>
where
    K: Display + Send + Sync,
    V: DeserializeOwned + Display + Send + Serialize + Sync,
{
    async fn cache_remove(&self, prefix: &str, key: &K) {
        let _ = self
            .get_or_init(|| async { async_redis_cache(prefix).await })
            .await
            .cache_remove(key)
            .await;
    }
}

async fn async_redis_cache<K, V>(prefix: &str) -> AsyncRedisCache<K, V>
where
    K: Display + Send + Sync,
    V: DeserializeOwned + Display + Send + Serialize + Sync,
{
    AsyncRedisCache::new(format!("{prefix}:"), CACHE_CONFIG.ttl())
        .set_connection_string(&CACHE_CONFIG.redis_url)
        .set_refresh(true)
        .build()
        .await
        .expect("Could not get redis cache")
}

fn encrypt_password(value: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(value.as_bytes(), &salt).unwrap().to_string()
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

pub async fn insert_user(params: UserParams) -> Result<(), ValidationErrors> {
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

    Ok(())
}

pub(crate) async fn user_email_exists(email: &str) -> bool {
    get_user_id_by_email(email).await.is_ok()
}

pub(crate) async fn user_username_exists(username: &str) -> bool {
    get_user_id_by_username(username).await.is_ok()
}
