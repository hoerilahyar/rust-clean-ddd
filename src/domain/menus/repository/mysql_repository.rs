use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::domain::menus::{
    entity::{Menu, MenuFilter},
    repository::MenuRepository,
};

pub struct MySqlMenuRepository {
    db: Arc<MySqlPool>,
}

impl MySqlMenuRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MenuRepository for MySqlMenuRepository {
    async fn create(&self, menu: &Menu) -> Result<u64> {
        let result = sqlx::query(
            r#"
            INSERT INTO menus
            (
                parent_id,
                name,
                icon,
                path,
                sort_order,
                is_active,
                created_at,
                updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, UTC_TIMESTAMP(), UTC_TIMESTAMP())
            "#,
        )
        .bind(&menu.parent_id)
        .bind(&menu.name)
        .bind(&menu.icon)
        .bind(&menu.path)
        .bind(&menu.sort_order)
        .bind(menu.is_active)
        .execute(self.db.as_ref())
        .await?;

        Ok(result.last_insert_id())
    }

    async fn update(&self, menu: &Menu) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE menus
            SET
                parent_id = ?,
                name = ?,
                icon = ?,
                path = ?,
                sort_order = ?,
                is_active = ?,
                updated_at = UTC_TIMESTAMP()
            WHERE id = ?
            "#,
        )
        .bind(&menu.parent_id)
        .bind(&menu.name)
        .bind(&menu.icon)
        .bind(&menu.path)
        .bind(&menu.sort_order)
        .bind(menu.is_active)
        .bind(menu.id)
        .execute(self.db.as_ref())
        .await?;

        println!("menu: {:?}", menu);
        Ok(())
    }
    async fn delete(&self, id: u64) -> Result<()> {
        sqlx::query("DELETE FROM menus WHERE id = ?")
            .bind(id)
            .execute(self.db.as_ref())
            .await?;

        Ok(())
    }
    async fn find_by_id(&self, id: u64) -> Result<Option<Menu>> {
        let menu = sqlx::query_as::<_, Menu>(
            r#"
            SELECT
                id,
                parent_id,
                name,
                icon,
                path,
                sort_order,
                is_active,
                created_at,
                updated_at
            FROM menus
            WHERE id = ?
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(menu)
    }
    async fn find_by_name(&self, name: &str) -> Result<Option<Menu>> {
        let menu = sqlx::query_as::<_, Menu>(
            r#"
            SELECT
                id,
                parent_id,
                name,
                icon,
                path,
                sort_order,
                is_active,
                created_at,
                updated_at
            FROM menus
            WHERE name = ?
            LIMIT 1
            "#,
        )
        .bind(name)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(menu)
    }
    async fn exists_name(&self, name: &str) -> Result<bool> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM menus WHERE name = ?")
            .bind(name)
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(count > 0)
    }
    async fn list(&self, filter: &MenuFilter) -> Result<Vec<Menu>> {
        let mut builder = QueryBuilder::<MySql>::new(
            r#"
        SELECT
            id,
            parent_id,
            name,
            icon,
            path,
            sort_order,
            is_active,
            created_at,
            updated_at
        FROM menus
        WHERE 1 = 1
        "#,
        );

        if let Some(search) = &filter.search {
            if !search.trim().is_empty() {
                builder.push(" AND (");

                builder.push(" OR name LIKE ");
                builder.push_bind(format!("%{}%", search));

                builder.push(" OR path LIKE ");
                builder.push_bind(format!("%{}%", search));

                builder.push(")");
            }
        }

        let sort_by = match filter.sort_by.as_str() {
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

        let menus = builder
            .build_query_as::<Menu>()
            .fetch_all(self.db.as_ref())
            .await?;

        Ok(menus)
    }
    async fn count(&self, filter: &MenuFilter) -> Result<u64> {
        let mut builder = QueryBuilder::<MySql>::new("SELECT COUNT(*) FROM menus WHERE 1 = 1");

        if let Some(search) = &filter.search {
            if !search.trim().is_empty() {
                builder.push(" AND (");

                builder.push(" OR name LIKE ");
                builder.push_bind(format!("%{}%", search));

                builder.push(" OR path LIKE ");
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
