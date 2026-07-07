use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    audit_log::{
        entity::audit_action,
        services::{AuditLogService, RecordAuditLogInput},
    },
    user_setting::{
        dto::{
            ListUserSettingRequest, UpsertUserSettingRequest, UserSettingListResponse,
            UserSettingResponse,
        },
        entity::UserSetting,
        repository::UserSettingRepository,
        services::UserSettingService,
    },
};

pub struct DefaultUserSettingService {
    repository: Arc<dyn UserSettingRepository>,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultUserSettingService {
    pub fn new(
        repository: Arc<dyn UserSettingRepository>,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            audit_log_service,
        }
    }

    fn map_response(&self, setting: UserSetting) -> UserSettingResponse {
        UserSettingResponse {
            id: setting.id,
            setting_key: setting.setting_key,
            setting_value: setting.setting_value,
            data_type: format!("{:?}", setting.data_type).to_lowercase(),
            description: setting.description,
            is_active: setting.is_active,
            created_at: setting.created_at.and_utc(),
            updated_at: setting.updated_at.map(|dt| dt.and_utc()),
        }
    }

    async fn upsert_inner(
        &self,
        user_id: u64,
        request: &UpsertUserSettingRequest,
    ) -> Result<UserSetting> {
        self.repository
            .upsert(
                user_id,
                &request.setting_key,
                request.setting_value.clone(),
                &request.data_type,
                request.description.clone(),
            )
            .await
    }
}

#[async_trait]
impl UserSettingService for DefaultUserSettingService {
    async fn upsert(
        &self,
        user_id: u64,
        request: UpsertUserSettingRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<UserSettingResponse> {
        let before = self
            .repository
            .find_by_key(user_id, &request.setting_key)
            .await
            .ok()
            .flatten();

        let result = self.upsert_inner(user_id, &request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id: Some(user_id),
                actor_email: None,
                action: audit_action::USER_SETTING_UPSERTED.to_string(),
                entity_type: Some("user_setting".into()),
                entity_id: Some(request.setting_key.clone()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "before": before.map(|s| serde_json::json!({
                        "setting_value": s.setting_value,
                        "is_active": s.is_active,
                    })),
                    "after": {
                        "setting_value": request.setting_value,
                        "data_type": request.data_type,
                    },
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result.map(|s| self.map_response(s))
    }

    async fn set_active(
        &self,
        user_id: u64,
        key: &str,
        is_active: bool,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.repository.set_active(user_id, key, is_active).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id: Some(user_id),
                actor_email: None,
                action: if is_active {
                    audit_action::USER_SETTING_ACTIVATED.to_string()
                } else {
                    audit_action::USER_SETTING_DEACTIVATED.to_string()
                },
                entity_type: Some("user_setting".into()),
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
        user_id: u64,
        key: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let marker = Uuid::new_v4().to_string();
        let result = self.repository.delete_by_key(user_id, key, &marker).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id: Some(user_id),
                actor_email: None,
                action: audit_action::USER_SETTING_DELETED.to_string(),
                entity_type: Some("user_setting".into()),
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

    async fn find_by_key(&self, user_id: u64, key: &str) -> Result<UserSettingResponse> {
        let setting = self
            .repository
            .find_by_key(user_id, key)
            .await?
            .ok_or_else(|| anyhow!("Setting not found"))?;

        Ok(self.map_response(setting))
    }

    async fn list(
        &self,
        user_id: u64,
        request: ListUserSettingRequest,
    ) -> Result<UserSettingListResponse> {
        let settings = self.repository.find_all(user_id).await?;

        let items = settings
            .into_iter()
            .filter(|s| request.is_active.map_or(true, |v| s.is_active == v))
            .map(|s| self.map_response(s))
            .collect();

        Ok(UserSettingListResponse { items })
    }
}
