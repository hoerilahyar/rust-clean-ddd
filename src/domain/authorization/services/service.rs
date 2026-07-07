use async_trait::async_trait;

use crate::domain::authorization::{
    dto::{AuthorizeRequest, AuthorizeResponse, CurrentUser},
    entity::PermissionContext,
};

#[async_trait]
pub trait AuthorizationService: Send + Sync {
    async fn authorize(&self, request: AuthorizeRequest) -> anyhow::Result<AuthorizeResponse>;

    async fn current_user(&self, user_id: u64) -> anyhow::Result<CurrentUser>;

    async fn permission_context(&self, user_id: u64) -> anyhow::Result<PermissionContext>;
}
