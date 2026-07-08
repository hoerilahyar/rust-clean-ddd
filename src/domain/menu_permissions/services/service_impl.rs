use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    domain::{
        audit_log::services::{AuditLogService, RecordAuditLogInput},
        menu_permissions::{
            dto::{AssignMenuPermissionRequest, MenuPermissionResponse},
            repository::MenuPermissionRepository,
            services::MenuPermissionService,
        },
    },
    infrastructure::cache::CacheHelper,
};

const MENU_PERMISSION_LIST_TTL: Duration = Duration::from_secs(120);

pub struct DefaultMenuPermissionService {
    repository: Arc<dyn MenuPermissionRepository>,
    cache: CacheHelper,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultMenuPermissionService {
    pub fn new(
        repository: Arc<dyn MenuPermissionRepository>,
        cache: CacheHelper,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            cache,
            audit_log_service,
        }
    }

    fn list_cache_key(menu_id: u64) -> String {
        format!("menu_permission:list:{menu_id}")
    }
}

#[async_trait]
impl MenuPermissionService for DefaultMenuPermissionService {
    async fn assign(
        &self,
        menu_id: u64,
        request: AssignMenuPermissionRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self
            .repository
            .assign(menu_id, &request.permission_ids)
            .await;

        if result.is_ok() {
            self.cache.invalidate(&Self::list_cache_key(menu_id)).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "menu_permission.assigned".to_string(),
                entity_type: Some("role".into()),
                entity_id: Some(menu_id.to_string()),
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
        menu_id: u64,
        permission_id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.repository.revoke(menu_id, permission_id).await;

        if result.is_ok() {
            self.cache.invalidate(&Self::list_cache_key(menu_id)).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "menu_permission.revoked".to_string(),
                entity_type: Some("role".into()),
                entity_id: Some(format!("role:{} permission:{}", menu_id, permission_id)),
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

    async fn list(&self, menu_id: u64) -> Result<Vec<MenuPermissionResponse>> {
        let cache_key = Self::list_cache_key(menu_id);

        if let Some(cached) = self
            .cache
            .get_json::<Vec<MenuPermissionResponse>>(&cache_key)
            .await
        {
            return Ok(cached);
        }

        let items = self.repository.find_permissions(menu_id).await?;
        let response: Vec<MenuPermissionResponse> = items
            .into_iter()
            .map(|item| MenuPermissionResponse {
                menu_id: item.menu_id,
                permission_id: item.permission_id,
            })
            .collect();

        self.cache
            .set_json(&cache_key, &response, Some(MENU_PERMISSION_LIST_TTL))
            .await;

        Ok(response)
    }
}
