use std::fmt::Display;

use ab_glyph::{FontRef, PxScale};
use cached::{AsyncRedisCache, IOCachedAsync};
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::sync::OnceCell;
use validator::ValidationErrors;

use crate::config::{CACHE_CONFIG, STORAGE_CONFIG};

mod session_commands;
mod user_commands;

pub use session_commands::*;
pub use user_commands::*;

trait OrValidationErrors<T> {
    fn or_validation_errors(self) -> Result<T, ValidationErrors>;
}

impl<T> OrValidationErrors<T> for Result<T, sqlx::Error> {
    fn or_validation_errors(self) -> Result<T, ValidationErrors> {
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

pub fn generate_text_icon(text: &str, size: u32) -> anyhow::Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
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
