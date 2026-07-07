use anyhow::Result;
use async_trait::async_trait;

use crate::domain::menu_permissions::dto::{AssignMenuPermissionRequest, MenuPermissionResponse};

#[async_trait]
pub trait MenuPermissionService: Send + Sync {
    async fn assign(
        &self,
        menu_id: u64,
        request: AssignMenuPermissionRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn revoke(
        &self,
        menu_id: u64,
        permission_id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn list(&self, menu_id: u64) -> Result<Vec<MenuPermissionResponse>>;
}
