use anyhow::Result;
use async_trait::async_trait;

use crate::domain::user::dto::{
    ChangePasswordRequest, CreateUserRequest, GetUserRequest, ListUserRequest, UpdateUserRequest,
    UserListResponse, UserResponse,
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(
        &self,
        request: CreateUserRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64>;

    async fn update(
        &self,
        id: u64,
        request: UpdateUserRequest,
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

    async fn change_password(
        &self,
        user_id: u64,
        request: ChangePasswordRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()>;

    async fn find_by_id(&self, request: GetUserRequest) -> Result<UserResponse>;

    async fn list(&self, request: ListUserRequest) -> Result<UserListResponse>;
}
