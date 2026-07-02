use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::domain::user::{
    entity::{User, UserFilter},
    repository::UserRepository,
};

pub struct MySqlUserRepository {
    db: Arc<MySqlPool>,
}

impl MySqlUserRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn find_by_id(&self, id: u64) -> Result<Option<User>> {
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
        "#,
        )
        .bind(id)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(user)
    }
    async fn create(&self, user: &User) -> Result<u64> {
        let result = sqlx::query(
            r#"
        INSERT INTO users
        (
            username,
            fullname,
            email,
            password,
            is_active,
            last_login_at,
            created_at,
            updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(&user.username)
        .bind(&user.fullname)
        .bind(&user.email)
        .bind(&user.password)
        .bind(user.is_active)
        .bind(user.last_login_at)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(self.db.as_ref())
        .await?;

        Ok(result.last_insert_id())
    }

    async fn update(&self, user: &User) -> Result<()> {
        sqlx::query(
            r#"
        UPDATE users
        SET
            fullname = ?,
            email = ?,
            is_active = ?,
            updated_at = ?
        WHERE id = ?
        "#,
        )
        .bind(&user.fullname)
        .bind(&user.email)
        .bind(user.is_active)
        .bind(user.updated_at)
        .bind(user.id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
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
        WHERE username = ?
        LIMIT 1
        "#,
        )
        .bind(username)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
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
        WHERE email = ?
        LIMIT 1
        "#,
        )
        .bind(email)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(user)
    }
    async fn exists_username(&self, username: &str) -> Result<bool> {
        let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(exists > 0)
    }
    async fn exists_email(&self, email: &str) -> Result<bool> {
        let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = ?")
            .bind(email)
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(exists > 0)
    }
    async fn list(&self, filter: &UserFilter) -> Result<Vec<User>> {
        let mut builder = QueryBuilder::<MySql>::new(
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
        WHERE 1 = 1
        "#,
        );

        if let Some(search) = &filter.search {
            if !search.trim().is_empty() {
                builder.push(" AND (username LIKE ");
                builder.push_bind(format!("%{}%", search));
                builder.push(" OR fullname LIKE ");
                builder.push_bind(format!("%{}%", search));
                builder.push(" OR email LIKE ");
                builder.push_bind(format!("%{}%", search));
                builder.push(")");
            }
        }

        let sort_by = match filter.sort_by.as_str() {
            "username" => "username",
            "fullname" => "fullname",
            "email" => "email",
            "created_at" => "created_at",
            _ => "id",
        };

        let sort_type = if filter.sort_type.eq_ignore_ascii_case("ASC") {
            "ASC"
        } else {
            "DESC"
        };

        builder.push(format!(" ORDER BY {} {}", sort_by, sort_type));

        builder.push(" LIMIT ");
        builder.push_bind(filter.page_size as i64);

        builder.push(" OFFSET ");
        builder.push_bind(((filter.page - 1) * filter.page_size) as i64);

        let users = builder
            .build_query_as::<User>()
            .fetch_all(self.db.as_ref())
            .await?;

        Ok(users)
    }
    async fn count(&self, filter: &UserFilter) -> Result<u64> {
        let mut builder = QueryBuilder::<MySql>::new("SELECT COUNT(*) FROM users WHERE 1 = 1");

        if let Some(search) = &filter.search {
            if !search.trim().is_empty() {
                builder.push(" AND (username LIKE ");
                builder.push_bind(format!("%{}%", search));
                builder.push(" OR fullname LIKE ");
                builder.push_bind(format!("%{}%", search));
                builder.push(" OR email LIKE ");
                builder.push_bind(format!("%{}%", search));
                builder.push(")");
            }
        }

        let total: i64 = builder
            .build_query_scalar()
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(total as u64)
    }

    async fn delete(&self, id: u64) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(self.db.as_ref())
            .await?;

        Ok(())
    }
}
