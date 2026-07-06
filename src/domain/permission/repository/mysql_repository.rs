use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::domain::permission::{
    entity::{Permission, PermissionFilter},
    repository::PermissionRepository,
};

pub struct MySqlPermissionRepository {
    db: Arc<MySqlPool>,
}

impl MySqlPermissionRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PermissionRepository for MySqlPermissionRepository {
    async fn create(&self, permission: &Permission) -> Result<u64> {
        let result = sqlx::query(
            r#"
        INSERT INTO permissions
        (
            code,
            name,
            resource,
            action,
            description,
            is_active,
            created_at,
            updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, UTC_TIMESTAMP(), UTC_TIMESTAMP())
        "#,
        )
        .bind(&permission.code)
        .bind(&permission.name)
        .bind(&permission.resource)
        .bind(&permission.action)
        .bind(&permission.description)
        .bind(permission.is_active)
        .execute(self.db.as_ref())
        .await?;

        Ok(result.last_insert_id())
    }

    async fn update(&self, permission: &Permission) -> Result<()> {
        sqlx::query(
            r#"
        UPDATE permissions
        SET
            code = ?,
            name = ?,
            description = ?,
            is_active = ?,
            updated_at = UTC_TIMESTAMP()
        WHERE id = ?
        "#,
        )
        .bind(&permission.code)
        .bind(&permission.name)
        .bind(&permission.description)
        .bind(permission.is_active)
        .bind(permission.id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }
    async fn delete(&self, id: u64) -> Result<()> {
        sqlx::query("DELETE FROM permissions WHERE id = ?")
            .bind(id)
            .execute(self.db.as_ref())
            .await?;

        Ok(())
    }
    async fn find_by_id(&self, id: u64) -> Result<Option<Permission>> {
        let permission = sqlx::query_as::<_, Permission>(
            r#"
        SELECT
            id,
            code,
            name,
            resource,
            action,
            description,
            is_active,
            created_at,
            updated_at
        FROM permissions
        WHERE id = ?
        LIMIT 1
        "#,
        )
        .bind(id)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(permission)
    }
    async fn find_by_code(&self, code: &str) -> Result<Option<Permission>> {
        let permission = sqlx::query_as::<_, Permission>(
            r#"
        SELECT
            id,
            code,
            name,
            resource,
            action,
            description,
            is_active,
            created_at,
            updated_at
        FROM permissions
        WHERE code = ?
        LIMIT 1
        "#,
        )
        .bind(code)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(permission)
    }
    async fn exists_code(&self, code: &str) -> Result<bool> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM permissions WHERE code = ?")
            .bind(code)
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(count > 0)
    }
    async fn list(&self, filter: &PermissionFilter) -> Result<Vec<Permission>> {
        let mut builder = QueryBuilder::<MySql>::new(
            r#"
        SELECT
            id,
            code,
            name,
            resource,
            action,
            description,
            is_active,
            created_at,
            updated_at
        FROM permissions
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

        if let Some(resource) = &filter.resource {
            if !resource.trim().is_empty() {
                builder.push(" AND resource = ");
                builder.push_bind(resource);
            }
        }

        let sort_by = match filter.sort_by.as_str() {
            "code" => "code",
            "name" => "name",
            "resource" => "resource",
            "action" => "action",
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

        let permissions = builder
            .build_query_as::<Permission>()
            .fetch_all(self.db.as_ref())
            .await?;

        Ok(permissions)
    }
    async fn count(&self, filter: &PermissionFilter) -> Result<u64> {
        let mut builder =
            QueryBuilder::<MySql>::new("SELECT COUNT(*) FROM permissions WHERE 1 = 1");

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

        if let Some(resource) = &filter.resource {
            if !resource.trim().is_empty() {
                builder.push(" AND resource = ");
                builder.push_bind(resource);
            }
        }

        let total: i64 = builder
            .build_query_scalar()
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(total as u64)
    }
}
