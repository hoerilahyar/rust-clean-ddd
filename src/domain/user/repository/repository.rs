use async_trait::async_trait;

use crate::domain::user::entity::{User, UserFilter};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> anyhow::Result<u64>;

    async fn update(&self, user: &User) -> anyhow::Result<()>;

    async fn delete(&self, id: u64) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: u64) -> anyhow::Result<Option<User>>;

    async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<User>>;

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;

    async fn exists_username(&self, username: &str) -> anyhow::Result<bool>;

    async fn exists_email(&self, email: &str) -> anyhow::Result<bool>;

    async fn list(&self, filter: &UserFilter) -> anyhow::Result<Vec<User>>;

    async fn count(&self, filter: &UserFilter) -> anyhow::Result<u64>;
}
