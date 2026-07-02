use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::MySqlPool;

use crate::domain::role_permission::{
    entity::RolePermission, repository::RolePermissionRepository,
};

pub struct MySqlRolePermissionRepository {
    db: Arc<MySqlPool>,
}

impl MySqlRolePermissionRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RolePermissionRepository for MySqlRolePermissionRepository {
    async fn assign(&self, role_id: u64, permission_ids: &[u64]) -> Result<()> {
        let mut tx = self.db.begin().await?;

        sqlx::query("DELETE FROM role_permissions WHERE role_id = ?")
            .bind(role_id)
            .execute(&mut *tx)
            .await?;

        for permission_id in permission_ids {
            sqlx::query(
                r#"
            INSERT INTO role_permissions
            (
                role_id,
                permission_id,
                created_at
            )
            VALUES (?, ?, ?)
            "#,
            )
            .bind(role_id)
            .bind(permission_id)
            .bind(Utc::now())
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
    async fn revoke(&self, role_id: u64, permission_id: u64) -> Result<()> {
        sqlx::query(
            r#"
        DELETE FROM role_permissions
        WHERE role_id = ?
          AND permission_id = ?
        "#,
        )
        .bind(role_id)
        .bind(permission_id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }
    async fn revoke_all(&self, role_id: u64) -> Result<()> {
        sqlx::query("DELETE FROM role_permissions WHERE role_id = ?")
            .bind(role_id)
            .execute(self.db.as_ref())
            .await?;

        Ok(())
    }
    async fn find_permissions(&self, role_id: u64) -> Result<Vec<RolePermission>> {
        let permissions = sqlx::query_as::<_, RolePermission>(
            r#"
        SELECT
            role_id,
            permission_id,
            created_at
        FROM role_permissions
        WHERE role_id = ?
        ORDER BY permission_id
        "#,
        )
        .bind(role_id)
        .fetch_all(self.db.as_ref())
        .await?;

        Ok(permissions)
    }
}
