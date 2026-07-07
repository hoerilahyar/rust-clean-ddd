use anyhow::Result;
use async_trait::async_trait;

use crate::domain::user_setting::dto::{
    ListUserSettingRequest, UpsertUserSettingRequest, UserSettingListResponse,
    UserSettingResponse,
};

#[async_trait]
pub trait UserSettingService: Send + Sync {
    async fn upsert(
        &self,
        user_id: u64,
        request: UpsertUserSettingRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<UserSettingResponse>;

    async fn set_active(
        &self,
        user_id: u64,
        key: &str,
        is_active: bool,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn delete(
        &self,
        user_id: u64,
        key: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn find_by_key(&self, user_id: u64, key: &str) -> Result<UserSettingResponse>;

    async fn list(
        &self,
        user_id: u64,
        request: ListUserSettingRequest,
    ) -> Result<UserSettingListResponse>;
}
