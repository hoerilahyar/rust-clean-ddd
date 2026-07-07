use anyhow::Result;
use async_trait::async_trait;

use crate::domain::role::dto::{
    CreateRoleRequest, GetRoleRequest, ListRoleRequest, RoleListResponse, RoleResponse,
    UpdateRoleRequest,
};

#[async_trait]
pub trait RoleService: Send + Sync {
    async fn create(
        &self,
        request: CreateRoleRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64>;

    async fn update(
        &self,
        id: u64,
        request: UpdateRoleRequest,
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

    async fn find_by_id(&self, request: GetRoleRequest) -> Result<RoleResponse>;

    async fn list(&self, request: ListRoleRequest) -> Result<RoleListResponse>;
}
