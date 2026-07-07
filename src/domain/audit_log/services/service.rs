use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::domain::audit_log::dto::{AuditLogListResponse, AuditLogQueryRequest, AuditLogResponse};

#[derive(Debug, Clone)]
pub struct RecordAuditLogInput {
    pub actor_id: Option<u64>,
    pub actor_email: Option<String>,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub is_success: bool,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<Value>,
}

#[async_trait]
pub trait AuditLogService: Send + Sync {
    async fn record(&self, input: RecordAuditLogInput);
    async fn get_by_id(&self, id: u64) -> Result<AuditLogResponse>;
    async fn list(&self, query: AuditLogQueryRequest) -> Result<AuditLogListResponse>;
}
