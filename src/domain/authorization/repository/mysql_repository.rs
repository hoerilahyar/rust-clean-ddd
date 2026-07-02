use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::domain::{
    authorization::repository::AuthorizationRepository, permission::entity::Permission,
    role::entity::Role, user::entity::User,
};

pub struct MySqlAuthorizationRepository {
    db: Arc<MySqlPool>,
}

impl MySqlAuthorizationRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthorizationRepository for MySqlAuthorizationRepository {
    async fn find_user(&self, user_id: u64) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT
                id,
                username,
                fullname,
                email,
                password,
                is_active,
                last_login_at,
                created_at,
                updated_at
            FROM users
            WHERE id = ?
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(user)
    }

    async fn find_roles(&self, user_id: u64) -> Result<Vec<Role>> {
        let roles = sqlx::query_as::<_, Role>(
            r#"
            SELECT
                r.id,
                r.code,
                r.name,
                r.description,
                r.is_active,
                r.created_at,
                r.updated_at
            FROM user_roles ur
            INNER JOIN roles r
                ON r.id = ur.role_id
            WHERE ur.user_id = ?
            ORDER BY r.code
            "#,
        )
        .bind(user_id)
        .fetch_all(self.db.as_ref())
        .await?;

        Ok(roles)
    }

    async fn find_permissions(&self, role_ids: &[u64]) -> Result<Vec<Permission>> {
        if role_ids.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders = std::iter::repeat("?")
            .take(role_ids.len())
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!(
            r#"
            SELECT DISTINCT
                p.id,
                p.code,
                p.name,
                p.resource,
                p.action,
                p.description,
                p.is_active,
                p.created_at,
                p.updated_at
            FROM role_permissions rp
            INNER JOIN permissions p
                ON p.id = rp.permission_id
            WHERE rp.role_id IN ({})
            ORDER BY p.code
            "#,
            placeholders
        );

        let mut query = sqlx::query_as::<_, Permission>(&sql);

        for role_id in role_ids {
            query = query.bind(role_id);
        }

        let permissions = query.fetch_all(self.db.as_ref()).await?;

        Ok(permissions)
    }
}
