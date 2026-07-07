use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;

use crate::domain::{
    audit_log::{
        entity::audit_action,
        services::{AuditLogService, RecordAuditLogInput},
    },
    menus::{
        dto::{
            CreateMenuRequest, GetMenuRequest, ListMenuRequest, MenuListResponse, MenuResponse,
            UpdateMenuRequest,
        },
        entity::{Menu, MenuFilter},
        repository::MenuRepository,
        services::MenuService,
    },
};

pub struct DefaultMenuService {
    repository: Arc<dyn MenuRepository>,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultMenuService {
    pub fn new(
        repository: Arc<dyn MenuRepository>,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            audit_log_service,
        }
    }

    fn map_response(&self, menu: Menu) -> MenuResponse {
        MenuResponse {
            id: menu.id,
            parent_id: menu.parent_id,
            name: menu.name,
            icon: menu.icon,
            path: menu.path,
            sort_order: menu.sort_order,
            is_active: menu.is_active,
            created_at: menu.created_at,
        }
    }

    async fn create_inner(&self, request: &CreateMenuRequest) -> Result<u64> {
        if self.repository.exists_name(&request.name).await? {
            return Err(anyhow!("Name code already exists"));
        }

        let now = Utc::now();

        let menu = Menu {
            id: 0,
            parent_id: request.parent_id.clone(),
            name: request.name.clone(),
            icon: request.icon.clone(),
            path: request.path.clone(),
            sort_order: request.sort_order.unwrap_or_default(),
            is_active: request.is_active.unwrap_or(true),
            created_at: now,
            updated_at: now,
        };

        self.repository.create(&menu).await
    }

    async fn update_inner(&self, id: u64, request: &UpdateMenuRequest) -> Result<()> {
        let mut menu = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("Menu not found"))?;

        menu.name = request.name.clone().unwrap_or(menu.name);
        menu.parent_id = request.parent_id.clone();
        menu.icon = request.icon.clone();
        menu.path = request.path.clone().unwrap_or(menu.path);
        menu.sort_order = request.sort_order.unwrap_or_default();
        menu.is_active = request.is_active.unwrap_or(menu.is_active);
        menu.updated_at = Utc::now();

        self.repository.update(&menu).await
    }
}

#[async_trait]
impl MenuService for DefaultMenuService {
    async fn create(
        &self,
        request: CreateMenuRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64> {
        let result = self.create_inner(&request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::MENU_CREATED.to_string(),
                entity_type: Some("menu".into()),
                entity_id: result.as_ref().ok().map(|id| id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "name": request.name,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn update(
        &self,
        id: u64,
        request: UpdateMenuRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.update_inner(id, &request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::MENU_UPDATED.to_string(),
                entity_type: Some("menu".into()),
                entity_id: Some(id.to_string()),
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

    async fn delete(
        &self,
        id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let result = self.repository.delete(id).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::MENU_DELETED.to_string(),
                entity_type: Some("menu".into()),
                entity_id: Some(id.to_string()),
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

    async fn find_by_id(&self, request: GetMenuRequest) -> Result<MenuResponse> {
        let menu = self
            .repository
            .find_by_id(request.id)
            .await?
            .ok_or_else(|| anyhow!("Menu not found"))?;

        Ok(self.map_response(menu))
    }

    async fn list(&self, request: ListMenuRequest) -> Result<MenuListResponse> {
        let filter = MenuFilter {
            page: request.page.unwrap_or(1),
            page_size: request.page_size.unwrap_or(10),
            search: request.search,
            sort_by: request.sort_by.unwrap_or_else(|| "created_at".to_string()),
            sort_type: request.sort_type.unwrap_or_else(|| "DESC".to_string()),
        };

        let menus = self.repository.list(&filter).await?;
        let total = self.repository.count(&filter).await?;

        let items = menus.into_iter().map(|r| self.map_response(r)).collect();

        Ok(MenuListResponse {
            items,
            page: filter.page,
            page_size: filter.page_size,
            total,
        })
    }
}
