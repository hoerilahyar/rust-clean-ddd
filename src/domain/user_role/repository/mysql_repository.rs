use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::MySqlPool;

use crate::domain::{role::entity::Role, user_role::repository::UserRoleRepository};

pub struct MySqlUserRoleRepository {
    db: Arc<MySqlPool>,
}

impl MySqlUserRoleRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRoleRepository for MySqlUserRoleRepository {
    async fn assign(&self, user_id: u64, role_ids: &[u64]) -> Result<()> {
        let mut tx = self.db.begin().await?;

        sqlx::query("DELETE FROM user_roles WHERE user_id = ?")
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        for role_id in role_ids {
            sqlx::query(
                r#"
            INSERT INTO user_roles
            (
                user_id,
                role_id,
                created_at
            )
            VALUES (?, ?, ?)
            "#,
            )
            .bind(user_id)
            .bind(role_id)
            .bind(Utc::now())
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
    async fn revoke(&self, user_id: u64, role_id: u64) -> Result<()> {
        sqlx::query(
            r#"
        DELETE FROM user_roles
        WHERE user_id = ?
          AND role_id = ?
        "#,
        )
        .bind(user_id)
        .bind(role_id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }
    async fn revoke_all(&self, user_id: u64) -> Result<()> {
        sqlx::query("DELETE FROM user_roles WHERE user_id = ?")
            .bind(user_id)
            .execute(self.db.as_ref())
            .await?;

        Ok(())
    }
    async fn find_roles(&self, user_id: u64) -> Result<Vec<Role>> {
        let roles = sqlx::query_as::<_, Role>(
            r#"
        SELECT
            user_id,
            role_id,
            created_at
        FROM user_roles
        WHERE user_id = ?
        ORDER BY role_id
        "#,
        )
        .bind(user_id)
        .fetch_all(self.db.as_ref())
        .await?;

        Ok(roles)
    }
}
