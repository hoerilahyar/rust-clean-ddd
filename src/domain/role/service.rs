use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;

use crate::domain::role::{
    dto::{
        CreateRoleRequest, GetRoleRequest, ListRoleRequest, RoleListResponse, RoleResponse,
        UpdateRoleRequest,
    },
    entity::{Role, RoleFilter},
    repository::RoleRepository,
};

#[async_trait]
pub trait RoleService: Send + Sync {
    async fn create(&self, request: CreateRoleRequest) -> Result<u64>;

    async fn update(&self, id: u64, request: UpdateRoleRequest) -> Result<()>;

    async fn delete(&self, id: u64) -> Result<()>;

    async fn find_by_id(&self, request: GetRoleRequest) -> Result<RoleResponse>;

    async fn list(&self, request: ListRoleRequest) -> Result<RoleListResponse>;
}

pub struct DefaultRoleService {
    repository: Arc<dyn RoleRepository>,
}

impl DefaultRoleService {
    pub fn new(repository: Arc<dyn RoleRepository>) -> Self {
        Self { repository }
    }
    fn map_response(&self, role: Role) -> RoleResponse {
        RoleResponse {
            id: role.id,
            code: role.code,
            name: role.name,
            description: role.description,
            is_active: role.is_active,
            created_at: role.created_at,
        }
    }
}

// IMPLEMENTATION
#[async_trait]
impl RoleService for DefaultRoleService {
    async fn create(&self, request: CreateRoleRequest) -> Result<u64> {
        if self.repository.exists_code(&request.code).await? {
            return Err(anyhow!("Role code already exists"));
        }

        let now = Utc::now();

        let role = Role {
            id: 0,

            code: request.code,

            name: request.name,

            description: request.description,

            is_active: request.is_active,

            created_at: now,

            updated_at: now,
        };

        self.repository.create(&role).await
    }
    async fn update(&self, id: u64, request: UpdateRoleRequest) -> Result<()> {
        let mut role = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("Role not found"))?;

        role.name = request.name;

        role.description = request.description;

        role.is_active = request.is_active;

        role.updated_at = Utc::now();

        self.repository.update(&role).await
    }
    async fn delete(&self, id: u64) -> Result<()> {
        self.repository.delete(id).await
    }
    async fn find_by_id(&self, request: GetRoleRequest) -> Result<RoleResponse> {
        let role = self
            .repository
            .find_by_id(request.id)
            .await?
            .ok_or_else(|| anyhow!("Role not found"))?;

        Ok(self.map_response(role))
    }
    async fn list(&self, request: ListRoleRequest) -> Result<RoleListResponse> {
        let filter = RoleFilter {
            page: request.page.unwrap_or(1),

            page_size: request.page_size.unwrap_or(10),

            search: request.search,

            sort_by: request.sort_by.unwrap_or_else(|| "created_at".to_string()),

            sort_type: request.sort_type.unwrap_or_else(|| "DESC".to_string()),
        };

        let roles = self.repository.list(&filter).await?;

        let total = self.repository.count(&filter).await?;

        let items = roles.into_iter().map(|r| self.map_response(r)).collect();

        Ok(RoleListResponse {
            items,
            page: filter.page,
            page_size: filter.page_size,
            total,
        })
    }
}
