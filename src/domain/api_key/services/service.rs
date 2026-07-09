use anyhow::Result;
use async_trait::async_trait;

use crate::domain::api_key::{
    dto::{
        ApiKeyListResponse, ApiKeyResponse, CreateApiKeyRequest, CreateApiKeyResponse,
        ListApiKeyRequest, UpdateApiKeyRequest,
    },
    entity::ApiKey,
};

#[async_trait]
pub trait ApiKeyService: Send + Sync {
    /// `actor_permissions` is the caller's own permission set. A key can
    /// only be granted permissions the actor already holds, so this
    /// endpoint can't be used to mint a key with more access than the
    /// person creating it actually has.
    async fn create(
        &self,
        request: CreateApiKeyRequest,
        actor_id: Option<u64>,
        actor_permissions: &[String],
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<CreateApiKeyResponse>;

    async fn list(&self, request: ListApiKeyRequest) -> Result<ApiKeyListResponse>;

    async fn find_by_id(&self, id: u64) -> Result<ApiKeyResponse>;

    async fn update(
        &self,
        id: u64,
        request: UpdateApiKeyRequest,
        actor_id: Option<u64>,
        actor_permissions: &[String],
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<ApiKeyResponse>;

    async fn set_active(
        &self,
        id: u64,
        is_active: bool,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn delete(
        &self,
        id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    /// Verifies a raw `rak_<prefix>.<secret>` key from an incoming request.
    /// Returns the matching, usable (active & unexpired) key on success.
    async fn verify(&self, full_key: &str) -> Result<Option<ApiKey>>;

    /// Best-effort bookkeeping call after a successful `verify`.
    async fn record_usage(&self, id: u64);
}
