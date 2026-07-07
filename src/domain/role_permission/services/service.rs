use anyhow::Result;
use async_trait::async_trait;

use crate::domain::role_permission::dto::{AssignRolePermissionRequest, RolePermissionResponse};

#[async_trait]
pub trait RolePermissionService: Send + Sync {
    async fn assign(
        &self,
        role_id: u64,
        request: AssignRolePermissionRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn revoke(
        &self,
        role_id: u64,
        permission_id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn list(&self, role_id: u64) -> Result<Vec<RolePermissionResponse>>;
}
