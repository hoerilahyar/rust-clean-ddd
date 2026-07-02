use std::sync::Arc;

use redis::aio::ConnectionManager;
use sqlx::MySqlPool;

use crate::{
    config::Config,
    domain::{
        auth::service::AuthService, permission::service::PermissionService,
        role::service::RoleService, role_permission::service::RolePermissionService,
        user::service::UserService, user_role::service::UserRoleService,
    },
    infrastructure::{security::JwtService, storage::Uploader},
};

#[derive(Clone)]
pub struct Services {
    pub auth: Arc<dyn AuthService>,
    pub user: Arc<dyn UserService>,
    pub role: Arc<dyn RoleService>,
    pub permission: Arc<dyn PermissionService>,
    pub role_permission: Arc<dyn RolePermissionService>,
    pub user_role: Arc<dyn UserRoleService>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: MySqlPool,
    pub redis: ConnectionManager,
    pub storage: Arc<Uploader>,
    pub jwt: Arc<JwtService>,

    pub services: Services,
}
