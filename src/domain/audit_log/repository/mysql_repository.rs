use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::domain::audit_log::entity::AuditLog;
use crate::domain::audit_log::repository::audit_log_repository::{
    AuditLogFilter, AuditLogRepository, NewAuditLog,
};

pub struct MySqlAuditLogRepository {
    pool: MySqlPool,
}

impl MySqlAuditLogRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogRepository for MySqlAuditLogRepository {
    async fn insert(&self, log: &NewAuditLog) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (
                actor_id, actor_email, action, entity_type, entity_id,
                status, ip_address, user_agent, metadata, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, UTC_TIMESTAMP())
            "#,
        )
        .bind(log.actor_id)
        .bind(&log.actor_email)
        .bind(&log.action)
        .bind(&log.entity_type)
        .bind(&log.entity_id)
        .bind(&log.status)
        .bind(&log.ip_address)
        .bind(&log.user_agent)
        .bind(&log.metadata)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: u64) -> anyhow::Result<Option<AuditLog>> {
        let log = sqlx::query_as::<_, AuditLog>(
            r#"
            SELECT
                id, actor_id, actor_email, action, entity_type, entity_id,
                status, ip_address, user_agent, metadata, created_at
            FROM audit_logs
            WHERE id = ?
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(log)
    }

    async fn find_all(&self, filter: &AuditLogFilter) -> anyhow::Result<(Vec<AuditLog>, i64)> {
        let mut conditions: Vec<String> = vec![];

        if filter.actor_id.is_some() {
            conditions.push("actor_id = ?".into());
        }
        if filter.action.is_some() {
            conditions.push("action = ?".into());
        }
        if filter.entity_type.is_some() {
            conditions.push("entity_type = ?".into());
        }
        if filter.status.is_some() {
            conditions.push("status = ?".into());
        }
        if filter.date_from.is_some() {
            conditions.push("created_at >= ?".into());
        }
        if filter.date_to.is_some() {
            conditions.push("created_at <= ?".into());
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            r#"
            SELECT
                id, actor_id, actor_email, action, entity_type, entity_id,
                status, ip_address, user_agent, metadata, created_at
            FROM audit_logs
            {}
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            where_clause
        );

        let count_sql = format!("SELECT COUNT(*) FROM audit_logs {}", where_clause);

        macro_rules! bind_filters {
            ($q:expr) => {{
                let mut q = $q;
                if let Some(actor_id) = filter.actor_id {
                    q = q.bind(actor_id);
                }
                if let Some(action) = &filter.action {
                    q = q.bind(action);
                }
                if let Some(entity_type) = &filter.entity_type {
                    q = q.bind(entity_type);
                }
                if let Some(status) = &filter.status {
                    q = q.bind(status);
                }
                if let Some(date_from) = filter.date_from {
                    q = q.bind(date_from);
                }
                if let Some(date_to) = filter.date_to {
                    q = q.bind(date_to);
                }
                q
            }};
        }

        let rows = bind_filters!(sqlx::query_as::<_, AuditLog>(&sql))
            .bind(filter.page_size)
            .bind(filter.offset())
            .fetch_all(&self.pool)
            .await?;

        let total: i64 = bind_filters!(sqlx::query_scalar::<_, i64>(&count_sql))
            .fetch_one(&self.pool)
            .await?;

        Ok((rows, total))
    }
}
