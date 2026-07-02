use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::domain::role_permission::{
    dto::{AssignRolePermissionRequest, RolePermissionResponse},
    repository::RolePermissionRepository,
};

#[async_trait]
pub trait RolePermissionService: Send + Sync {
    async fn assign(&self, role_id: u64, request: AssignRolePermissionRequest) -> Result<()>;

    async fn revoke(&self, role_id: u64, permission_id: u64) -> Result<()>;

    async fn list(&self, role_id: u64) -> Result<Vec<RolePermissionResponse>>;
}

pub struct DefaultRolePermissionService {
    repository: Arc<dyn RolePermissionRepository>,
}

impl DefaultRolePermissionService {
    pub fn new(repository: Arc<dyn RolePermissionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl RolePermissionService for DefaultRolePermissionService {
    async fn assign(&self, role_id: u64, request: AssignRolePermissionRequest) -> Result<()> {
        self.repository
            .assign(role_id, &request.permission_ids)
            .await
    }

    async fn revoke(&self, role_id: u64, permission_id: u64) -> Result<()> {
        self.repository.revoke(role_id, permission_id).await
    }

    async fn list(&self, role_id: u64) -> Result<Vec<RolePermissionResponse>> {
        let items = self.repository.find_permissions(role_id).await?;

        Ok(items
            .into_iter()
            .map(|item| RolePermissionResponse {
                role_id: item.role_id,
                permission_id: item.permission_id,
            })
            .collect())
    }
}
