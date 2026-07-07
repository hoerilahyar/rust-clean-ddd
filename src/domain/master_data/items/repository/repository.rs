use async_trait::async_trait;

use crate::domain::master_data::items::entity::{MasterDataItem, MasterDataItemFilter};

#[async_trait]
pub trait MasterDataItemsRepository: Send + Sync {
    async fn create_item(&self, item: &MasterDataItem) -> anyhow::Result<u64>;

    async fn update_item(&self, item: &MasterDataItem) -> anyhow::Result<()>;

    async fn delete_item(&self, id: u64) -> anyhow::Result<()>;

    async fn find_item_by_id(&self, id: u64) -> anyhow::Result<Option<MasterDataItem>>;

    async fn find_item_by_code(
        &self,
        group_id: u64,
        code: &str,
    ) -> anyhow::Result<Option<MasterDataItem>>;

    async fn exists_item_code(&self, group_id: u64, code: &str) -> anyhow::Result<bool>;

    async fn list_items(
        &self,
        filter: &MasterDataItemFilter,
    ) -> anyhow::Result<Vec<MasterDataItem>>;

    async fn count_items(&self, filter: &MasterDataItemFilter) -> anyhow::Result<u64>;

    async fn list_options(
        &self,
        group_id: u64,
        parent_id: Option<u64>,
        only_root: bool,
    ) -> anyhow::Result<Vec<MasterDataItem>>;

    async fn count_items_in_group(&self, group_id: u64) -> anyhow::Result<u64>;

    async fn count_children(&self, item_id: u64) -> anyhow::Result<u64>;
}
