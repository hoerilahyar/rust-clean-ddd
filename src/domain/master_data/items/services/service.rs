use anyhow::Result;
use async_trait::async_trait;

use crate::domain::master_data::{
    groups::dto::MasterDataOptionListResponse,
    items::dto::{
        CreateMasterDataItemRequest, ListMasterDataItemRequest, MasterDataItemListResponse,
        MasterDataItemResponse, UpdateMasterDataItemRequest,
    },
};

#[async_trait]
pub trait MasterDataItemsService: Send + Sync {
    async fn create_item(
        &self,
        group_code: &str,
        request: CreateMasterDataItemRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64>;

    async fn update_item(
        &self,
        group_code: &str,
        item_id: u64,
        request: UpdateMasterDataItemRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn delete_item(
        &self,
        group_code: &str,
        item_id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn find_item_by_id(
        &self,
        group_code: &str,
        item_id: u64,
    ) -> Result<MasterDataItemResponse>;

    async fn list_items(
        &self,
        group_code: &str,
        request: ListMasterDataItemRequest,
    ) -> Result<MasterDataItemListResponse>;

    async fn list_options(
        &self,
        group_code: &str,
        parent_id: Option<u64>,
        only_root: bool,
    ) -> Result<MasterDataOptionListResponse>;
}
