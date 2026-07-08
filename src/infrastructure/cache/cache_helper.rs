use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use std::time::Duration;

use super::cache_repository::CacheRepository;

#[derive(Clone)]
pub struct CacheHelper {
    inner: Arc<dyn CacheRepository>,
}

impl CacheHelper {
    pub fn new(inner: Arc<dyn CacheRepository>) -> Self {
        Self { inner }
    }

    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self.inner.get(key).await {
            Ok(Some(raw)) => match serde_json::from_str(&raw) {
                Ok(value) => Some(value),
                Err(err) => {
                    tracing::warn!(key, error = %err, "cache deserialize failed");
                    None
                }
            },
            Ok(None) => None,
            Err(err) => {
                tracing::warn!(key, error = %err, "cache get failed");
                None
            }
        }
    }

    pub async fn set_json<T: Serialize + Sync>(&self, key: &str, value: &T, ttl: Option<Duration>) {
        match serde_json::to_string(value) {
            Ok(json) => {
                if let Err(err) = self.inner.set(key, &json, ttl).await {
                    tracing::warn!(key, error = %err, "cache set failed");
                }
            }
            Err(err) => tracing::warn!(key, error = %err, "cache serialize failed"),
        }
    }

    pub async fn invalidate(&self, key: &str) {
        if let Err(err) = self.inner.delete(key).await {
            tracing::warn!(key, error = %err, "cache invalidate failed");
        }
    }

    pub async fn invalidate_prefix(&self, prefix: &str) {
        if let Err(err) = self.inner.delete_by_prefix(prefix).await {
            tracing::warn!(prefix, error = %err, "cache invalidate prefix failed");
        }
    }
}
