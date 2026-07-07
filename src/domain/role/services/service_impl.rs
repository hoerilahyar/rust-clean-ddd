use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;

use crate::domain::{
    audit_log::{
        entity::audit_action,
        services::{AuditLogService, RecordAuditLogInput},
    },
    role::{
        dto::{
            CreateRoleRequest, GetRoleRequest, ListRoleRequest, RoleListResponse, RoleResponse,
            UpdateRoleRequest,
        },
        entity::{Role, RoleFilter},
        repository::RoleRepository,
        services::RoleService,
    },
};

pub struct DefaultRoleService {
    repository: Arc<dyn RoleRepository>,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultRoleService {
    pub fn new(
        repository: Arc<dyn RoleRepository>,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            audit_log_service,
        }
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

    async fn create_inner(&self, request: &CreateRoleRequest) -> Result<u64> {
        if self.repository.exists_code(&request.code).await? {
            return Err(anyhow!("Role code already exists"));
        }

        let now = Utc::now();

        let role = Role {
            id: 0,
            code: request.code.clone(),
            name: request.name.clone(),
            description: request.description.clone(),
            is_active: request.is_active.unwrap_or(true),
            created_at: now,
            updated_at: now,
        };

        self.repository.create(&role).await
    }

    async fn update_inner(&self, id: u64, request: &UpdateRoleRequest) -> Result<()> {
        let mut role = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("Role not found"))?;

        role.code = request.code.clone().unwrap_or(role.code);
        role.name = request.name.clone();
        role.description = request.description.clone();
        role.is_active = request.is_active.unwrap_or(role.is_active);

        self.repository.update(&role).await
    }
}

#[async_trait]
impl RoleService for DefaultRoleService {
    async fn create(
        &self,
        request: CreateRoleRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64> {
        let result = self.create_inner(&request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "role.created".to_string(),
                entity_type: Some("role".into()),
                entity_id: result.as_ref().ok().map(|id| id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "code": request.code,
                    "name": request.name,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn update(
        &self,
        id: u64,
        request: UpdateRoleRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.update_inner(id, &request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "role.updated".to_string(),
                entity_type: Some("role".into()),
                entity_id: Some(id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "name": request.name,
                    "is_active": request.is_active,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn delete(
        &self,
        id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.repository.delete(id).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::RBAC_ROLE_DELETED.to_string(),
                entity_type: Some("role".into()),
                entity_id: Some(id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: result
                    .as_ref()
                    .err()
                    .map(|e| serde_json::json!({ "error": e.to_string() })),
            })
            .await;

        result
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
