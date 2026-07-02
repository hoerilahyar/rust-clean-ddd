use async_trait::async_trait;

use crate::domain::role::entity::Role;

#[async_trait]
pub trait UserRoleRepository: Send + Sync {
    async fn assign(&self, user_id: u64, role_ids: &[u64]) -> anyhow::Result<()>;

    async fn revoke(&self, user_id: u64, role_id: u64) -> anyhow::Result<()>;

    async fn revoke_all(&self, user_id: u64) -> anyhow::Result<()>;

    async fn find_roles(&self, user_id: u64) -> anyhow::Result<Vec<Role>>;
}
