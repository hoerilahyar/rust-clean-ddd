use async_trait::async_trait;

use crate::domain::{
    menus::entity::Menu, permission::entity::Permission, role::entity::Role, user::entity::User,
};

#[async_trait]
pub trait AuthorizationRepository: Send + Sync {
    async fn find_user(&self, user_id: u64) -> anyhow::Result<Option<User>>;

    async fn find_roles(&self, user_id: u64) -> anyhow::Result<Vec<Role>>;

    async fn find_permissions(&self, role_ids: &[u64]) -> anyhow::Result<Vec<Permission>>;

    async fn find_menus(&self, role_ids: &[u64]) -> anyhow::Result<Vec<Menu>>;
}
