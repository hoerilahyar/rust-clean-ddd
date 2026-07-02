use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{
    role::entity::Role,
    user_role::{
        dto::{AssignUserRoleRequest, UserRoleResponse},
        repository::UserRoleRepository,
    },
};

#[async_trait]
pub trait UserRoleService: Send + Sync {
    async fn assign(&self, user_id: u64, request: AssignUserRoleRequest) -> Result<()>;

    async fn revoke(&self, user_id: u64, role_id: u64) -> Result<()>;

    async fn list(&self, user_id: u64) -> Result<Vec<UserRoleResponse>>;
}

pub struct DefaultUserRoleService {
    repository: Arc<dyn UserRoleRepository>,
}

impl DefaultUserRoleService {
    pub fn new(repository: Arc<dyn UserRoleRepository>) -> Self {
        Self { repository }
    }

    fn map_response(role: Role) -> UserRoleResponse {
        UserRoleResponse {
            role_id: role.id,
            code: role.code,
            name: role.name,
        }
    }
}

#[async_trait]
impl UserRoleService for DefaultUserRoleService {
    async fn assign(&self, user_id: u64, request: AssignUserRoleRequest) -> Result<()> {
        self.repository.assign(user_id, &request.role_ids).await
    }

    async fn revoke(&self, user_id: u64, role_id: u64) -> Result<()> {
        self.repository.revoke(user_id, role_id).await
    }

    async fn list(&self, user_id: u64) -> Result<Vec<UserRoleResponse>> {
        let roles = self.repository.find_roles(user_id).await?;

        Ok(roles.into_iter().map(Self::map_response).collect())
    }
}
