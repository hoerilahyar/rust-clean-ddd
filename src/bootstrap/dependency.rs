use std::sync::Arc;

use anyhow::Result;

use crate::{
    bootstrap::state::{AppState, Infrastructure, Services},
    config::Config,
    domain::{
        audit_log::{repository::MySqlAuditLogRepository, services::DefaultAuditLogService},
        auth::{repository::mysql_repository::MySqlAuthRepository, services::DefaultAuthService},
        authorization::{
            repository::MySqlAuthorizationRepository, services::DefaultAuthorizationService,
        },
        menu_permissions::{
            repository::MySqlMenuPermissionRepository, services::DefaultMenuPermissionService,
        },
        menus::{repository::MySqlMenuRepository, services::DefaultMenuService},
        permission::{repository::MySqlPermissionRepository, services::DefaultPermissionService},
        role::{repository::MySqlRoleRepository, services::DefaultRoleService},
        role_permission::{
            repository::MySqlRolePermissionRepository, services::DefaultRolePermissionService,
        },
        session::services::DefaultSessionService,
        system_settings::{
            repository::MySqlSystemSettingRepository, services::DefaultSystemSettingService,
        },
        user_role::{repository::MySqlUserRoleRepository, services::DefaultUserRoleService},
        user_setting::{
            repository::MySqlUserSettingRepository, services::DefaultUserSettingService,
        },
    },
    infrastructure::{database, security::JwtService},
};

use crate::domain::user::{repository::MySqlUserRepository, services::DefaultUserService};

pub async fn build_state() -> Result<AppState> {
    let config = Config::load()?;

    let db = database::connect(&config).await?;

    // let redis = cache::redis::connect(&config).await?;

    // let storage = Arc::new(Uploader::new(&config));

    // repository
    let auth_repository = Arc::new(MySqlAuthRepository::new(db.clone()));
    let user_repository = Arc::new(MySqlUserRepository::new(Arc::new(db.clone())));
    let role_repo = Arc::new(MySqlRoleRepository::new(Arc::new(db.clone())));
    let permission_repo = Arc::new(MySqlPermissionRepository::new(Arc::new(db.clone())));
    let role_permission_repo = Arc::new(MySqlRolePermissionRepository::new(Arc::new(db.clone())));
    let user_role_repo = Arc::new(MySqlUserRoleRepository::new(Arc::new(db.clone())));
    let authorization_repo = Arc::new(MySqlAuthorizationRepository::new(Arc::new(db.clone())));
    let audit_log_repo = Arc::new(MySqlAuditLogRepository::new(db.clone()));
    let menu_repo = Arc::new(MySqlMenuRepository::new(Arc::new(db.clone())));
    let menu_permission_repo = Arc::new(MySqlMenuPermissionRepository::new(Arc::new(db.clone())));
    let system_setting_repo = Arc::new(MySqlSystemSettingRepository::new(Arc::new(db.clone())));
    let user_setting_repo = Arc::new(MySqlUserSettingRepository::new(Arc::new(db.clone())));
    let session_auth_repository = auth_repository.clone();

    // infrastructure service
    let jwt = Arc::new(JwtService::new(&config));

    // domain service
    let audit_log_service = Arc::new(DefaultAuditLogService::new(audit_log_repo));

    let auth_service = Arc::new(DefaultAuthService::new(
        auth_repository,
        jwt.clone(),
        audit_log_service.clone(),
    ));
    let user_service = Arc::new(DefaultUserService::new(
        user_repository,
        audit_log_service.clone(),
    ));
    let role_service = Arc::new(DefaultRoleService::new(
        role_repo,
        audit_log_service.clone(),
    ));
    let permission_service = Arc::new(DefaultPermissionService::new(
        permission_repo,
        audit_log_service.clone(),
    ));
    let role_permission_service = Arc::new(DefaultRolePermissionService::new(
        role_permission_repo,
        audit_log_service.clone(),
    ));
    let user_role_service = Arc::new(DefaultUserRoleService::new(
        user_role_repo,
        audit_log_service.clone(),
    ));
    let authorization_service = Arc::new(DefaultAuthorizationService::new(authorization_repo));
    let menu_service = Arc::new(DefaultMenuService::new(
        menu_repo,
        audit_log_service.clone(),
    ));
    let menu_permission_service = Arc::new(DefaultMenuPermissionService::new(
        menu_permission_repo,
        audit_log_service.clone(),
    ));
    let system_setting_service = Arc::new(DefaultSystemSettingService::new(
        system_setting_repo,
        audit_log_service.clone(),
    ));
    let user_setting_service = Arc::new(DefaultUserSettingService::new(
        user_setting_repo,
        audit_log_service.clone(),
    ));
    let session_service = Arc::new(DefaultSessionService::new(
        session_auth_repository,
        audit_log_service.clone(),
    ));

    let infra = Infrastructure {
        db,
        // redis,
        // storage,
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
            audit_logs: audit_log_service,
            menu: menu_service,
            menu_permissions: menu_permission_service,
            system_setting: system_setting_service,
            user_setting: user_setting_service,
            session: session_service,
        },
    })
}
