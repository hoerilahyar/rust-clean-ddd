use async_trait::async_trait;

use crate::domain::role::entity::{Role, RoleFilter};

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: &Role) -> anyhow::Result<u64>;

    async fn update(&self, role: &Role) -> anyhow::Result<()>;

    async fn delete(&self, id: u64) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: u64) -> anyhow::Result<Option<Role>>;

    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<Role>>;

    async fn exists_code(&self, code: &str) -> anyhow::Result<bool>;

    async fn list(&self, filter: &RoleFilter) -> anyhow::Result<Vec<Role>>;

    async fn count(&self, filter: &RoleFilter) -> anyhow::Result<u64>;
}
