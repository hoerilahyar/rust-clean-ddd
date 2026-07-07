use anyhow::Result;
use async_trait::async_trait;

use crate::domain::auth::dto::{
    LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse,
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(
        &self,
        request: LoginRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<LoginResponse>;

    async fn refresh_token(
        &self,
        request: RefreshTokenRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<RefreshTokenResponse>;

    async fn logout(
        &self,
        refresh_token: RefreshTokenRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn logout_all(
        &self,
        user_id: u64,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;
}
