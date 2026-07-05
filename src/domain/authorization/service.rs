use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::domain::authorization::{
    dto::{AuthorizeRequest, AuthorizeResponse, CurrentUser},
    entity::PermissionContext,
    repository::AuthorizationRepository,
};

#[async_trait]
pub trait AuthorizationService: Send + Sync {
    async fn authorize(&self, request: AuthorizeRequest) -> anyhow::Result<AuthorizeResponse>;

    async fn current_user(&self, user_id: u64) -> anyhow::Result<CurrentUser>;

    async fn permission_context(&self, user_id: u64) -> anyhow::Result<PermissionContext>;
}

pub struct DefaultAuthorizationService {
    repository: Arc<dyn AuthorizationRepository>,
}

impl DefaultAuthorizationService {
    pub fn new(repository: Arc<dyn AuthorizationRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl AuthorizationService for DefaultAuthorizationService {
    async fn authorize(&self, request: AuthorizeRequest) -> Result<AuthorizeResponse> {
        let user = self
            .repository
            .find_user(request.user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        let roles = self.repository.find_roles(user.id).await?;

        let role_ids = roles.iter().map(|r| r.id).collect::<Vec<_>>();

        let permissions = self.repository.find_permissions(&role_ids).await?;

        Ok(AuthorizeResponse {
            context: PermissionContext {
                user_id: user.id,
                username: user.username,
                fullname: user.fullname,
                roles: roles.iter().map(|r| r.code.clone()).collect(),
                permissions: permissions.iter().map(|p| p.code.clone()).collect(),
            },
        })
    }

    async fn current_user(&self, user_id: u64) -> Result<CurrentUser> {
        let authorize = self.authorize(AuthorizeRequest { user_id }).await?;

        Ok(CurrentUser {
            id: authorize.context.user_id,
            username: authorize.context.username,
            fullname: authorize.context.fullname,
            roles: authorize.context.roles,
            permissions: authorize.context.permissions,
        })
    }

    async fn permission_context(&self, user_id: u64) -> Result<PermissionContext> {
        Ok(self.authorize(AuthorizeRequest { user_id }).await?.context)
    }
}
