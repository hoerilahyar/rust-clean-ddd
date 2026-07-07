use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::domain::system_settings::{
    entity::{SystemSetting, SystemSettingRow},
    repository::SystemSettingRepository,
};

pub struct MySqlSystemSettingRepository {
    db: Arc<MySqlPool>,
}

impl MySqlSystemSettingRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SystemSettingRepository for MySqlSystemSettingRepository {
    async fn find_all(&self) -> Result<Vec<SystemSetting>> {
        let rows = sqlx::query_as::<_, SystemSettingRow>(
            r#"
        SELECT id, setting_key, setting_value, data_type, description,
               is_public, is_active, created_at, updated_at, deleted_at
        FROM system_settings
        WHERE delete_marker IS NULL
        ORDER BY setting_key ASC
        "#,
        )
        .fetch_all(self.db.as_ref())
        .await?;

        rows.into_iter().map(SystemSetting::try_from).collect()
    }

    async fn find_by_key(&self, key: &str) -> Result<Option<SystemSetting>> {
        let row = sqlx::query_as::<_, SystemSettingRow>(
            r#"
        SELECT id, setting_key, setting_value, data_type, description,
               is_public, is_active, created_at, updated_at, deleted_at
        FROM system_settings
        WHERE setting_key = ? AND delete_marker IS NULL
        "#,
        )
        .bind(key)
        .fetch_optional(self.db.as_ref())
        .await?;

        row.map(SystemSetting::try_from).transpose()
    }

    async fn upsert(
        &self,
        key: &str,
        value: Option<String>,
        data_type: &str,
        description: Option<String>,
        is_public: bool,
    ) -> Result<SystemSetting> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            INSERT INTO system_settings
                (setting_key, setting_value, data_type, description, is_public, is_active, created_at)
            VALUES (?, ?, ?, ?, ?, TRUE, ?)
            ON DUPLICATE KEY UPDATE
                setting_value = VALUES(setting_value),
                data_type = VALUES(data_type),
                description = VALUES(description),
                is_public = VALUES(is_public),
                updated_at = VALUES(created_at)
            "#,
        )
        .bind(key)
        .bind(value)
        .bind(data_type)
        .bind(description)
        .bind(is_public)
        .bind(now)
        .execute(self.db.as_ref())
        .await?;

        self.find_by_key(key)
            .await?
            .ok_or_else(|| anyhow!("Failed to read back upserted setting"))
    }

    async fn set_active(&self, key: &str, is_active: bool) -> Result<()> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE system_settings
            SET is_active = ?, updated_at = ?
            WHERE setting_key = ? AND delete_marker IS NULL
            "#,
        )
        .bind(is_active)
        .bind(now)
        .bind(key)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn delete_by_key(&self, key: &str, delete_marker: &str) -> Result<()> {
        let marker = Uuid::parse_str(delete_marker).unwrap_or_else(|_| Uuid::new_v4());
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE system_settings
            SET delete_marker = ?, deleted_at = ?
            WHERE setting_key = ? AND delete_marker IS NULL
            "#,
        )
        .bind(marker.to_string())
        .bind(now)
        .bind(key)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }
}
