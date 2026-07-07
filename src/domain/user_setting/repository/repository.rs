use anyhow::Result;
use async_trait::async_trait;

use crate::domain::user_setting::entity::UserSetting;

#[async_trait]
pub trait UserSettingRepository: Send + Sync {
    async fn find_all(&self, user_id: u64) -> Result<Vec<UserSetting>>;
    async fn find_by_key(&self, user_id: u64, key: &str) -> Result<Option<UserSetting>>;
    async fn upsert(
        &self,
        user_id: u64,
        key: &str,
        value: Option<String>,
        data_type: &str,
        description: Option<String>,
    ) -> Result<UserSetting>;
    async fn set_active(&self, user_id: u64, key: &str, is_active: bool) -> Result<()>;
    async fn delete_by_key(&self, user_id: u64, key: &str, delete_marker: &str) -> Result<()>;
}
