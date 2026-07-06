use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::domain::menu_permissions::{
    entity::MenuPermission, repository::MenuPermissionRepository,
};

pub struct MySqlMenuPermissionRepository {
    db: Arc<MySqlPool>,
}

impl MySqlMenuPermissionRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MenuPermissionRepository for MySqlMenuPermissionRepository {
    async fn assign(&self, menu_id: u64, permission_ids: &[u64]) -> Result<()> {
        let mut tx = self.db.begin().await?;

        for permission_id in permission_ids {
            sqlx::query("DELETE FROM menu_permissions WHERE menu_id = ? AND permission_id = ?")
                .bind(menu_id)
                .bind(permission_id)
                .execute(&mut *tx)
                .await?;

            sqlx::query(
                r#"
            INSERT INTO menu_permissions
            (
                menu_id,
                permission_id
            )
            VALUES (?, ?)
            "#,
            )
            .bind(menu_id)
            .bind(permission_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
    async fn revoke(&self, menu_id: u64, permission_id: u64) -> Result<()> {
        sqlx::query(
            r#"
        DELETE FROM menu_permissions
        WHERE menu_id = ?
          AND permission_id = ?
        "#,
        )
        .bind(menu_id)
        .bind(permission_id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }
    async fn revoke_all(&self, menu_id: u64) -> Result<()> {
        sqlx::query("DELETE FROM menu_permissions WHERE menu_id = ?")
            .bind(menu_id)
            .execute(self.db.as_ref())
            .await?;

        Ok(())
    }
    async fn find_permissions(&self, menu_id: u64) -> Result<Vec<MenuPermission>> {
        let permissions = sqlx::query_as::<_, MenuPermission>(
            r#"
        SELECT
            menu_id,
            permission_id
        FROM menu_permissions
        WHERE menu_id = ?
        ORDER BY permission_id
        "#,
        )
        .bind(menu_id)
        .fetch_all(self.db.as_ref())
        .await?;

        Ok(permissions)
    }
}
