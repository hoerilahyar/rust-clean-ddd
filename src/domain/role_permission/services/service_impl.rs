use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    domain::{
        audit_log::services::{AuditLogService, RecordAuditLogInput},
        role_permission::{
            dto::{AssignRolePermissionRequest, RolePermissionResponse},
            repository::RolePermissionRepository,
            services::RolePermissionService,
        },
    },
    infrastructure::cache::CacheHelper,
};

const ROLE_PERMISSION_LIST_TTL: Duration = Duration::from_secs(120);

pub struct DefaultRolePermissionService {
    repository: Arc<dyn RolePermissionRepository>,
    cache: CacheHelper,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultRolePermissionService {
    pub fn new(
        repository: Arc<dyn RolePermissionRepository>,
        cache: CacheHelper,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            cache,
            audit_log_service,
        }
    }

    fn list_cache_key(role_id: u64) -> String {
        format!("role_permission:list:{role_id}")
    }
}

#[async_trait]
impl RolePermissionService for DefaultRolePermissionService {
    async fn assign(
        &self,
        role_id: u64,
        request: AssignRolePermissionRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self
            .repository
            .assign(role_id, &request.permission_ids)
            .await;

        if result.is_ok() {
            self.cache.invalidate(&Self::list_cache_key(role_id)).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "role_permission.assigned".to_string(),
                entity_type: Some("role".into()),
                entity_id: Some(role_id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "permission_ids": request.permission_ids,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn revoke(
        &self,
        role_id: u64,
        permission_id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.repository.revoke(role_id, permission_id).await;

        if result.is_ok() {
            self.cache.invalidate(&Self::list_cache_key(role_id)).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "role_permission.revoked".to_string(),
                entity_type: Some("role".into()),
                entity_id: Some(format!("role:{} permission:{}", role_id, permission_id)),
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

    async fn list(&self, role_id: u64) -> Result<Vec<RolePermissionResponse>> {
        let cache_key = Self::list_cache_key(role_id);

        if let Some(cached) = self
            .cache
            .get_json::<Vec<RolePermissionResponse>>(&cache_key)
            .await
        {
            return Ok(cached);
        }

        let items = self.repository.find_permissions(role_id).await?;
        let response: Vec<RolePermissionResponse> = items
            .into_iter()
            .map(|item| RolePermissionResponse {
                role_id: item.role_id,
                permission_id: item.permission_id,
            })
            .collect();

        self.cache
            .set_json(&cache_key, &response, Some(ROLE_PERMISSION_LIST_TTL))
            .await;

        Ok(response)
    }
}
