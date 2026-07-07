use async_trait::async_trait;

use crate::domain::{
    auth::{
        dto::RefreshTokenRequest,
        entity::{AuthUser, Permission, RefreshToken, Role},
    },
    menus::entity::Menu,
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_by_username_or_email(&self, value: &str) -> anyhow::Result<Option<AuthUser>>;
    async fn find_roles(&self, user_id: u64) -> anyhow::Result<Vec<Role>>;
    async fn find_permissions(&self, role_ids: &[u64]) -> anyhow::Result<Vec<Permission>>;
    async fn insert_refresh_token(&self, token: &RefreshToken) -> anyhow::Result<()>;
    async fn find_refresh_token(
        &self,
        token: RefreshTokenRequest,
    ) -> anyhow::Result<Option<RefreshToken>>;
    async fn revoke_refresh_token(&self, id: u64) -> anyhow::Result<()>;
    async fn revoke_all_refresh_tokens(&self, user_id: u64) -> anyhow::Result<()>;
    async fn update_last_login(&self, user_id: u64) -> anyhow::Result<()>;
    async fn find_by_id(&self, user_id: u64) -> anyhow::Result<Option<AuthUser>>;
    async fn find_menus(&self, role_ids: &[u64]) -> anyhow::Result<Vec<Menu>>;

    async fn find_active_sessions(&self, user_id: u64) -> anyhow::Result<Vec<RefreshToken>>;
    async fn find_refresh_token_by_id(&self, id: u64) -> anyhow::Result<Option<RefreshToken>>;
    async fn revoke_all_except(&self, user_id: u64, device_id: &str) -> anyhow::Result<()>;
}
