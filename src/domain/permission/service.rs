use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;

use crate::domain::permission::{
    dto::{
        CreatePermissionRequest, GetPermissionRequest, ListPermissionRequest,
        PermissionListResponse, PermissionResponse, UpdatePermissionRequest,
    },
    entity::{Permission, PermissionFilter},
    repository::PermissionRepository,
};

#[async_trait]
pub trait PermissionService: Send + Sync {
    async fn create(&self, request: CreatePermissionRequest) -> Result<u64>;

    async fn update(&self, id: u64, request: UpdatePermissionRequest) -> Result<()>;

    async fn delete(&self, id: u64) -> Result<()>;

    async fn find_by_id(&self, request: GetPermissionRequest) -> Result<PermissionResponse>;

    async fn list(&self, request: ListPermissionRequest) -> Result<PermissionListResponse>;
}

pub struct DefaultPermissionService {
    repository: Arc<dyn PermissionRepository>,
}

impl DefaultPermissionService {
    pub fn new(repository: Arc<dyn PermissionRepository>) -> Self {
        Self { repository }
    }

    fn map_response(&self, permission: Permission) -> PermissionResponse {
        PermissionResponse {
            id: permission.id,
            code: permission.code,
            name: permission.name,
            resource: permission.resource,
            action: permission.action,
            description: permission.description,
            is_active: permission.is_active,
            created_at: permission.created_at,
        }
    }
}

// IMPLEMENTATION
#[async_trait]
impl PermissionService for DefaultPermissionService {
    async fn create(&self, request: CreatePermissionRequest) -> Result<u64> {
        if self.repository.exists_code(&request.code).await? {
            return Err(anyhow!("Permission code already exists"));
        }

        let now = Utc::now();

        let permission = Permission {
            id: 0,

            code: request.code,

            name: request.name,

            resource: request.resource,

            action: request.action,

            description: request.description,

            is_active: request.is_active,

            created_at: now,

            updated_at: now,
        };

        self.repository.create(&permission).await
    }
    async fn update(&self, id: u64, request: UpdatePermissionRequest) -> Result<()> {
        let mut permission = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("Permission not found"))?;

        permission.name = request.name;

        permission.description = request.description;

        permission.is_active = request.is_active;

        permission.updated_at = Utc::now();

        self.repository.update(&permission).await
    }
    async fn delete(&self, id: u64) -> Result<()> {
        self.repository.delete(id).await
    }
    async fn find_by_id(&self, request: GetPermissionRequest) -> Result<PermissionResponse> {
        let permission = self
            .repository
            .find_by_id(request.id)
            .await?
            .ok_or_else(|| anyhow!("Permission not found"))?;

        Ok(self.map_response(permission))
    }
    async fn list(&self, request: ListPermissionRequest) -> Result<PermissionListResponse> {
        let filter = PermissionFilter {
            page: request.page.unwrap_or(1),

            page_size: request.page_size.unwrap_or(10),

            search: request.search,

            resource: request.resource,

            sort_by: request.sort_by.unwrap_or_else(|| "created_at".to_string()),

            sort_type: request.sort_type.unwrap_or_else(|| "DESC".to_string()),
        };

        let permissions = self.repository.list(&filter).await?;

        let total = self.repository.count(&filter).await?;

        let items = permissions
            .into_iter()
            .map(|p| self.map_response(p))
            .collect();

        Ok(PermissionListResponse {
            items,
            page: filter.page,
            page_size: filter.page_size,
            total,
        })
    }
}
