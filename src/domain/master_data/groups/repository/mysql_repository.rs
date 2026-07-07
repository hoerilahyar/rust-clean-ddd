use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::domain::master_data::groups::{
    entity::{MasterDataGroup, MasterDataGroupFilter},
    repository::MasterDataGroupRepository,
};

pub struct MySqlMasterDataGroupRepository {
    db: Arc<MySqlPool>,
}

impl MySqlMasterDataGroupRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MasterDataGroupRepository for MySqlMasterDataGroupRepository {
    async fn create_group(&self, group: &MasterDataGroup) -> Result<u64> {
        let result = sqlx::query(
            r#"
            INSERT INTO master_data_groups
                (code, name, description, is_hierarchical, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, UTC_TIMESTAMP(), UTC_TIMESTAMP())
            "#,
        )
        .bind(&group.code)
        .bind(&group.name)
        .bind(&group.description)
        .bind(group.is_hierarchical)
        .bind(group.is_active)
        .execute(self.db.as_ref())
        .await?;

        Ok(result.last_insert_id())
    }

    async fn update_group(&self, group: &MasterDataGroup) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE master_data_groups
            SET
                name = ?,
                description = ?,
                is_hierarchical = ?,
                is_active = ?,
                updated_at = UTC_TIMESTAMP()
            WHERE id = ? AND delete_marker IS NULL
            "#,
        )
        .bind(&group.name)
        .bind(&group.description)
        .bind(group.is_hierarchical)
        .bind(group.is_active)
        .bind(group.id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn delete_group(&self, id: u64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE master_data_groups
            SET delete_marker = UUID(), deleted_at = UTC_TIMESTAMP()
            WHERE id = ? AND delete_marker IS NULL
            "#,
        )
        .bind(id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn find_group_by_id(&self, id: u64) -> Result<Option<MasterDataGroup>> {
        let group = sqlx::query_as::<_, MasterDataGroup>(
            r#"
            SELECT id, code, name, description, is_hierarchical, is_active, created_at, updated_at
            FROM master_data_groups
            WHERE id = ? AND delete_marker IS NULL
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(group)
    }

    async fn find_group_by_code(&self, code: &str) -> Result<Option<MasterDataGroup>> {
        let group = sqlx::query_as::<_, MasterDataGroup>(
            r#"
            SELECT id, code, name, description, is_hierarchical, is_active, created_at, updated_at
            FROM master_data_groups
            WHERE code = ? AND delete_marker IS NULL
            LIMIT 1
            "#,
        )
        .bind(code)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(group)
    }

    async fn exists_group_code(&self, code: &str) -> Result<bool> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM master_data_groups WHERE code = ? AND delete_marker IS NULL",
        )
        .bind(code)
        .fetch_one(self.db.as_ref())
        .await?;

        Ok(count > 0)
    }

    async fn list_groups(&self, filter: &MasterDataGroupFilter) -> Result<Vec<MasterDataGroup>> {
        let mut builder = QueryBuilder::<MySql>::new(
            r#"
            SELECT id, code, name, description, is_hierarchical, is_active, created_at, updated_at
            FROM master_data_groups
            WHERE delete_marker IS NULL
            "#,
        );

        push_group_search(&mut builder, filter);

        let sort_by = match filter.sort_by.as_str() {
            "code" => "code",
            "name" => "name",
            _ => "created_at",
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

        let groups = builder
            .build_query_as::<MasterDataGroup>()
            .fetch_all(self.db.as_ref())
            .await?;

        Ok(groups)
    }

    async fn count_groups(&self, filter: &MasterDataGroupFilter) -> Result<u64> {
        let mut builder = QueryBuilder::<MySql>::new(
            "SELECT COUNT(*) FROM master_data_groups WHERE delete_marker IS NULL",
        );

        push_group_search(&mut builder, filter);

        let total: i64 = builder
            .build_query_scalar()
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(total as u64)
    }
}

fn push_group_search(builder: &mut QueryBuilder<MySql>, filter: &MasterDataGroupFilter) {
    if let Some(search) = &filter.search {
        if !search.trim().is_empty() {
            builder.push(" AND (code LIKE ");
            builder.push_bind(format!("%{}%", search));
            builder.push(" OR name LIKE ");
            builder.push_bind(format!("%{}%", search));
            builder.push(")");
        }
    }
}
