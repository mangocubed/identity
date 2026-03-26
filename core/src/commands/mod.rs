use std::fmt::Display;

use ab_glyph::{FontRef, PxScale};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use cached::{AsyncRedisCache, IOCachedAsync};
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use rand::distr::Alphanumeric;
use rand::distr::uniform::SampleRange;
use rand::{RngExt, rng};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::sync::OnceCell;
use validator::ValidationErrors;

use crate::config::{CACHE_CONFIG, STORAGE_CONFIG};

mod access_token_commands;
mod application_commands;
mod authorization_commands;
mod confirmation_commands;
mod session_commands;
mod user_commands;

pub use access_token_commands::*;
pub use application_commands::*;
pub use authorization_commands::*;
pub use confirmation_commands::*;
pub use session_commands::*;
pub use user_commands::*;

pub type ValidationResult<T = ()> = Result<T, ValidationErrors>;

trait OrValidationErrors<T> {
    fn or_validation_errors(self) -> ValidationResult<T>;
}

impl<T> OrValidationErrors<T> for Result<T, sqlx::Error> {
    fn or_validation_errors(self) -> ValidationResult<T> {
        self.map_err(|_| Default::default())
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

fn generate_random_string<R: SampleRange<u8>>(length: R) -> String {
    let mut rng = rng();

    let length = rng.random_range(length);

    rng.sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}

pub(crate) fn generate_text_icon(text: &str, size: u32) -> anyhow::Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let text = &text[0..2].to_uppercase();
    let mut rgb_image = RgbImage::new(size, size);

    draw_filled_rect_mut(
        &mut rgb_image,
        Rect::at(0, 0).of_size(size, size),
        Rgb([111u8, 111u8, 111u8]),
    );

    let font_file = std::fs::read(&STORAGE_CONFIG.font_path)?;
    let font = FontRef::try_from_slice(&font_file)?;
    let scale = PxScale::from(size as f32 / 1.7);
    let (text_width, _) = text_size(scale, &font, text);
    let x = ((size - text_width) / 2) as i32;
    let y = (size as f32 / 4.6) as i32;

    draw_text_mut(&mut rgb_image, Rgb([225u8, 225u8, 225u8]), x, y, scale, &font, text);

    Ok(rgb_image)
}

pub(crate) fn verify_password(encrypted_password: &str, password: &str) -> bool {
    let argon2 = Argon2::default();

    let Ok(password_hash) = PasswordHash::new(encrypted_password) else {
        return false;
    };

    argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
}
