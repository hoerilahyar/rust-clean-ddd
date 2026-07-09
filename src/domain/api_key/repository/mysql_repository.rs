use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::domain::api_key::{
    entity::{ApiKey, ApiKeyRow},
    repository::ApiKeyRepository,
};

pub struct MySqlApiKeyRepository {
    db: Arc<MySqlPool>,
}

impl MySqlApiKeyRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

const SELECT_COLUMNS: &str = r#"
    id, name, key_prefix, key_hash, permissions, is_active,
    expires_at, last_used_at, created_by, created_at, updated_at, deleted_at
"#;

#[async_trait]
impl ApiKeyRepository for MySqlApiKeyRepository {
    async fn create(
        &self,
        name: &str,
        key_prefix: &str,
        key_hash: &str,
        permissions: &[String],
        expires_at: Option<chrono::NaiveDateTime>,
        created_by: Option<u64>,
    ) -> Result<ApiKey> {
        let now = Utc::now().naive_utc();
        let permissions_json = serde_json::to_value(permissions)?;

        let id = sqlx::query(
            r#"
            INSERT INTO api_keys
                (name, key_prefix, key_hash, permissions, is_active, expires_at, created_by, created_at)
            VALUES (?, ?, ?, ?, TRUE, ?, ?, ?)
            "#,
        )
        .bind(name)
        .bind(key_prefix)
        .bind(key_hash)
        .bind(&permissions_json)
        .bind(expires_at)
        .bind(created_by)
        .bind(now)
        .execute(self.db.as_ref())
        .await?
        .last_insert_id();

        self.find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("Failed to read back created API key"))
    }

    async fn find_all(&self) -> Result<Vec<ApiKey>> {
        let rows = sqlx::query_as::<_, ApiKeyRow>(&format!(
            r#"
            SELECT {SELECT_COLUMNS}
            FROM api_keys
            WHERE delete_marker IS NULL
            ORDER BY created_at DESC
            "#
        ))
        .fetch_all(self.db.as_ref())
        .await?;

        rows.into_iter().map(ApiKey::try_from).collect()
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<ApiKey>> {
        let row = sqlx::query_as::<_, ApiKeyRow>(&format!(
            r#"
            SELECT {SELECT_COLUMNS}
            FROM api_keys
            WHERE id = ? AND delete_marker IS NULL
            "#
        ))
        .bind(id)
        .fetch_optional(self.db.as_ref())
        .await?;

        row.map(ApiKey::try_from).transpose()
    }

    async fn find_by_prefix(&self, key_prefix: &str) -> Result<Option<ApiKey>> {
        let row = sqlx::query_as::<_, ApiKeyRow>(&format!(
            r#"
            SELECT {SELECT_COLUMNS}
            FROM api_keys
            WHERE key_prefix = ? AND delete_marker IS NULL
            "#
        ))
        .bind(key_prefix)
        .fetch_optional(self.db.as_ref())
        .await?;

        row.map(ApiKey::try_from).transpose()
    }

    async fn update(
        &self,
        id: u64,
        name: &str,
        permissions: &[String],
        expires_at: Option<chrono::NaiveDateTime>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let permissions_json = serde_json::to_value(permissions)?;

        sqlx::query(
            r#"
            UPDATE api_keys
            SET name = ?, permissions = ?, expires_at = ?, updated_at = ?
            WHERE id = ? AND delete_marker IS NULL
            "#,
        )
        .bind(name)
        .bind(&permissions_json)
        .bind(expires_at)
        .bind(now)
        .bind(id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn set_active(&self, id: u64, is_active: bool) -> Result<()> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE api_keys
            SET is_active = ?, updated_at = ?
            WHERE id = ? AND delete_marker IS NULL
            "#,
        )
        .bind(is_active)
        .bind(now)
        .bind(id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn touch_last_used(&self, id: u64) -> Result<()> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE api_keys
            SET last_used_at = ?
            WHERE id = ? AND delete_marker IS NULL
            "#,
        )
        .bind(now)
        .bind(id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn delete_by_id(&self, id: u64, delete_marker: &str) -> Result<()> {
        let marker = Uuid::parse_str(delete_marker).unwrap_or_else(|_| Uuid::new_v4());
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE api_keys
            SET delete_marker = ?, deleted_at = ?
            WHERE id = ? AND delete_marker IS NULL
            "#,
        )
        .bind(marker.to_string())
        .bind(now)
        .bind(id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }
}
