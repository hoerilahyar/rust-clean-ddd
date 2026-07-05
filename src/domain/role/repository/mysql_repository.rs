use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::domain::role::{
    entity::{Role, RoleFilter},
    repository::RoleRepository,
};

pub struct MySqlRoleRepository {
    db: Arc<MySqlPool>,
}

impl MySqlRoleRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RoleRepository for MySqlRoleRepository {
    async fn create(&self, role: &Role) -> Result<u64> {
        let result = sqlx::query(
            r#"
            INSERT INTO roles
            (
                code,
                name,
                description,
                is_active,
                created_at,
                updated_at
            )
            VALUES (?, ?, ?, ?, UTC_TIMESTAMP(), UTC_TIMESTAMP())
            "#,
        )
        .bind(&role.code)
        .bind(&role.name)
        .bind(&role.description)
        .bind(role.is_active)
        .execute(self.db.as_ref())
        .await?;

        Ok(result.last_insert_id())
    }

    async fn update(&self, role: &Role) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE roles
            SET
                name = ?,
                description = ?,
                is_active = ?,
                updated_at = UTC_TIMESTAMP()
            WHERE id = ?
            "#,
        )
        .bind(&role.name)
        .bind(&role.description)
        .bind(role.is_active)
        .bind(role.id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }
    async fn delete(&self, id: u64) -> Result<()> {
        sqlx::query("DELETE FROM roles WHERE id = ?")
            .bind(id)
            .execute(self.db.as_ref())
            .await?;

        Ok(())
    }
    async fn find_by_id(&self, id: u64) -> Result<Option<Role>> {
        let role = sqlx::query_as::<_, Role>(
            r#"
            SELECT
                id,
                code,
                name,
                description,
                is_active,
                created_at,
                updated_at
            FROM roles
            WHERE id = ?
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(role)
    }
    async fn find_by_code(&self, code: &str) -> Result<Option<Role>> {
        let role = sqlx::query_as::<_, Role>(
            r#"
            SELECT
                id,
                code,
                name,
                description,
                is_active,
                created_at,
                updated_at
            FROM roles
            WHERE code = ?
            LIMIT 1
            "#,
        )
        .bind(code)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(role)
    }
    async fn exists_code(&self, code: &str) -> Result<bool> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM roles WHERE code = ?")
            .bind(code)
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(count > 0)
    }
    async fn list(&self, filter: &RoleFilter) -> Result<Vec<Role>> {
        let mut builder = QueryBuilder::<MySql>::new(
            r#"
        SELECT
            id,
            code,
            name,
            description,
            is_active,
            created_at,
            updated_at
        FROM roles
        WHERE 1 = 1
        "#,
        );

        if let Some(search) = &filter.search {
            if !search.trim().is_empty() {
                builder.push(" AND (");

                builder.push("code LIKE ");
                builder.push_bind(format!("%{}%", search));

                builder.push(" OR name LIKE ");
                builder.push_bind(format!("%{}%", search));

                builder.push(" OR description LIKE ");
                builder.push_bind(format!("%{}%", search));

                builder.push(")");
            }
        }

        let sort_by = match filter.sort_by.as_str() {
            "code" => "code",
            "name" => "name",
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

        let roles = builder
            .build_query_as::<Role>()
            .fetch_all(self.db.as_ref())
            .await?;

        Ok(roles)
    }
    async fn count(&self, filter: &RoleFilter) -> Result<u64> {
        let mut builder = QueryBuilder::<MySql>::new("SELECT COUNT(*) FROM roles WHERE 1 = 1");

        if let Some(search) = &filter.search {
            if !search.trim().is_empty() {
                builder.push(" AND (");

                builder.push("code LIKE ");
                builder.push_bind(format!("%{}%", search));

                builder.push(" OR name LIKE ");
                builder.push_bind(format!("%{}%", search));

                builder.push(" OR description LIKE ");
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
}
