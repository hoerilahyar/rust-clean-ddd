use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::MySqlPool;

use crate::domain::auth::dto::RefreshTokenRequest;
use crate::domain::auth::entity::{AuthUser, Permission, RefreshToken, Role};
use crate::domain::auth::repository::auth_repository::AuthRepository;

pub struct MySqlAuthRepository {
    pool: MySqlPool,
}

impl MySqlAuthRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for MySqlAuthRepository {
    async fn find_by_id(&self, user_id: u64) -> anyhow::Result<Option<AuthUser>> {
        let user = sqlx::query_as::<_, AuthUser>(
            r#"
        SELECT
            id,
            username,
            email,
            password,
            fullname,
            is_active,
            created_at,
            updated_at
        FROM users
        WHERE id = ?
        LIMIT 1
        "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(user))
    }

    async fn find_by_username_or_email(&self, value: &str) -> Result<Option<AuthUser>> {
        let user = sqlx::query_as::<_, AuthUser>(
            r#"
            SELECT
                id,
                username,
                email,
                password,
                fullname,
                phone,
                avatar,
                is_active,
                last_login_at,
                created_at,
                updated_at,
                deleted_at
            FROM users
            WHERE deleted_at IS NULL
              AND is_active = TRUE
              AND (
                    username = ?
                 OR email = ?
              )
            LIMIT 1
            "#,
        )
        .bind(value)
        .bind(value)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_roles(&self, user_id: u64) -> Result<Vec<Role>> {
        let roles = sqlx::query_as::<_, Role>(
            r#"
            SELECT
                r.id,
                r.name,
                r.slug,
                r.description,
                r.is_active,
                r.created_at,
                r.updated_at,
                r.deleted_at
            FROM roles r
            INNER JOIN user_roles ur
                ON ur.role_id = r.id
            WHERE
                ur.user_id = ?
                AND r.deleted_at IS NULL
                AND r.is_active = TRUE
            ORDER BY
                r.name ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
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
                p.resource,
                p.action,
                p.slug,
                p.description,
                p.created_at,
                p.updated_at
            FROM permissions p
            INNER JOIN role_permissions rp
                ON rp.permission_id = p.id
            WHERE
                rp.role_id IN ({})
            ORDER BY
                p.resource ASC,
                p.action ASC
            "#,
            placeholders
        );

        let mut query = sqlx::query_as::<_, Permission>(&sql);

        for role_id in role_ids {
            query = query.bind(role_id);
        }

        let permissions = query.fetch_all(&self.pool).await?;

        Ok(permissions)
    }
    async fn insert_refresh_token(&self, token: &RefreshToken) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (
                user_id,
                device_id,
                ip_address,
                token,
                expired_at,
                revoked_at,
                created_at
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?
            )
            "#,
        )
        .bind(token.user_id)
        .bind(&token.device_id)
        .bind(&token.ip_address)
        .bind(&token.token)
        .bind(token.expired_at)
        .bind(token.revoked_at)
        .bind(token.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_refresh_token(&self, token: RefreshTokenRequest) -> Result<Option<RefreshToken>> {
        let refresh_token = sqlx::query_as::<_, RefreshToken>(
            r#"
            SELECT
                id,
                user_id,
                device_id,
                ip_address,
                token,
                expired_at,
                revoked_at,
                created_at
            FROM refresh_tokens
            WHERE
                token = ?
                AND revoked_at IS NULL
                AND expired_at > UTC_TIMESTAMP()
            LIMIT 1
            "#,
        )
        .bind(token.refresh_token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(refresh_token)
    }

    async fn revoke_refresh_token(&self, id: u64) -> anyhow::Result<()> {
        sqlx::query(
            r#"
        DELETE FROM refresh_tokens
        WHERE id = ?
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn revoke_all_refresh_tokens(&self, user_id: u64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = UTC_TIMESTAMP()
            WHERE
                user_id = ?
                AND revoked_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_last_login(&self, user_id: u64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET
                last_login_at = ?,
                updated_at = ?
            WHERE
                id = ?
                AND deleted_at IS NULL
            "#,
        )
        .bind(Utc::now())
        .bind(Utc::now())
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
