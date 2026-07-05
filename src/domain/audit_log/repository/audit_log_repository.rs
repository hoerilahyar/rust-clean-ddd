use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::domain::audit_log::entity::AuditLog;

#[derive(Debug, Clone, Default)]
pub struct AuditLogFilter {
    pub actor_id: Option<u64>,
    pub action: Option<String>,
    pub entity_type: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub page: u32,
    pub page_size: u32,
}

impl AuditLogFilter {
    pub fn offset(&self) -> u32 {
        (self.page.max(1) - 1) * self.page_size.max(1)
    }
}

#[derive(Debug, Clone)]
pub struct NewAuditLog {
    pub actor_id: Option<u64>,
    pub actor_email: Option<String>,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub status: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<Value>,
}

#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn insert(&self, log: &NewAuditLog) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: u64) -> anyhow::Result<Option<AuditLog>>;
    async fn find_all(&self, filter: &AuditLogFilter) -> anyhow::Result<(Vec<AuditLog>, i64)>;
}
