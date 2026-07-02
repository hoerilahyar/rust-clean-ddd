use async_trait::async_trait;

use crate::domain::permission::entity::{Permission, PermissionFilter};

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn create(&self, permission: &Permission) -> anyhow::Result<u64>;

    async fn update(&self, permission: &Permission) -> anyhow::Result<()>;

    async fn delete(&self, id: u64) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: u64) -> anyhow::Result<Option<Permission>>;

    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<Permission>>;

    async fn exists_code(&self, code: &str) -> anyhow::Result<bool>;

    async fn list(&self, filter: &PermissionFilter) -> anyhow::Result<Vec<Permission>>;

    async fn count(&self, filter: &PermissionFilter) -> anyhow::Result<u64>;
}
