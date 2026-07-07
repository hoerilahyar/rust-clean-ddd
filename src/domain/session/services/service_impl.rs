use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::domain::{
    audit_log::{
        entity::audit_action,
        services::{AuditLogService, RecordAuditLogInput},
    },
    auth::{entity::RefreshToken, repository::auth_repository::AuthRepository},
    session::{dto::SessionResponse, services::SessionService},
};

pub struct DefaultSessionService {
    repository: Arc<dyn AuthRepository>,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultSessionService {
    pub fn new(
        repository: Arc<dyn AuthRepository>,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            audit_log_service,
        }
    }

    fn map_response(
        &self,
        session: RefreshToken,
        current_device_id: Option<&str>,
    ) -> SessionResponse {
        let is_current = current_device_id
            .map(|id| id == session.device_id)
            .unwrap_or(false);

        SessionResponse {
            id: session.id,
            device_id: session.device_id,
            ip_address: session.ip_address,
            created_at: session.created_at,
            expired_at: session.expired_at,
            is_current,
        }
    }
}

#[async_trait]
impl SessionService for DefaultSessionService {
    async fn list(
        &self,
        user_id: u64,
        current_device_id: Option<String>,
    ) -> Result<Vec<SessionResponse>> {
        let sessions = self.repository.find_active_sessions(user_id).await?;

        Ok(sessions
            .into_iter()
            .map(|s| self.map_response(s, current_device_id.as_deref()))
            .collect())
    }

    async fn revoke(
        &self,
        user_id: u64,
        session_id: u64,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        // Don't reveal whether a session id exists at all if it doesn't belong to this user.
        let owned = self
            .repository
            .find_refresh_token_by_id(session_id)
            .await?
            .filter(|s| s.user_id == user_id);

        let result = match owned {
            Some(_) => self.repository.revoke_refresh_token(session_id).await,
            None => Err(anyhow!("Session not found")),
        };

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id: Some(user_id),
                actor_email: None,
                action: audit_action::SESSION_REVOKED.to_string(),
                entity_type: Some("session".into()),
                entity_id: Some(session_id.to_string()),
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

    async fn revoke_others(
        &self,
        user_id: u64,
        current_device_id: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self
            .repository
            .revoke_all_except(user_id, &current_device_id)
            .await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id: Some(user_id),
                actor_email: None,
                action: audit_action::SESSION_REVOKED_OTHERS.to_string(),
                entity_type: Some("session".into()),
                entity_id: None,
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "kept_device_id": current_device_id,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }
}
