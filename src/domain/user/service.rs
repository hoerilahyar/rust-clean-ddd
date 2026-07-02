use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;

use crate::{
    domain::user::{
        dto::{
            CreateUserRequest, GetUserRequest, ListUserRequest, UpdateUserRequest,
            UserListResponse, UserResponse,
        },
        entity::{User, UserFilter},
        repository::UserRepository,
    },
    infrastructure::security::PasswordService,
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, request: CreateUserRequest) -> Result<u64>;

    async fn update(&self, id: u64, request: UpdateUserRequest) -> Result<()>;

    async fn delete(&self, id: u64) -> Result<()>;

    async fn find_by_id(&self, request: GetUserRequest) -> Result<UserResponse>;

    async fn list(&self, request: ListUserRequest) -> Result<UserListResponse>;
}

pub struct DefaultUserService {
    repository: Arc<dyn UserRepository>,
}

impl DefaultUserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    fn map_response(&self, user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            username: user.username,
            fullname: user.fullname,
            email: user.email,
            is_active: user.is_active,
            last_login_at: user.last_login_at,
            created_at: user.created_at,
        }
    }
}
#[async_trait]
impl UserService for DefaultUserService {
    async fn create(&self, request: CreateUserRequest) -> Result<u64> {
        if self.repository.exists_username(&request.username).await? {
            return Err(anyhow!("Username already exists"));
        }

        if self.repository.exists_email(&request.email).await? {
            return Err(anyhow!("Email already exists"));
        }

        let now = Utc::now();

        let entity = User {
            id: 0,
            username: request.username,
            fullname: request.fullname,
            email: request.email,
            password: PasswordService::hash(&request.password)?,
            is_active: request.is_active,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        };

        let id = self.repository.create(&entity).await?;

        Ok(id)
    }

    async fn update(&self, id: u64, request: UpdateUserRequest) -> Result<()> {
        let mut user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        if user.email != request.email && self.repository.exists_email(&request.email).await? {
            return Err(anyhow!("Email already exists"));
        }

        user.fullname = request.fullname;
        user.email = request.email;
        user.is_active = request.is_active;
        user.updated_at = Utc::now();

        self.repository.update(&user).await
    }

    async fn delete(&self, id: u64) -> Result<()> {
        self.repository.delete(id).await
    }

    async fn find_by_id(&self, request: GetUserRequest) -> Result<UserResponse> {
        let user = self
            .repository
            .find_by_id(request.id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        Ok(self.map_response(user))
    }

    async fn list(&self, request: ListUserRequest) -> Result<UserListResponse> {
        let filter = UserFilter {
            page: request.page.unwrap_or(1),
            page_size: request.page_size.unwrap_or(10),
            search: request.search,
            sort_by: request.sort_by.unwrap_or_else(|| "created_at".to_string()),
            sort_type: request.sort_type.unwrap_or_else(|| "DESC".to_string()),
        };

        let users = self.repository.list(&filter).await?;
        let total = self.repository.count(&filter).await?;

        let items = users.into_iter().map(|u| self.map_response(u)).collect();

        Ok(UserListResponse {
            items,
            page: filter.page,
            page_size: filter.page_size,
            total,
        })
    }
}
