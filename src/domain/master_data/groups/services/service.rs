use crate::domain::master_data::groups::dto::{
    CreateMasterDataGroupRequest, ListMasterDataGroupRequest, MasterDataGroupListResponse,
    MasterDataGroupResponse, UpdateMasterDataGroupRequest,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MasterDataGroupService: Send + Sync {
    async fn create_group(
        &self,
        request: CreateMasterDataGroupRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64>;

    async fn update_group(
        &self,
        code: &str,
        request: UpdateMasterDataGroupRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn delete_group(
        &self,
        code: &str,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn find_group_by_code(&self, code: &str) -> Result<MasterDataGroupResponse>;

    async fn list_groups(
        &self,
        request: ListMasterDataGroupRequest,
    ) -> Result<MasterDataGroupListResponse>;
}
