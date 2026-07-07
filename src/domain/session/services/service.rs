use anyhow::Result;
use async_trait::async_trait;

use crate::domain::session::dto::SessionResponse;

#[async_trait]
pub trait SessionService: Send + Sync {
    async fn list(
        &self,
        user_id: u64,
        current_device_id: Option<String>,
    ) -> Result<Vec<SessionResponse>>;

    async fn revoke(
        &self,
        user_id: u64,
        session_id: u64,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn revoke_others(
        &self,
        user_id: u64,
        current_device_id: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;
}
