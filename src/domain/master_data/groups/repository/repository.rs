use async_trait::async_trait;

use crate::domain::master_data::groups::entity::{MasterDataGroup, MasterDataGroupFilter};

#[async_trait]
pub trait MasterDataGroupRepository: Send + Sync {
    async fn create_group(&self, group: &MasterDataGroup) -> anyhow::Result<u64>;

    async fn update_group(&self, group: &MasterDataGroup) -> anyhow::Result<()>;

    async fn delete_group(&self, id: u64) -> anyhow::Result<()>;

    async fn find_group_by_id(&self, id: u64) -> anyhow::Result<Option<MasterDataGroup>>;

    async fn find_group_by_code(&self, code: &str) -> anyhow::Result<Option<MasterDataGroup>>;

    async fn exists_group_code(&self, code: &str) -> anyhow::Result<bool>;

    async fn list_groups(
        &self,
        filter: &MasterDataGroupFilter,
    ) -> anyhow::Result<Vec<MasterDataGroup>>;

    async fn count_groups(&self, filter: &MasterDataGroupFilter) -> anyhow::Result<u64>;
}
