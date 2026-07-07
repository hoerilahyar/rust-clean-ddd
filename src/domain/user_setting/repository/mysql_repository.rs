use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::domain::user_setting::{
    entity::{UserSetting, UserSettingRow},
    repository::UserSettingRepository,
};

pub struct MySqlUserSettingRepository {
    db: Arc<MySqlPool>,
}

impl MySqlUserSettingRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserSettingRepository for MySqlUserSettingRepository {
    async fn find_all(&self, user_id: u64) -> Result<Vec<UserSetting>> {
        let rows = sqlx::query_as::<_, UserSettingRow>(
            r#"
        SELECT id, user_id, setting_key, setting_value, data_type, description,
               is_active, created_at, updated_at, deleted_at
        FROM user_settings
        WHERE user_id = ? AND delete_marker IS NULL
        ORDER BY setting_key ASC
        "#,
        )
        .bind(user_id)
        .fetch_all(self.db.as_ref())
        .await?;

        rows.into_iter().map(UserSetting::try_from).collect()
    }

    async fn find_by_key(&self, user_id: u64, key: &str) -> Result<Option<UserSetting>> {
        let row = sqlx::query_as::<_, UserSettingRow>(
            r#"
        SELECT id, user_id, setting_key, setting_value, data_type, description,
               is_active, created_at, updated_at, deleted_at
        FROM user_settings
        WHERE user_id = ? AND setting_key = ? AND delete_marker IS NULL
        "#,
        )
        .bind(user_id)
        .bind(key)
        .fetch_optional(self.db.as_ref())
        .await?;

        row.map(UserSetting::try_from).transpose()
    }

    async fn upsert(
        &self,
        user_id: u64,
        key: &str,
        value: Option<String>,
        data_type: &str,
        description: Option<String>,
    ) -> Result<UserSetting> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            INSERT INTO user_settings
                (user_id, setting_key, setting_value, data_type, description, is_active, created_at)
            VALUES (?, ?, ?, ?, ?, TRUE, ?)
            ON DUPLICATE KEY UPDATE
                setting_value = VALUES(setting_value),
                data_type = VALUES(data_type),
                description = VALUES(description),
                updated_at = VALUES(created_at)
            "#,
        )
        .bind(user_id)
        .bind(key)
        .bind(value)
        .bind(data_type)
        .bind(description)
        .bind(now)
        .execute(self.db.as_ref())
        .await?;

        self.find_by_key(user_id, key)
            .await?
            .ok_or_else(|| anyhow!("Failed to read back upserted setting"))
    }

    async fn set_active(&self, user_id: u64, key: &str, is_active: bool) -> Result<()> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE user_settings
            SET is_active = ?, updated_at = ?
            WHERE user_id = ? AND setting_key = ? AND delete_marker IS NULL
            "#,
        )
        .bind(is_active)
        .bind(now)
        .bind(user_id)
        .bind(key)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn delete_by_key(&self, user_id: u64, key: &str, delete_marker: &str) -> Result<()> {
        let marker = Uuid::parse_str(delete_marker).unwrap_or_else(|_| Uuid::new_v4());
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE user_settings
            SET delete_marker = ?, deleted_at = ?
            WHERE user_id = ? AND setting_key = ? AND delete_marker IS NULL
            "#,
        )
        .bind(marker.to_string())
        .bind(now)
        .bind(user_id)
        .bind(key)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }
}
