use anyhow::Result;
use async_trait::async_trait;

use crate::domain::permission::dto::{
    CreatePermissionRequest, GetPermissionRequest, ListPermissionRequest, PermissionListResponse,
    PermissionResponse, UpdatePermissionRequest,
};

#[async_trait]
pub trait PermissionService: Send + Sync {
    async fn create(
        &self,
        request: CreatePermissionRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64>;

    async fn update(
        &self,
        id: u64,
        request: UpdatePermissionRequest,
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

    async fn find_by_id(&self, request: GetPermissionRequest) -> Result<PermissionResponse>;

    async fn list(&self, request: ListPermissionRequest) -> Result<PermissionListResponse>;
}
