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
                dto::{
                    CreateMasterDataGroupRequest, ListMasterDataGroupRequest,
                    MasterDataGroupListResponse, MasterDataGroupResponse,
                    UpdateMasterDataGroupRequest,
                },
                entity::{MasterDataGroup, MasterDataGroupFilter},
                repository::MasterDataGroupRepository,
                services::MasterDataGroupService,
            },
            items::repository::MasterDataItemsRepository,
        },
    },
    infrastructure::cache::CacheHelper,
};

const MASTER_DATA_GROUP_TTL: Duration = Duration::from_secs(300);

pub struct DefaultMasterDataGroupService {
    repository: Arc<dyn MasterDataGroupRepository>,
    item_repository: Arc<dyn MasterDataItemsRepository>,
    cache: CacheHelper,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultMasterDataGroupService {
    pub fn new(
        repository: Arc<dyn MasterDataGroupRepository>,
        item_repository: Arc<dyn MasterDataItemsRepository>,
        cache: CacheHelper,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            audit_log_service,
            cache,
            item_repository,
        }
    }

    fn group_cache_key(code: &str) -> String {
        format!("master_data_group:code:{code}")
    }

    fn map_group(&self, group: MasterDataGroup) -> MasterDataGroupResponse {
        MasterDataGroupResponse {
            id: group.id,
            code: group.code,
            name: group.name,
            description: group.description,
            is_hierarchical: group.is_hierarchical,
            is_active: group.is_active,
            created_at: group.created_at,
            updated_at: group.updated_at,
        }
    }

    async fn require_group(&self, code: &str) -> Result<MasterDataGroup> {
        let cache_key = Self::group_cache_key(code);

        if let Some(cached) = self.cache.get_json::<MasterDataGroup>(&cache_key).await {
            return Ok(cached);
        }

        let group = self
            .repository
            .find_group_by_code(code)
            .await?
            .ok_or_else(|| anyhow!("Master data group '{code}' not found"))?;

        self.cache
            .set_json(&cache_key, &group, Some(MASTER_DATA_GROUP_TTL))
            .await;

        Ok(group)
    }

    async fn create_group_inner(&self, request: &CreateMasterDataGroupRequest) -> Result<u64> {
        if self.repository.exists_group_code(&request.code).await? {
            return Err(anyhow!("Group code already exists"));
        }

        let now = Utc::now();

        let group = MasterDataGroup {
            id: 0,
            code: request.code.clone(),
            name: request.name.clone(),
            description: request.description.clone(),
            is_hierarchical: request.is_hierarchical.unwrap_or(false),
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.repository.create_group(&group).await
    }

    async fn update_group_inner(
        &self,
        code: &str,
        request: &UpdateMasterDataGroupRequest,
    ) -> Result<()> {
        let mut group = self.require_group(code).await?;

        group.name = request.name.clone().unwrap_or(group.name);
        group.description = request.description.clone().or(group.description);
        group.is_hierarchical = request.is_hierarchical.unwrap_or(group.is_hierarchical);
        group.is_active = request.is_active.unwrap_or(group.is_active);
        group.updated_at = Utc::now();

        self.repository.update_group(&group).await
    }

    async fn delete_group_inner(&self, code: &str) -> Result<()> {
        let group = self.require_group(code).await?;

        let item_count = self.item_repository.count_items_in_group(group.id).await?;
        if item_count > 0 {
            return Err(anyhow!(
                "Cannot delete group '{code}': it still has {item_count} item(s)"
            ));
        }

        self.repository.delete_group(group.id).await
    }
}

#[async_trait]
impl MasterDataGroupService for DefaultMasterDataGroupService {
    async fn create_group(
        &self,
        request: CreateMasterDataGroupRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64> {
        let result = self.create_group_inner(&request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::MASTER_DATA_GROUP_CREATED.to_string(),
                entity_type: Some("master_data_group".into()),
                entity_id: result.as_ref().ok().map(|id| id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "code": request.code,
                    "name": request.name,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn update_group(
        &self,
        code: &str,
        request: UpdateMasterDataGroupRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.update_group_inner(code, &request).await;

        if result.is_ok() {
            self.cache.invalidate(&Self::group_cache_key(code)).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::MASTER_DATA_GROUP_UPDATED.to_string(),
                entity_type: Some("master_data_group".into()),
                entity_id: Some(code.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "name": request.name,
                    "is_active": request.is_active,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn delete_group(
        &self,
        code: &str,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.delete_group_inner(code).await;

        if result.is_ok() {
            self.cache.invalidate(&Self::group_cache_key(code)).await;
        }

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::MASTER_DATA_GROUP_DELETED.to_string(),
                entity_type: Some("master_data_group".into()),
                entity_id: Some(code.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: result
                    .as_ref()
                    .err()
                    .map(|e| serde_json::json!({ "error": e.to_string() })),
            })
            .await;

        result
    }

    async fn find_group_by_code(&self, code: &str) -> Result<MasterDataGroupResponse> {
        let group = self.require_group(code).await?;
        Ok(self.map_group(group))
    }

    async fn list_groups(
        &self,
        request: ListMasterDataGroupRequest,
    ) -> Result<MasterDataGroupListResponse> {
        let filter = MasterDataGroupFilter {
            page: request.page.unwrap_or(1),
            page_size: request.page_size.unwrap_or(10),
            search: request.search,
            sort_by: request.sort_by.unwrap_or_else(|| "created_at".to_string()),
            sort_type: request.sort_type.unwrap_or_else(|| "DESC".to_string()),
        };

        let groups = self.repository.list_groups(&filter).await?;
        let total = self.repository.count_groups(&filter).await?;

        let items = groups.into_iter().map(|g| self.map_group(g)).collect();

        Ok(MasterDataGroupListResponse {
            items,
            page: filter.page,
            page_size: filter.page_size,
            total,
        })
    }
}
