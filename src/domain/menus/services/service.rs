use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::domain::menus::dto::{
    CreateMenuRequest, GetMenuRequest, ListMenuRequest, MenuListResponse, MenuResponse,
    UpdateMenuRequest,
};

#[async_trait]
pub trait MenuService: Send + Sync {
    async fn create(
        &self,
        request: CreateMenuRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64>;

    async fn update(
        &self,
        id: u64,
        request: UpdateMenuRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn delete(
        &self,
        id: u64,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn find_by_id(&self, request: GetMenuRequest) -> Result<MenuResponse>;

    async fn list(&self, request: ListMenuRequest) -> Result<MenuListResponse>;
}
