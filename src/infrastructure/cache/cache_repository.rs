use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait CacheRepository: Send + Sync {
    async fn get(&self, key: &str) -> anyhow::Result<Option<String>>;
    async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> anyhow::Result<()>;
    async fn delete(&self, key: &str) -> anyhow::Result<()>;
    async fn delete_by_prefix(&self, prefix: &str) -> anyhow::Result<()>; // buat invalidate list
}
