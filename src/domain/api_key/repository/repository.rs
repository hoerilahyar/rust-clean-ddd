use anyhow::Result;
use async_trait::async_trait;

use crate::domain::api_key::entity::ApiKey;

#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn create(
        &self,
        name: &str,
        key_prefix: &str,
        key_hash: &str,
        permissions: &[String],
        expires_at: Option<chrono::NaiveDateTime>,
        created_by: Option<u64>,
    ) -> Result<ApiKey>;

    async fn find_all(&self) -> Result<Vec<ApiKey>>;

    async fn find_by_id(&self, id: u64) -> Result<Option<ApiKey>>;

    /// Looked up on every authenticated request via the public prefix.
    async fn find_by_prefix(&self, key_prefix: &str) -> Result<Option<ApiKey>>;

    async fn update(
        &self,
        id: u64,
        name: &str,
        permissions: &[String],
        expires_at: Option<chrono::NaiveDateTime>,
    ) -> Result<()>;

    async fn set_active(&self, id: u64, is_active: bool) -> Result<()>;

    async fn touch_last_used(&self, id: u64) -> Result<()>;

    async fn delete_by_id(&self, id: u64, delete_marker: &str) -> Result<()>;
}
