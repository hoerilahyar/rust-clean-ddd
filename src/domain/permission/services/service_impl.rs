use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;

use crate::domain::{
    audit_log::{
        entity::audit_action,
        services::{AuditLogService, RecordAuditLogInput},
    },
    permission::{
        dto::{
            CreatePermissionRequest, GetPermissionRequest, ListPermissionRequest,
            PermissionListResponse, PermissionResponse, UpdatePermissionRequest,
        },
        entity::{Permission, PermissionFilter},
        repository::PermissionRepository,
        services::PermissionService,
    },
};

pub struct DefaultPermissionService {
    repository: Arc<dyn PermissionRepository>,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultPermissionService {
    pub fn new(
        repository: Arc<dyn PermissionRepository>,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            audit_log_service,
        }
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

    async fn create_inner(&self, request: &CreatePermissionRequest) -> Result<u64> {
        if self.repository.exists_code(&request.code).await? {
            return Err(anyhow!("Permission code already exists"));
        }

        let now = Utc::now();

        let permission = Permission {
            id: 0,
            code: request.code.clone(),
            name: request.name.clone(),
            resource: request.resource.clone(),
            action: request.action.clone(),
            description: request.description.clone(),
            is_active: request.is_active.unwrap_or(true),
            created_at: now,
            updated_at: now,
        };

        self.repository.create(&permission).await
    }

    async fn update_inner(&self, id: u64, request: &UpdatePermissionRequest) -> Result<()> {
        let mut permission = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("Permission not found"))?;

        permission.code = request.code.clone().unwrap_or(permission.code);
        permission.name = request.name.clone();
        permission.description = request.description.clone();
        permission.is_active = request.is_active.clone().unwrap_or(permission.is_active);
        permission.updated_at = Utc::now();

        self.repository.update(&permission).await
    }
}

#[async_trait]
impl PermissionService for DefaultPermissionService {
    async fn create(
        &self,
        request: CreatePermissionRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64> {
        let result = self.create_inner(&request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "permission.created".to_string(),
                entity_type: Some("permission".into()),
                entity_id: result.as_ref().ok().map(|id| id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "code": request.code,
                    "resource": request.resource,
                    "action": request.action,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn update(
        &self,
        id: u64,
        request: UpdatePermissionRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.update_inner(id, &request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "permission.updated".to_string(),
                entity_type: Some("permission".into()),
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
                action: audit_action::RBAC_PERMISSION_DELETED.to_string(),
                entity_type: Some("permission".into()),
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
