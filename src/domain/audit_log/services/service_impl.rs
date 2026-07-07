use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::domain::audit_log::dto::{AuditLogListResponse, AuditLogQueryRequest, AuditLogResponse};
use crate::domain::audit_log::entity::audit_status;
use crate::domain::audit_log::errors::AuditLogError;
use crate::domain::audit_log::mapper;
use crate::domain::audit_log::repository::{AuditLogFilter, AuditLogRepository, NewAuditLog};
use crate::domain::audit_log::services::{AuditLogService, RecordAuditLogInput};

pub struct DefaultAuditLogService {
    repository: Arc<dyn AuditLogRepository>,
}

impl DefaultAuditLogService {
    pub fn new(repository: Arc<dyn AuditLogRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl AuditLogService for DefaultAuditLogService {
    async fn record(&self, input: RecordAuditLogInput) {
        let status = if input.is_success {
            audit_status::SUCCESS
        } else {
            audit_status::FAILED
        }
        .to_string();

        let new_log = NewAuditLog {
            actor_id: input.actor_id,
            actor_email: input.actor_email,
            action: input.action,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            status,
            ip_address: input.ip_address,
            user_agent: input.user_agent,
            metadata: input.metadata,
        };

        if let Err(err) = self.repository.insert(&new_log).await {
            tracing::error!(error = ?err, "failed to write audit log");
        }
    }

    async fn get_by_id(&self, id: u64) -> Result<AuditLogResponse> {
        let log = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!(AuditLogError::NotFound))?;

        Ok(mapper::to_response(log))
    }

    async fn list(&self, query: AuditLogQueryRequest) -> Result<AuditLogListResponse> {
        let filter = AuditLogFilter {
            actor_id: query.actor_id,
            action: query.action,
            entity_type: query.entity_type,
            status: query.status,
            date_from: query.date_from,
            date_to: query.date_to,
            page: query.page,
            page_size: query.page_size,
        };

        let (logs, total) = self.repository.find_all(&filter).await?;

        Ok(mapper::to_list_response(
            logs,
            total,
            filter.page,
            filter.page_size,
        ))
    }
}
