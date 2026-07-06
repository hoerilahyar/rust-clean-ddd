use async_trait::async_trait;

use crate::domain::menu_permissions::entity::MenuPermission;

#[async_trait]
pub trait MenuPermissionRepository: Send + Sync {
    async fn assign(&self, role_id: u64, permission_ids: &[u64]) -> anyhow::Result<()>;

    async fn revoke(&self, role_id: u64, permission_id: u64) -> anyhow::Result<()>;

    async fn revoke_all(&self, role_id: u64) -> anyhow::Result<()>;

    async fn find_permissions(&self, role_id: u64) -> anyhow::Result<Vec<MenuPermission>>;
}
