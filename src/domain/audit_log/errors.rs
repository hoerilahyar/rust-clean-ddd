use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuditLogError {
    #[error("audit log not found")]
    NotFound,
}
