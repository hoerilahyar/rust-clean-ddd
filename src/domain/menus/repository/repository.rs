use async_trait::async_trait;

use crate::domain::menus::entity::{Menu, MenuFilter};

#[async_trait]
pub trait MenuRepository: Send + Sync {
    async fn create(&self, menu: &Menu) -> anyhow::Result<u64>;

    async fn update(&self, menu: &Menu) -> anyhow::Result<()>;

    async fn delete(&self, id: u64) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: u64) -> anyhow::Result<Option<Menu>>;

    async fn find_by_name(&self, name: &str) -> anyhow::Result<Option<Menu>>;

    async fn exists_name(&self, name: &str) -> anyhow::Result<bool>;

    async fn list(&self, filter: &MenuFilter) -> anyhow::Result<Vec<Menu>>;

    async fn count(&self, filter: &MenuFilter) -> anyhow::Result<u64>;
}
