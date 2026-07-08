use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    domain::{
        audit_log::services::{AuditLogService, RecordAuditLogInput},
        user_role::{
            dto::{AssignUserRoleRequest, UserRoleResponse},
            entity::Role,
            repository::UserRoleRepository,
            services::UserRoleService,
        },
    },
    infrastructure::cache::CacheHelper,
};

const USER_ROLE_LIST_TTL: Duration = Duration::from_secs(120);

pub struct DefaultUserRoleService {
    repository: Arc<dyn UserRoleRepository>,
    cache: CacheHelper,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultUserRoleService {
    pub fn new(
        repository: Arc<dyn UserRoleRepository>,
        cache: CacheHelper,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            cache,
            audit_log_service,
        }
    }

    fn list_cache_key(user_id: u64) -> String {
        format!("user_role:list:{user_id}")
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
    async fn assign(
        &self,
        user_id: u64,
        request: AssignUserRoleRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.repository.assign(user_id, &request.role_ids).await;

        if result.is_ok() {
            self.cache.invalidate(&Self::list_cache_key(user_id)).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "user_role.assigned".to_string(),
                entity_type: Some("user".into()),
                entity_id: Some(user_id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "role_ids": request.role_ids,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn revoke(
        &self,
        user_id: u64,
        role_id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.repository.revoke(user_id, role_id).await;

        if result.is_ok() {
            self.cache.invalidate(&Self::list_cache_key(user_id)).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: "user_role.revoked".to_string(),
                entity_type: Some("user".into()),
                entity_id: Some(format!("user:{} role:{}", user_id, role_id)),
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

    async fn list(&self, user_id: u64) -> Result<Vec<UserRoleResponse>> {
        let cache_key = Self::list_cache_key(user_id);

        if let Some(cached) = self
            .cache
            .get_json::<Vec<UserRoleResponse>>(&cache_key)
            .await
        {
            return Ok(cached);
        }

        let roles = self.repository.find_roles(user_id).await?;
        let response: Vec<UserRoleResponse> = roles.into_iter().map(Self::map_response).collect();

        self.cache
            .set_json(&cache_key, &response, Some(USER_ROLE_LIST_TTL))
            .await;

        Ok(response)
    }
}
