use crate::domain::audit_log::dto::{AuditLogListResponse, AuditLogResponse};
use crate::domain::audit_log::entity::AuditLog;

pub fn to_response(log: AuditLog) -> AuditLogResponse {
    AuditLogResponse {
        id: log.id,
        actor_id: log.actor_id,
        actor_email: log.actor_email,
        action: log.action,
        entity_type: log.entity_type,
        entity_id: log.entity_id,
        status: log.status,
        ip_address: log.ip_address,
        user_agent: log.user_agent,
        metadata: log.metadata,
        created_at: log.created_at,
    }
}

pub fn to_list_response(
    logs: Vec<AuditLog>,
    total: i64,
    page: u32,
    page_size: u32,
) -> AuditLogListResponse {
    AuditLogListResponse {
        data: logs.into_iter().map(to_response).collect(),
        total,
        page,
        page_size,
    }
}
