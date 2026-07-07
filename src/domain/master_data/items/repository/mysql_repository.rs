use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::domain::master_data::items::{
    entity::{MasterDataItem, MasterDataItemFilter},
    repository::MasterDataItemsRepository,
};

pub struct MySqlMasterDataItemsRepository {
    db: Arc<MySqlPool>,
}

impl MySqlMasterDataItemsRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MasterDataItemsRepository for MySqlMasterDataItemsRepository {
    async fn create_item(&self, item: &MasterDataItem) -> Result<u64> {
        let result = sqlx::query(
            r#"
            INSERT INTO master_data_items
                (group_id, parent_id, code, name, metadata, sort_order, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, UTC_TIMESTAMP(), UTC_TIMESTAMP())
            "#,
        )
        .bind(item.group_id)
        .bind(item.parent_id)
        .bind(&item.code)
        .bind(&item.name)
        .bind(&item.metadata)
        .bind(item.sort_order)
        .bind(item.is_active)
        .execute(self.db.as_ref())
        .await?;

        Ok(result.last_insert_id())
    }

    async fn update_item(&self, item: &MasterDataItem) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE master_data_items
            SET
                parent_id = ?,
                name = ?,
                metadata = ?,
                sort_order = ?,
                is_active = ?,
                updated_at = UTC_TIMESTAMP()
            WHERE id = ? AND delete_marker IS NULL
            "#,
        )
        .bind(item.parent_id)
        .bind(&item.name)
        .bind(&item.metadata)
        .bind(item.sort_order)
        .bind(item.is_active)
        .bind(item.id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn delete_item(&self, id: u64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE master_data_items
            SET delete_marker = UUID(), deleted_at = UTC_TIMESTAMP()
            WHERE id = ? AND delete_marker IS NULL
            "#,
        )
        .bind(id)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn find_item_by_id(&self, id: u64) -> Result<Option<MasterDataItem>> {
        let item = sqlx::query_as::<_, MasterDataItem>(
            r#"
            SELECT id, group_id, parent_id, code, name, metadata, sort_order, is_active, created_at, updated_at
            FROM master_data_items
            WHERE id = ? AND delete_marker IS NULL
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(item)
    }

    async fn find_item_by_code(&self, group_id: u64, code: &str) -> Result<Option<MasterDataItem>> {
        let item = sqlx::query_as::<_, MasterDataItem>(
            r#"
            SELECT id, group_id, parent_id, code, name, metadata, sort_order, is_active, created_at, updated_at
            FROM master_data_items
            WHERE group_id = ? AND code = ? AND delete_marker IS NULL
            LIMIT 1
            "#,
        )
        .bind(group_id)
        .bind(code)
        .fetch_optional(self.db.as_ref())
        .await?;

        Ok(item)
    }

    async fn exists_item_code(&self, group_id: u64, code: &str) -> Result<bool> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM master_data_items WHERE group_id = ? AND code = ? AND delete_marker IS NULL",
        )
        .bind(group_id)
        .bind(code)
        .fetch_one(self.db.as_ref())
        .await?;

        Ok(count > 0)
    }

    async fn list_items(&self, filter: &MasterDataItemFilter) -> Result<Vec<MasterDataItem>> {
        let mut builder = QueryBuilder::<MySql>::new(
            r#"
            SELECT id, group_id, parent_id, code, name, metadata, sort_order, is_active, created_at, updated_at
            FROM master_data_items
            WHERE delete_marker IS NULL AND group_id =
            "#,
        );
        builder.push_bind(filter.group_id);

        push_item_filters(&mut builder, filter);

        let sort_by = match filter.sort_by.as_str() {
            "code" => "code",
            "name" => "name",
            "sort_order" => "sort_order",
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

        let items = builder
            .build_query_as::<MasterDataItem>()
            .fetch_all(self.db.as_ref())
            .await?;

        Ok(items)
    }

    async fn count_items(&self, filter: &MasterDataItemFilter) -> Result<u64> {
        let mut builder = QueryBuilder::<MySql>::new(
            "SELECT COUNT(*) FROM master_data_items WHERE delete_marker IS NULL AND group_id = ",
        );
        builder.push_bind(filter.group_id);

        push_item_filters(&mut builder, filter);

        let total: i64 = builder
            .build_query_scalar()
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(total as u64)
    }

    async fn list_options(
        &self,
        group_id: u64,
        parent_id: Option<u64>,
        only_root: bool,
    ) -> Result<Vec<MasterDataItem>> {
        let mut builder = QueryBuilder::<MySql>::new(
            r#"
            SELECT id, group_id, parent_id, code, name, metadata, sort_order, is_active, created_at, updated_at
            FROM master_data_items
            WHERE delete_marker IS NULL AND is_active = TRUE AND group_id =
            "#,
        );
        builder.push_bind(group_id);

        if only_root {
            builder.push(" AND parent_id IS NULL");
        } else if let Some(parent_id) = parent_id {
            builder.push(" AND parent_id = ");
            builder.push_bind(parent_id);
        }

        builder.push(" ORDER BY sort_order ASC, name ASC");

        let items = builder
            .build_query_as::<MasterDataItem>()
            .fetch_all(self.db.as_ref())
            .await?;

        Ok(items)
    }

    async fn count_items_in_group(&self, group_id: u64) -> Result<u64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM master_data_items WHERE group_id = ? AND delete_marker IS NULL",
        )
        .bind(group_id)
        .fetch_one(self.db.as_ref())
        .await?;

        Ok(count as u64)
    }

    async fn count_children(&self, item_id: u64) -> Result<u64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM master_data_items WHERE parent_id = ? AND delete_marker IS NULL",
        )
        .bind(item_id)
        .fetch_one(self.db.as_ref())
        .await?;

        Ok(count as u64)
    }
}

fn push_item_filters(builder: &mut QueryBuilder<MySql>, filter: &MasterDataItemFilter) {
    if filter.only_root {
        builder.push(" AND parent_id IS NULL");
    } else if let Some(parent_id) = filter.parent_id {
        builder.push(" AND parent_id = ");
        builder.push_bind(parent_id);
    }

    if let Some(is_active) = filter.is_active {
        builder.push(" AND is_active = ");
        builder.push_bind(is_active);
    }

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
