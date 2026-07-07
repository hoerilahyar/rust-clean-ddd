use anyhow::Result;
use async_trait::async_trait;

use crate::domain::system_settings::entity::SystemSetting;

#[async_trait]
pub trait SystemSettingRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<SystemSetting>>;
    async fn find_by_key(&self, key: &str) -> Result<Option<SystemSetting>>;
    async fn upsert(
        &self,
        key: &str,
        value: Option<String>,
        data_type: &str,
        description: Option<String>,
        is_public: bool,
    ) -> Result<SystemSetting>;
    async fn set_active(&self, key: &str, is_active: bool) -> Result<()>;
    async fn delete_by_key(&self, key: &str, delete_marker: &str) -> Result<()>;
}
