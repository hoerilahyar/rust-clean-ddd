use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AuditLog {
    pub id: u64,
    pub actor_id: Option<u64>,
    pub actor_email: Option<String>,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub status: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

/// Konstanta nama action, dipakai module lain (auth, rbac, user)
/// supaya penulisan action string konsisten dan tidak typo.
pub mod audit_action {
    pub const AUTH_LOGIN: &str = "auth.login";
    pub const AUTH_LOGOUT: &str = "auth.logout";
    pub const AUTH_LOGOUT_ALL: &str = "auth.logout_all";
    pub const AUTH_REFRESH_TOKEN: &str = "auth.refresh_token";
    pub const USER_CREATED: &str = "user.created";
    pub const USER_UPDATED: &str = "user.updated";
    pub const USER_DELETED: &str = "user.deleted";
    pub const RBAC_ROLE_ASSIGNED: &str = "rbac.role_assigned";
    pub const RBAC_ROLE_REVOKED: &str = "rbac.role_revoked";
    pub const RBAC_ROLE_DELETED: &str = "rbac.role_deleted";
    pub const RBAC_PERMISSION_UPDATED: &str = "rbac.permission_updated";
    pub const RBAC_PERMISSION_DELETED: &str = "rbac.permission_deleted";
    pub const MENU_CREATED: &str = "menu.menu_created";
    pub const MENU_UPDATED: &str = "menu.menu_updated";
    pub const MENU_DELETED: &str = "menu.menu_deleted";
    pub const MENU_PERMISSION_ASSIGNED: &str = "menu.permission.assigned";
    pub const MENU_PERMISSION_REVOKE_PERMISSION: &str = "menu.permission.revoke_permission";
    pub const MENU_PERMISSION_REVOKE_ALL: &str = "menu.permission.revoke_all_permission";
    pub const SETTING_UPSERTED: &str = "setting.upserted";
    pub const SETTING_ACTIVATED: &str = "setting.activated";
    pub const SETTING_DEACTIVATED: &str = "setting.deactivate";
    pub const SETTING_DELETED: &str = "setting.deleted";
    pub const USER_SETTING_UPSERTED: &str = "user_setting.upserted";
    pub const USER_SETTING_ACTIVATED: &str = "user_setting.activated";
    pub const USER_SETTING_DEACTIVATED: &str = "user_setting.deactivated";
    pub const USER_SETTING_DELETED: &str = "user_setting.deleted";
    pub const SESSION_REVOKED: &str = "session.revoked";
    pub const SESSION_REVOKED_OTHERS: &str = "session.revoked_others";
}

pub mod audit_status {
    pub const SUCCESS: &str = "success";
    pub const FAILED: &str = "failed";
}
