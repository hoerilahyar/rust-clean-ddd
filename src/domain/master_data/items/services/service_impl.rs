use std::{sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;

use crate::{
    domain::{
        audit_log::{
            entity::audit_action,
            services::{AuditLogService, RecordAuditLogInput},
        },
        master_data::{
            groups::{
                dto::{MasterDataOptionListResponse, MasterDataOptionResponse},
                entity::MasterDataGroup,
                repository::MasterDataGroupRepository,
            },
            items::{
                dto::{
                    CreateMasterDataItemRequest, ListMasterDataItemRequest,
                    MasterDataItemListResponse, MasterDataItemResponse,
                    UpdateMasterDataItemRequest,
                },
                entity::{MasterDataItem, MasterDataItemFilter},
                repository::MasterDataItemsRepository,
                services::MasterDataItemsService,
            },
        },
    },
    infrastructure::cache::CacheHelper,
};

const MASTER_DATA_GROUP_TTL: Duration = Duration::from_secs(300);
const MASTER_DATA_OPTIONS_TTL: Duration = Duration::from_secs(180);

pub struct DefaultMasterDataItemsService {
    repository: Arc<dyn MasterDataItemsRepository>,
    repository_group: Arc<dyn MasterDataGroupRepository>,
    cache: CacheHelper,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultMasterDataItemsService {
    pub fn new(
        repository: Arc<dyn MasterDataItemsRepository>,
        repository_group: Arc<dyn MasterDataGroupRepository>,
        cache: CacheHelper,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            repository_group,
            cache,
            audit_log_service,
        }
    }

    fn group_cache_key(code: &str) -> String {
        format!("master_data_group:code:{code}")
    }

    fn options_cache_key(group_code: &str, parent_id: Option<u64>, only_root: bool) -> String {
        let parent = parent_id
            .map(|p| p.to_string())
            .unwrap_or_else(|| "none".into());
        format!("master_data:options:{group_code}:{parent}:{only_root}")
    }

    fn map_item(&self, item: MasterDataItem) -> MasterDataItemResponse {
        MasterDataItemResponse {
            id: item.id,
            group_id: item.group_id,
            parent_id: item.parent_id,
            code: item.code,
            name: item.name,
            metadata: item.metadata,
            sort_order: item.sort_order,
            is_active: item.is_active,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }

    fn map_option(&self, item: MasterDataItem) -> MasterDataOptionResponse {
        MasterDataOptionResponse {
            id: item.id,
            code: item.code,
            name: item.name,
            parent_id: item.parent_id,
        }
    }

    async fn require_group(&self, code: &str) -> Result<MasterDataGroup> {
        let cache_key = Self::group_cache_key(code);

        if let Some(cached) = self.cache.get_json::<MasterDataGroup>(&cache_key).await {
            return Ok(cached);
        }

        let group = self
            .repository_group
            .find_group_by_code(code)
            .await?
            .ok_or_else(|| anyhow!("Master data group '{code}' not found"))?;

        self.cache
            .set_json(&cache_key, &group, Some(MASTER_DATA_GROUP_TTL))
            .await;

        Ok(group)
    }

    async fn invalidate_options(&self, group_code: &str) {
        self.cache
            .invalidate_prefix(&format!("master_data:options:{group_code}:"))
            .await;
    }

    async fn create_item_inner(
        &self,
        group: &MasterDataGroup,
        request: &CreateMasterDataItemRequest,
    ) -> Result<u64> {
        if self
            .repository
            .exists_item_code(group.id, &request.code)
            .await?
        {
            return Err(anyhow!("Item code already exists in this group"));
        }

        if !group.is_hierarchical && request.parent_id.is_some() {
            return Err(anyhow!(
                "Group '{}' is not hierarchical; parent_id is not allowed",
                group.code
            ));
        }

        let now = Utc::now();

        let item = MasterDataItem {
            id: 0,
            group_id: group.id,
            parent_id: request.parent_id,
            code: request.code.clone(),
            name: request.name.clone(),
            metadata: request.metadata.clone(),
            sort_order: request.sort_order.unwrap_or_default(),
            is_active: request.is_active.unwrap_or(true),
            created_at: now,
            updated_at: now,
        };

        self.repository.create_item(&item).await
    }

    async fn update_item_inner(
        &self,
        group: &MasterDataGroup,
        item_id: u64,
        request: &UpdateMasterDataItemRequest,
    ) -> Result<()> {
        let mut item = self
            .repository
            .find_item_by_id(item_id)
            .await?
            .filter(|i| i.group_id == group.id)
            .ok_or_else(|| anyhow!("Item not found in this group"))?;

        if !group.is_hierarchical && request.parent_id.is_some() {
            return Err(anyhow!(
                "Group '{}' is not hierarchical; parent_id is not allowed",
                group.code
            ));
        }

        item.parent_id = request.parent_id;
        item.name = request.name.clone().unwrap_or(item.name);
        item.metadata = request.metadata.clone().or(item.metadata);
        item.sort_order = request.sort_order.unwrap_or(item.sort_order);
        item.is_active = request.is_active.unwrap_or(item.is_active);
        item.updated_at = Utc::now();

        self.repository.update_item(&item).await
    }

    async fn delete_item_inner(&self, group: &MasterDataGroup, item_id: u64) -> Result<()> {
        let item = self
            .repository
            .find_item_by_id(item_id)
            .await?
            .filter(|i| i.group_id == group.id)
            .ok_or_else(|| anyhow!("Item not found in this group"))?;

        let child_count = self.repository.count_children(item.id).await?;
        if child_count > 0 {
            return Err(anyhow!(
                "Cannot delete item '{}': it still has {child_count} child item(s)",
                item.code
            ));
        }

        self.repository.delete_item(item.id).await
    }
}

#[async_trait]
impl MasterDataItemsService for DefaultMasterDataItemsService {
    async fn create_item(
        &self,
        group_code: &str,
        request: CreateMasterDataItemRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64> {
        let result = async {
            let group = self.require_group(group_code).await?;
            self.create_item_inner(&group, &request).await
        }
        .await;

        if result.is_ok() {
            self.invalidate_options(group_code).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::MASTER_DATA_ITEM_CREATED.to_string(),
                entity_type: Some("master_data_item".into()),
                entity_id: result.as_ref().ok().map(|id| id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "group_code": group_code,
                    "code": request.code,
                    "name": request.name,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn update_item(
        &self,
        group_code: &str,
        item_id: u64,
        request: UpdateMasterDataItemRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = async {
            let group = self.require_group(group_code).await?;
            self.update_item_inner(&group, item_id, &request).await
        }
        .await;

        if result.is_ok() {
            self.invalidate_options(group_code).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::MASTER_DATA_ITEM_UPDATED.to_string(),
                entity_type: Some("master_data_item".into()),
                entity_id: Some(item_id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "group_code": group_code,
                    "name": request.name,
                    "is_active": request.is_active,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn delete_item(
        &self,
        group_code: &str,
        item_id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = async {
            let group = self.require_group(group_code).await?;
            self.delete_item_inner(&group, item_id).await
        }
        .await;

        if result.is_ok() {
            self.invalidate_options(group_code).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::MASTER_DATA_ITEM_DELETED.to_string(),
                entity_type: Some("master_data_item".into()),
                entity_id: Some(item_id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: result.as_ref().err().map(
                    |e| serde_json::json!({ "group_code": group_code, "error": e.to_string() }),
                ),
            })
            .await;

        result
    }

    async fn find_item_by_id(
        &self,
        group_code: &str,
        item_id: u64,
    ) -> Result<MasterDataItemResponse> {
        let group = self.require_group(group_code).await?;

        let item = self
            .repository
            .find_item_by_id(item_id)
            .await?
            .filter(|i| i.group_id == group.id)
            .ok_or_else(|| anyhow!("Item not found in this group"))?;

        Ok(self.map_item(item))
    }

    async fn list_items(
        &self,
        group_code: &str,
        request: ListMasterDataItemRequest,
    ) -> Result<MasterDataItemListResponse> {
        let group = self.require_group(group_code).await?;

        let filter = MasterDataItemFilter {
            group_id: group.id,
            parent_id: request.parent_id,
            only_root: request.only_root.unwrap_or(false),
            search: request.search,
            is_active: request.is_active,
            page: request.page.unwrap_or(1),
            page_size: request.page_size.unwrap_or(10),
            sort_by: request.sort_by.unwrap_or_else(|| "sort_order".to_string()),
            sort_type: request.sort_type.unwrap_or_else(|| "ASC".to_string()),
        };

        let items = self.repository.list_items(&filter).await?;
        let total = self.repository.count_items(&filter).await?;

        let items = items.into_iter().map(|i| self.map_item(i)).collect();

        Ok(MasterDataItemListResponse {
            items,
            page: filter.page,
            page_size: filter.page_size,
            total,
        })
    }

    async fn list_options(
        &self,
        group_code: &str,
        parent_id: Option<u64>,
        only_root: bool,
    ) -> Result<MasterDataOptionListResponse> {
        let cache_key = Self::options_cache_key(group_code, parent_id, only_root);

        if let Some(cached) = self
            .cache
            .get_json::<MasterDataOptionListResponse>(&cache_key)
            .await
        {
            return Ok(cached);
        }

        let group = self.require_group(group_code).await?;

        let items = self
            .repository
            .list_options(group.id, parent_id, only_root)
            .await?;

        let response = MasterDataOptionListResponse {
            items: items.into_iter().map(|i| self.map_option(i)).collect(),
        };

        self.cache
            .set_json(&cache_key, &response, Some(MASTER_DATA_OPTIONS_TTL))
            .await;

        Ok(response)
    }
}
