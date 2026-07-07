use std::sync::Arc;

use sqlx::MySqlPool;

use crate::{
    config::Config,
    domain::{
        audit_log::services::AuditLogService,
        auth::services::AuthService,
        authorization::services::AuthorizationService,
        master_data::{
            groups::services::MasterDataGroupService, items::services::MasterDataItemsService,
        },
        menu_permissions::services::MenuPermissionService,
        menus::services::MenuService,
        permission::services::PermissionService,
        role::services::RoleService,
        role_permission::services::RolePermissionService,
        session::services::SessionService,
        system_settings::services::SystemSettingService,
        user::services::UserService,
        user_role::services::UserRoleService,
        user_setting::services::UserSettingService,
    },
    infrastructure::security::JwtService,
};

#[derive(Clone)]
pub struct Infrastructure {
    pub db: MySqlPool,
    // pub redis: ConnectionManager,
    // pub storage: Arc<Uploader>,
    pub jwt: Arc<JwtService>,
}

#[derive(Clone)]
pub struct Services {
    pub auth: Arc<dyn AuthService>,
    pub user: Arc<dyn UserService>,
    pub role: Arc<dyn RoleService>,
    pub permission: Arc<dyn PermissionService>,
    pub role_permission: Arc<dyn RolePermissionService>,
    pub user_role: Arc<dyn UserRoleService>,
    pub authorization: Arc<dyn AuthorizationService>,
    pub audit_logs: Arc<dyn AuditLogService>,
    pub menu: Arc<dyn MenuService>,
    pub menu_permissions: Arc<dyn MenuPermissionService>,
    pub system_setting: Arc<dyn SystemSettingService>,
    pub user_setting: Arc<dyn UserSettingService>,
    pub session: Arc<dyn SessionService>,
    pub master_group: Arc<dyn MasterDataGroupService>,
    pub master_items: Arc<dyn MasterDataItemsService>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub infra: Infrastructure,

    pub services: Services,
}
