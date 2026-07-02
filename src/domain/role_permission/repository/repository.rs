use async_trait::async_trait;

use crate::domain::role_permission::entity::RolePermission;

#[async_trait]
pub trait RolePermissionRepository: Send + Sync {
    async fn assign(&self, role_id: u64, permission_ids: &[u64]) -> anyhow::Result<()>;

    async fn revoke(&self, role_id: u64, permission_id: u64) -> anyhow::Result<()>;

    async fn revoke_all(&self, role_id: u64) -> anyhow::Result<()>;

    async fn find_permissions(&self, role_id: u64) -> anyhow::Result<Vec<RolePermission>>;
}
