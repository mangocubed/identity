use std::fmt::Display;

use cached::{AsyncRedisCache, IOCachedAsync};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::sync::OnceCell;
use validator::ValidationErrors;

use crate::config::CACHE_CONFIG;

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
