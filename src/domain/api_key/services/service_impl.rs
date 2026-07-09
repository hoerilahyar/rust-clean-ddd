use std::{sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        api_key::{
            dto::{
                ApiKeyListResponse, ApiKeyResponse, CreateApiKeyRequest, CreateApiKeyResponse,
                ListApiKeyRequest, UpdateApiKeyRequest,
            },
            entity::ApiKey,
            repository::ApiKeyRepository,
            services::ApiKeyService,
        },
        audit_log::{
            entity::audit_action,
            services::{AuditLogService, RecordAuditLogInput},
        },
        permission::entity::PermissionCode,
    },
    infrastructure::{cache::CacheHelper, security::ApiKeySecret},
};

const API_KEY_LIST_TTL: Duration = Duration::from_secs(60);
const API_KEY_LIST_KEY: &str = "api_key:list";

pub struct DefaultApiKeyService {
    repository: Arc<dyn ApiKeyRepository>,
    cache: CacheHelper,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultApiKeyService {
    pub fn new(
        repository: Arc<dyn ApiKeyRepository>,
        cache: CacheHelper,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            cache,
            audit_log_service,
        }
    }

    fn map_response(&self, key: ApiKey) -> ApiKeyResponse {
        ApiKeyResponse {
            id: key.id,
            name: key.name,
            key_prefix: key.key_prefix,
            permissions: key.permissions,
            is_active: key.is_active,
            expires_at: key.expires_at.map(|dt| dt.and_utc()),
            last_used_at: key.last_used_at.map(|dt| dt.and_utc()),
            created_by: key.created_by,
            created_at: key.created_at.and_utc(),
            updated_at: key.updated_at.map(|dt| dt.and_utc()),
        }
    }

    /// Every requested permission must (a) be a permission code that
    /// actually exists and (b) already be held by the actor granting it.
    fn assert_permissions_allowed(
        &self,
        requested: &[String],
        actor_permissions: &[String],
    ) -> Result<()> {
        const KNOWN: &[PermissionCode] = &[
            PermissionCode::AuthorizeMe,
            PermissionCode::UserCreate,
            PermissionCode::UserRead,
            PermissionCode::UserUpdate,
            PermissionCode::UserDelete,
            PermissionCode::RoleCreate,
            PermissionCode::RoleRead,
            PermissionCode::RoleUpdate,
            PermissionCode::RoleDelete,
            PermissionCode::PermissionCreate,
            PermissionCode::PermissionRead,
            PermissionCode::PermissionUpdate,
            PermissionCode::PermissionDelete,
            PermissionCode::RolePermissionAssign,
            PermissionCode::RolePermissionRevoke,
            PermissionCode::RolePermissionRead,
            PermissionCode::UserRoleAssign,
            PermissionCode::UserRoleRevoke,
            PermissionCode::UserRoleRead,
            PermissionCode::AuditLogRead,
            PermissionCode::MenuCreate,
            PermissionCode::MenuRead,
            PermissionCode::MenuUpdate,
            PermissionCode::MenuDelete,
            PermissionCode::MenuPermissionAssign,
            PermissionCode::MenuPermissionRead,
            PermissionCode::MenuPermissionRevoke,
            PermissionCode::SystemSettingUpdate,
            PermissionCode::SystemSettingDelete,
            PermissionCode::SystemSettingRead,
            PermissionCode::MasterDataGroupCreate,
            PermissionCode::MasterDataGroupRead,
            PermissionCode::MasterDataGroupUpdate,
            PermissionCode::MasterDataGroupDelete,
            PermissionCode::MasterDataItemCreate,
            PermissionCode::MasterDataItemRead,
            PermissionCode::MasterDataItemUpdate,
            PermissionCode::MasterDataItemDelete,
            PermissionCode::ApiKeyCreate,
            PermissionCode::ApiKeyRead,
            PermissionCode::ApiKeyUpdate,
            PermissionCode::ApiKeyRevoke,
            PermissionCode::ApiKeyDelete,
        ];

        for code in requested {
            if !KNOWN.iter().any(|k| k.as_str() == code.as_str()) {
                return Err(anyhow!("Unknown permission code: {code}"));
            }

            if !actor_permissions.iter().any(|p| p.as_str() == code.as_str()) {
                return Err(anyhow!(
                    "Cannot grant permission '{code}': you do not hold it yourself"
                ));
            }
        }

        Ok(())
    }

    async fn invalidate_list(&self) {
        self.cache.invalidate(API_KEY_LIST_KEY).await;
    }
}

#[async_trait]
impl ApiKeyService for DefaultApiKeyService {
    async fn create(
        &self,
        request: CreateApiKeyRequest,
        actor_id: Option<u64>,
        actor_permissions: &[String],
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<CreateApiKeyResponse> {
        let validation = self.assert_permissions_allowed(&request.permissions, actor_permissions);

        let result = if let Err(err) = validation {
            Err(err)
        } else {
            let generated = ApiKeySecret::generate();
            let key_hash = ApiKeySecret::hash(&generated.secret);

            self.repository
                .create(
                    &request.name,
                    &generated.prefix,
                    &key_hash,
                    &request.permissions,
                    request.expires_at.map(|dt| dt.naive_utc()),
                    actor_id,
                )
                .await
                .map(|key| (key, generated.full_key))
        };

        if result.is_ok() {
            self.invalidate_list().await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::API_KEY_CREATED.to_string(),
                entity_type: Some("api_key".into()),
                entity_id: result.as_ref().ok().map(|(key, _)| key.id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "name": request.name,
                    "permissions": request.permissions,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result.map(|(key, full_key)| {
            let mapped = self.map_response(key);
            CreateApiKeyResponse {
                id: mapped.id,
                name: mapped.name,
                key_prefix: mapped.key_prefix,
                permissions: mapped.permissions,
                is_active: mapped.is_active,
                expires_at: mapped.expires_at,
                created_by: mapped.created_by,
                created_at: mapped.created_at,
                api_key: full_key,
            }
        })
    }

    async fn list(&self, request: ListApiKeyRequest) -> Result<ApiKeyListResponse> {
        let keys = match self
            .cache
            .get_json::<Vec<ApiKeyResponse>>(API_KEY_LIST_KEY)
            .await
        {
            Some(cached) => cached,
            None => {
                let keys = self.repository.find_all().await?;
                let mapped: Vec<ApiKeyResponse> =
                    keys.into_iter().map(|k| self.map_response(k)).collect();
                self.cache
                    .set_json(API_KEY_LIST_KEY, &mapped, Some(API_KEY_LIST_TTL))
                    .await;
                mapped
            }
        };

        let search = request.search.map(|s| s.to_lowercase());

        let items = keys
            .into_iter()
            .filter(|k| request.is_active.map_or(true, |v| k.is_active == v))
            .filter(|k| {
                search
                    .as_ref()
                    .map_or(true, |s| k.name.to_lowercase().contains(s))
            })
            .collect();

        Ok(ApiKeyListResponse { items })
    }

    async fn find_by_id(&self, id: u64) -> Result<ApiKeyResponse> {
        let key = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("API key not found"))?;

        Ok(self.map_response(key))
    }

    async fn update(
        &self,
        id: u64,
        request: UpdateApiKeyRequest,
        actor_id: Option<u64>,
        actor_permissions: &[String],
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<ApiKeyResponse> {
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("API key not found"))?;

        let name = request.name.clone().unwrap_or_else(|| existing.name.clone());
        let permissions = request
            .permissions
            .clone()
            .unwrap_or_else(|| existing.permissions.clone());
        let expires_at = request
            .expires_at
            .map(|dt| dt.naive_utc())
            .or(existing.expires_at);

        let validation = self.assert_permissions_allowed(&permissions, actor_permissions);

        let result = if let Err(err) = validation {
            Err(err)
        } else {
            self.repository
                .update(id, &name, &permissions, expires_at)
                .await
        };

        if result.is_ok() {
            self.invalidate_list().await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::API_KEY_UPDATED.to_string(),
                entity_type: Some("api_key".into()),
                entity_id: Some(id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "before": {
                        "name": existing.name,
                        "permissions": existing.permissions,
                    },
                    "after": { "name": name, "permissions": permissions },
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result?;

        self.find_by_id(id).await
    }

    async fn set_active(
        &self,
        id: u64,
        is_active: bool,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.repository.set_active(id, is_active).await;

        if result.is_ok() {
            self.invalidate_list().await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: if is_active {
                    audit_action::API_KEY_ACTIVATED.to_string()
                } else {
                    audit_action::API_KEY_DEACTIVATED.to_string()
                },
                entity_type: Some("api_key".into()),
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

        result.map_err(Into::into)
    }

    async fn delete(
        &self,
        id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let marker = Uuid::new_v4().to_string();
        let result = self.repository.delete_by_id(id, &marker).await;

        if result.is_ok() {
            self.invalidate_list().await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::API_KEY_DELETED.to_string(),
                entity_type: Some("api_key".into()),
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

        result.map_err(Into::into)
    }

    async fn verify(&self, full_key: &str) -> Result<Option<ApiKey>> {
        let Some((prefix, secret)) = ApiKeySecret::parse(full_key) else {
            return Ok(None);
        };

        let Some(key) = self.repository.find_by_prefix(&prefix).await? else {
            return Ok(None);
        };

        if !key.is_usable() {
            return Ok(None);
        }

        if !ApiKeySecret::verify(&secret, &key.key_hash) {
            return Ok(None);
        }

        Ok(Some(key))
    }

    async fn record_usage(&self, id: u64) {
        if let Err(err) = self.repository.touch_last_used(id).await {
            tracing::warn!(id, error = %err, "failed to record api key usage");
        }
    }
}
