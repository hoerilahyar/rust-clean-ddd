use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::domain::auth::dto::{
    LoginRequest, LoginResponse, LoginUser, RefreshTokenRequest, RefreshTokenResponse,
};

use crate::domain::auth::entity::{AuthUser, Permission, RefreshToken, Role};

use crate::domain::auth::errors::AuthError;
use crate::domain::auth::repository::auth_repository::AuthRepository;

// use crate::infrastructure::security::claims::AccessClaims;
use crate::infrastructure::security::jwt::JwtService;
use crate::infrastructure::security::password::PasswordService;

use crate::infrastructure::security::TokenPair;

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

    async fn logout(&self, refresh_token: RefreshTokenRequest) -> Result<()>;

    async fn logout_all(&self, user_id: u64) -> Result<()>;
}

pub struct DefaultAuthService {
    repository: Arc<dyn AuthRepository>,
    jwt_service: Arc<JwtService>,
}

impl DefaultAuthService {
    pub fn new(repository: Arc<dyn AuthRepository>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            repository,
            jwt_service,
        }
    }

    async fn load_roles(&self, user_id: u64) -> Result<Vec<Role>> {
        self.repository.find_roles(user_id).await
    }

    async fn load_permissions(&self, role_ids: &[u64]) -> Result<Vec<Permission>> {
        if role_ids.is_empty() {
            return Ok(Vec::new());
        }

        self.repository.find_permissions(role_ids).await
    }

    // fn build_access_claims(&self, user: &AuthUser, roles: &[Role]) -> AccessClaims {
    //     AccessClaims {
    //         sub: user.id,
    //         username: user.username.clone(),
    //         roles: roles.iter().map(|r| r.name.clone()).collect(),
    //         iss: "your-app".to_string(),
    //         iat: Utc::now().timestamp() as usize,
    //         exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize,
    //     }
    // }

    fn map_login_response(
        &self,
        access_token: String,
        refresh_token: String,
        expires_in: u64,
        user: &AuthUser,
        roles: Vec<Role>,
        permissions: Vec<Permission>,
    ) -> LoginResponse {
        LoginResponse {
            token: TokenPair {
                access_token,
                refresh_token,
                token_type: "Bearer".to_string(),
                expires_in,
            },

            user: LoginUser {
                id: user.id,
                username: user.username.clone(),
                fullname: user.fullname.clone(),
                email: user.email.clone(),

                roles: roles.into_iter().map(|r| r.slug.clone()).collect(),

                permissions: permissions.into_iter().map(|p| p.slug.clone()).collect(),
            },
        }
    }

    fn map_refresh_response(
        &self,
        access_token: String,
        refresh_token: String,
        expires_in: u64,
    ) -> RefreshTokenResponse {
        RefreshTokenResponse {
            token: TokenPair {
                access_token,
                refresh_token,
                token_type: "Bearer".to_string(),
                expires_in,
            },
        }
    }

    fn deduplicate_permissions(&self, permissions: Vec<Permission>) -> Vec<Permission> {
        let mut unique = HashMap::<String, Permission>::new();

        for permission in permissions {
            unique.entry(permission.slug.clone()).or_insert(permission);
        }

        unique.into_values().collect()
    }

    fn role_ids(&self, roles: &[Role]) -> Vec<u64> {
        roles.iter().map(|r| r.id).collect()
    }

    // fn role_codes(&self, roles: &[Role]) -> HashSet<String> {
    //     roles.iter().map(|r| r.slug.clone()).collect()
    // }

    async fn verify_login(&self, request: &LoginRequest) -> Result<AuthUser> {
        let user = self
            .repository
            .find_by_username_or_email(&request.identity)
            .await?
            .ok_or_else(|| anyhow!(AuthError::InvalidCredential))?;

        if !user.is_active {
            return Err(anyhow!(AuthError::UserInactive));
        }

        let verified = PasswordService::verify(&request.password, &user.password);

        if !verified {
            return Err(anyhow!(AuthError::InvalidCredential));
        }

        Ok(user)
    }

    async fn issue_tokens(
        &self,
        user: &AuthUser,
        roles: &[Role],
        ip_address: Option<String>,
        device_id: String,
    ) -> Result<(String, String, u64)> {
        let role_names = roles
            .iter()
            .map(|r| r.slug.clone()) // atau r.name.clone(), sesuai desain Anda
            .collect::<Vec<_>>();

        let access_token =
            self.jwt_service
                .generate_access_token(user.id, &user.username, role_names)?;

        let refresh_token = self
            .jwt_service
            .generate_refresh_token(user.id, &device_id)?;

        let entity = RefreshToken {
            id: 0,
            user_id: user.id,

            device_id: device_id,

            ip_address,

            token: refresh_token.clone(),

            expired_at: Utc::now() + Duration::days(30),

            revoked_at: None,

            created_at: Utc::now(),
        };

        self.repository.insert_refresh_token(&entity).await?;

        Ok((
            access_token,
            refresh_token,
            self.jwt_service.access_token_expired(),
        ))
    }
}

#[async_trait]
impl AuthService for DefaultAuthService {
    async fn login(
        &self,
        request: LoginRequest,
        ip_address: Option<String>,
        _user_agent: Option<String>,
    ) -> Result<LoginResponse> {
        let user = self.verify_login(&request).await?;

        let roles = self.load_roles(user.id).await?;

        if roles.is_empty() {
            return Err(anyhow!(AuthError::RoleNotFound));
        }

        let role_ids = self.role_ids(&roles);

        let permissions = self.deduplicate_permissions(self.load_permissions(&role_ids).await?);

        let (access_token, refresh_token, expires_in) = self
            .issue_tokens(&user, &roles, ip_address, request.device_id.clone())
            .await?;

        self.repository.update_last_login(user.id).await?;

        Ok(self.map_login_response(
            access_token,
            refresh_token,
            expires_in,
            &user,
            roles,
            permissions,
        ))
    }

    async fn refresh_token(
        &self,
        request: RefreshTokenRequest,
        ip_address: Option<String>,
        _user_agent: Option<String>,
    ) -> Result<RefreshTokenResponse> {
        let stored = self
            .repository
            .find_refresh_token(request)
            .await?
            .ok_or_else(|| anyhow!(AuthError::InvalidRefreshToken))?;

        if stored.revoked_at.is_some() {
            return Err(anyhow!(AuthError::InvalidRefreshToken));
        }

        if stored.expired_at <= Utc::now() {
            self.repository.revoke_refresh_token(stored.id).await?;

            return Err(anyhow!(AuthError::RefreshTokenExpired));
        }

        let user = self
            .repository
            .find_by_id(stored.user_id)
            .await?
            .ok_or_else(|| anyhow!(AuthError::UserNotFound))?;

        if !user.is_active {
            return Err(anyhow!(AuthError::UserInactive));
        }

        let roles = self.load_roles(user.id).await?;

        if roles.is_empty() {
            return Err(anyhow!(AuthError::RoleNotFound));
        }

        // let role_ids = self.role_ids(&roles);

        // let permissions = self.deduplicate_permissions(self.load_permissions(&role_ids).await?);

        self.repository.revoke_refresh_token(stored.id).await?;

        let (access_token, new_refresh_token, expires_in) = self
            .issue_tokens(&user, &roles, ip_address, stored.device_id.clone())
            .await?;

        Ok(self.map_refresh_response(access_token, new_refresh_token, expires_in))
    }

    async fn logout(&self, refresh_token: RefreshTokenRequest) -> Result<()> {
        let stored = self.repository.find_refresh_token(refresh_token).await?;

        let Some(token) = stored else {
            return Ok(());
        };

        if !token.revoked_at.is_some() {
            self.repository.revoke_refresh_token(token.id).await?;
        }

        Ok(())
    }

    async fn logout_all(&self, user_id: u64) -> Result<()> {
        self.repository.revoke_all_refresh_tokens(user_id).await?;

        Ok(())
    }
}

impl DefaultAuthService {
    // async fn load_user_context(&self, user: &AuthUser) -> Result<(Vec<Role>, Vec<Permission>)> {
    //     let roles = self.load_roles(user.id).await?;

    //     if roles.is_empty() {
    //         return Err(anyhow!(AuthError::RoleNotFound));
    //     }

    //     let role_ids = self.role_ids(&roles);

    //     let permissions = self.deduplicate_permissions(self.load_permissions(&role_ids).await?);

    //     Ok((roles, permissions))
    // }

    // async fn rotate_refresh_token(
    //     &self,
    //     refresh_token: &RefreshToken,
    //     user: &AuthUser,
    //     ip_address: Option<String>,
    // ) -> Result<(String, String, u64)> {
    //     let (roles, _permissions) = self.load_user_context(user).await?;

    //     self.repository
    //         .revoke_refresh_token(refresh_token.id)
    //         .await?;

    //     self.issue_tokens(user, &roles, ip_address, refresh_token.device_id.clone())
    //         .await
    // }

    // async fn revoke_if_expired(&self, refresh_token: &RefreshToken) -> Result<bool> {
    //     if refresh_token.revoked_at.is_some() {
    //         return Ok(true);
    //     }

    //     if refresh_token.expired_at > Utc::now() {
    //         return Ok(false);
    //     }

    //     self.repository
    //         .revoke_refresh_token(refresh_token.id)
    //         .await?;

    //     Ok(true)
    // }

    // fn permission_codes(&self, permissions: &[Permission]) -> Vec<String> {
    //     permissions
    //         .iter()
    //         .map(|permission| permission.slug.clone())
    //         .collect()
    // }

    // fn role_codes_vec(&self, roles: &[Role]) -> Vec<String> {
    //     roles.iter().map(|role| role.slug.clone()).collect()
    // }
}
