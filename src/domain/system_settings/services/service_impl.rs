use std::{sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        audit_log::{
            entity::audit_action,
            services::{AuditLogService, RecordAuditLogInput},
        },
        system_settings::{
            dto::{
                ListSystemSettingRequest, SystemSettingListResponse, SystemSettingResponse,
                UpsertSystemSettingRequest,
            },
            entity::SystemSetting,
            repository::SystemSettingRepository,
            services::SystemSettingService,
        },
    },
    infrastructure::cache::CacheHelper,
};

const SYSTEM_SETTING_LIST_TTL: Duration = Duration::from_secs(300);
const SYSTEM_SETTING_KEY_TTL: Duration = Duration::from_secs(300);
const SYSTEM_SETTING_LIST_KEY: &str = "system_setting:list";

pub struct DefaultSystemSettingService {
    repository: Arc<dyn SystemSettingRepository>,
    cache: CacheHelper,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultSystemSettingService {
    pub fn new(
        repository: Arc<dyn SystemSettingRepository>,
        cache: CacheHelper,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            cache,
            audit_log_service,
        }
    }

    fn map_response(&self, setting: SystemSetting) -> SystemSettingResponse {
        SystemSettingResponse {
            id: setting.id,
            setting_key: setting.setting_key,
            setting_value: setting.setting_value,
            data_type: format!("{:?}", setting.data_type).to_lowercase(),
            description: setting.description,
            is_public: setting.is_public,
            is_active: setting.is_active,
            created_at: setting.created_at.and_utc(),
            updated_at: setting.updated_at.map(|dt| dt.and_utc()),
        }
    }

    fn key_cache_key(key: &str) -> String {
        format!("system_setting:key:{key}")
    }

    async fn invalidate_all(&self, key: &str) {
        self.cache.invalidate(SYSTEM_SETTING_LIST_KEY).await;
        self.cache.invalidate(&Self::key_cache_key(key)).await;
    }

    async fn upsert_inner(&self, request: &UpsertSystemSettingRequest) -> Result<SystemSetting> {
        let existing = self.repository.find_by_key(&request.setting_key).await?;

        let setting = self
            .repository
            .upsert(
                &request.setting_key,
                request.setting_value.clone(),
                &request.data_type,
                request.description.clone(),
                request.is_public.unwrap_or(true),
            )
            .await?;

        let _ = existing;

        Ok(setting)
    }
}

#[async_trait]
impl SystemSettingService for DefaultSystemSettingService {
    async fn upsert(
        &self,
        request: UpsertSystemSettingRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<SystemSettingResponse> {
        let before = self
            .repository
            .find_by_key(&request.setting_key)
            .await
            .ok()
            .flatten();

        let result = self.upsert_inner(&request).await;

        if result.is_ok() {
            self.invalidate_all(&request.setting_key).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::SETTING_UPSERTED.to_string(),
                entity_type: Some("system_setting".into()),
                entity_id: Some(request.setting_key.clone()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "before": before.map(|s| serde_json::json!({
                        "setting_value": s.setting_value,
                        "is_public": s.is_public,
                        "is_active": s.is_active,
                    })),
                    "after": {
                        "setting_value": request.setting_value,
                        "data_type": request.data_type,
                        "is_public": request.is_public,
                    },
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result.map(|s| self.map_response(s))
    }

    async fn set_active(
        &self,
        key: &str,
        is_active: bool,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.repository.set_active(key, is_active).await;

        if result.is_ok() {
            self.invalidate_all(key).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: if is_active {
                    audit_action::SETTING_ACTIVATED.to_string()
                } else {
                    audit_action::SETTING_DEACTIVATED.to_string()
                },
                entity_type: Some("system_setting".into()),
                entity_id: Some(key.to_string()),
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
        key: &str,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let marker = Uuid::new_v4().to_string();
        let result = self.repository.delete_by_key(key, &marker).await;

        if result.is_ok() {
            self.invalidate_all(key).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::SETTING_DELETED.to_string(),
                entity_type: Some("system_setting".into()),
                entity_id: Some(key.to_string()),
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

    async fn find_by_key(&self, key: &str) -> Result<SystemSettingResponse> {
        let cache_key = Self::key_cache_key(key);

        if let Some(cached) = self
            .cache
            .get_json::<SystemSettingResponse>(&cache_key)
            .await
        {
            return Ok(cached);
        }

        let setting = self
            .repository
            .find_by_key(key)
            .await?
            .ok_or_else(|| anyhow!("Setting not found"))?;

        let response = self.map_response(setting);
        self.cache
            .set_json(&cache_key, &response, Some(SYSTEM_SETTING_KEY_TTL))
            .await;

        Ok(response)
    }

    async fn list(&self, request: ListSystemSettingRequest) -> Result<SystemSettingListResponse> {
        let settings = match self
            .cache
            .get_json::<Vec<SystemSettingResponse>>(SYSTEM_SETTING_LIST_KEY)
            .await
        {
            Some(cached) => cached,
            None => {
                let settings = self.repository.find_all().await?;
                let mapped: Vec<SystemSettingResponse> =
                    settings.into_iter().map(|s| self.map_response(s)).collect();
                self.cache
                    .set_json(
                        SYSTEM_SETTING_LIST_KEY,
                        &mapped,
                        Some(SYSTEM_SETTING_LIST_TTL),
                    )
                    .await;
                mapped
            }
        };

        let items = settings
            .into_iter()
            .filter(|s| request.is_active.map_or(true, |v| s.is_active == v))
            .filter(|s| request.is_public.map_or(true, |v| s.is_public == v))
            .collect();

        Ok(SystemSettingListResponse { items })
    }
}
