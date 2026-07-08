use async_trait::async_trait;
use redis::{AsyncCommands, aio::ConnectionManager};
use std::time::Duration;

use super::cache_repository::CacheRepository;

pub struct RedisCacheRepository {
    conn: ConnectionManager,
}

impl RedisCacheRepository {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl CacheRepository for RedisCacheRepository {
    async fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        let mut conn = self.conn.clone();
        Ok(conn.get(key).await?)
    }

    async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> anyhow::Result<()> {
        let mut conn = self.conn.clone();
        match ttl {
            Some(d) => conn.set_ex(key, value, d.as_secs()).await?,
            None => conn.set(key, value).await?,
        }
        Ok(())
    }

    async fn delete(&self, key: &str) -> anyhow::Result<()> {
        let mut conn = self.conn.clone();
        let _: () = conn.del(key).await?;
        Ok(())
    }

    async fn delete_by_prefix(&self, prefix: &str) -> anyhow::Result<()> {
        let mut conn = self.conn.clone();
        let keys: Vec<String> = conn.keys(format!("{prefix}*")).await?;
        if !keys.is_empty() {
            let _: () = conn.del(keys).await?;
        }
        Ok(())
    }
}
