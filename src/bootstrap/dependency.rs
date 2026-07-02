use std::sync::Arc;

use anyhow::Result;

use crate::{
    bootstrap::state::{AppState, Infrastructure, Services},
    config::Config,
    domain::{
        auth::{repository::mysql_repository::MySqlAuthRepository, service::DefaultAuthService},
        authorization::{
            repository::MySqlAuthorizationRepository, service::DefaultAuthorizationService,
        },
        permission::{repository::MySqlPermissionRepository, service::DefaultPermissionService},
        role::{repository::MySqlRoleRepository, service::DefaultRoleService},
        role_permission::{
            repository::MySqlRolePermissionRepository, service::DefaultRolePermissionService,
        },
        user_role::{repository::MySqlUserRoleRepository, service::DefaultUserRoleService},
    },
    infrastructure::{cache, database, security::JwtService, storage::Uploader},
};

use crate::domain::user::{repository::MySqlUserRepository, service::DefaultUserService};

pub async fn build_state() -> Result<AppState> {
    let config = Config::load()?;

    let db = database::connect(&config).await?;

    let redis = cache::redis::connect(&config).await?;

    let storage = Arc::new(Uploader::new(&config));

    // repository
    let auth_repository = Arc::new(MySqlAuthRepository::new(db.clone()));
    let user_repository = Arc::new(MySqlUserRepository::new(Arc::new(db.clone())));
    let role_repository = Arc::new(MySqlRoleRepository::new(Arc::new(db.clone())));
    let permission_repo = Arc::new(MySqlPermissionRepository::new(Arc::new(db.clone())));
    let role_permission_repo = Arc::new(MySqlRolePermissionRepository::new(Arc::new(db.clone())));
    let user_role_repo = Arc::new(MySqlUserRoleRepository::new(Arc::new(db.clone())));
    let authorization_repo = Arc::new(MySqlAuthorizationRepository::new(Arc::new(db.clone())));

    // infrastructure service
    let jwt = Arc::new(JwtService::new(&config));

    // domain service
    let auth_service = Arc::new(DefaultAuthService::new(auth_repository, jwt.clone()));
    let user_service = Arc::new(DefaultUserService::new(user_repository));
    let role_service = Arc::new(DefaultRoleService::new(role_repository));
    let permission_service = Arc::new(DefaultPermissionService::new(permission_repo));
    let role_permission_service = Arc::new(DefaultRolePermissionService::new(role_permission_repo));
    let user_role_service = Arc::new(DefaultUserRoleService::new(user_role_repo));
    let authorization_service = Arc::new(DefaultAuthorizationService::new(authorization_repo));

    let infra = Infrastructure {
        db,
        redis,
        storage,
        jwt,
    };

    Ok(AppState {
        config: Arc::new(config),

        infra,

        services: Services {
            auth: auth_service,
            user: user_service,
            role: role_service,
            permission: permission_service,
            role_permission: role_permission_service,
            user_role: user_role_service,
            authorization: authorization_service,
        },
    })
}
