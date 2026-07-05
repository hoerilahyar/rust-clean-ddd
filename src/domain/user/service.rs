use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;

use crate::{
    domain::{
        audit_log::{
            entity::audit_action,
            service::{AuditLogService, RecordAuditLogInput},
        },
        user::{
            dto::{
                CreateUserRequest, GetUserRequest, ListUserRequest, UpdateUserRequest,
                UserListResponse, UserResponse,
            },
            entity::{User, UserFilter},
            repository::UserRepository,
        },
    },
    infrastructure::security::PasswordService,
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

    async fn find_by_id(&self, request: GetUserRequest) -> Result<UserResponse>;

    async fn list(&self, request: ListUserRequest) -> Result<UserListResponse>;
}

pub struct DefaultUserService {
    repository: Arc<dyn UserRepository>,
    audit_log_service: Arc<dyn AuditLogService>,
}

impl DefaultUserService {
    pub fn new(
        repository: Arc<dyn UserRepository>,
        audit_log_service: Arc<dyn AuditLogService>,
    ) -> Self {
        Self {
            repository,
            audit_log_service,
        }
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

    async fn create_inner(&self, request: &CreateUserRequest) -> Result<u64> {
        if self.repository.exists_username(&request.username).await? {
            return Err(anyhow!("Username already exists"));
        }

        if self.repository.exists_email(&request.email).await? {
            return Err(anyhow!("Email already exists"));
        }

        let now = Utc::now();

        let entity = User {
            id: 0,
            username: request.username.clone(),
            fullname: request.fullname.clone(),
            email: request.email.clone(),
            password: PasswordService::hash(&request.password)?,
            is_active: request.is_active,
            last_login_at: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        let id = self.repository.create(&entity).await?;

        Ok(id)
    }

    async fn update_inner(&self, id: u64, request: &UpdateUserRequest) -> Result<()> {
        let mut user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        if user.email != request.email && self.repository.exists_email(&request.email).await? {
            return Err(anyhow!("Email already exists"));
        }

        user.fullname = request.fullname.clone();
        user.email = request.email.clone();
        user.is_active = request.is_active;
        user.updated_at = Utc::now();

        self.repository.update(&user).await
    }
}

#[async_trait]
impl UserService for DefaultUserService {
    async fn create(
        &self,
        request: CreateUserRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<u64> {
        let result = self.create_inner(&request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::USER_CREATED.to_string(),
                entity_type: Some("user".into()),
                entity_id: result.as_ref().ok().map(|id| id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "username": request.username,
                    "email": request.email,
                    "error": result.as_ref().err().map(|e| e.to_string()),
                })),
            })
            .await;

        result
    }

    async fn update(
        &self,
        id: u64,
        request: UpdateUserRequest,
        actor_id: Option<u64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let before = self.repository.find_by_id(id).await.ok().flatten();

        let result = self.update_inner(id, &request).await;

        self.audit_log_service
            .record(RecordAuditLogInput {
                actor_id,
                actor_email: None,
                action: audit_action::USER_UPDATED.to_string(),
                entity_type: Some("user".into()),
                entity_id: Some(id.to_string()),
                is_success: result.is_ok(),
                ip_address,
                user_agent,
                metadata: Some(serde_json::json!({
                    "before": before.map(|u| serde_json::json!({
                        "fullname": u.fullname,
                        "email": u.email,
                        "is_active": u.is_active,
                    })),
                    "after": {
                        "fullname": request.fullname,
                        "email": request.email,
                        "is_active": request.is_active,
                    },
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
                action: audit_action::USER_DELETED.to_string(),
                entity_type: Some("user".into()),
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
