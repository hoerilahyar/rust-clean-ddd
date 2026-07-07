use anyhow::Result;
use async_trait::async_trait;

use crate::domain::system_settings::dto::{
    ListSystemSettingRequest, SystemSettingListResponse, SystemSettingResponse,
    UpsertSystemSettingRequest,
};

#[async_trait]
pub trait SystemSettingService: Send + Sync {
    async fn upsert(
        &self,
        request: UpsertSystemSettingRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<SystemSettingResponse>;

    async fn set_active(
        &self,
        key: &str,
        is_active: bool,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn delete(
        &self,
        key: &str,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn find_by_key(&self, key: &str) -> Result<SystemSettingResponse>;

    async fn list(&self, request: ListSystemSettingRequest) -> Result<SystemSettingListResponse>;
}
