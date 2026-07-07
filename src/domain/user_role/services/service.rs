use anyhow::Result;
use async_trait::async_trait;

use crate::domain::user_role::dto::{AssignUserRoleRequest, UserRoleResponse};

#[async_trait]
pub trait UserRoleService: Send + Sync {
    async fn assign(
        &self,
        user_id: u64,
        request: AssignUserRoleRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn revoke(
        &self,
        user_id: u64,
        role_id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn list(&self, user_id: u64) -> Result<Vec<UserRoleResponse>>;
}
