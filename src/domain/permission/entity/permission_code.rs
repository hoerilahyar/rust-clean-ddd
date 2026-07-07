#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionCode {
    AuthorizeMe,

    UserCreate,
    UserRead,
    UserUpdate,
    UserDelete,

    RoleCreate,
    RoleRead,
    RoleUpdate,
    RoleDelete,

    PermissionCreate,
    PermissionRead,
    PermissionUpdate,
    PermissionDelete,

    RolePermissionAssign,
    RolePermissionRevoke,
    RolePermissionRead,

    UserRoleAssign,
    UserRoleRevoke,
    UserRoleRead,

    AuditLogRead,

    MenuCreate,
    MenuRead,
    MenuUpdate,
    MenuDelete,

    MenuPermissionAssign,
    MenuPermissionRead,
    MenuPermissionRevoke,

    SystemSettingUpdate,
    SystemSettingDelete,
    SystemSettingRead,
}

impl PermissionCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorizeMe => "AUTHORIZE_ME",

            Self::UserCreate => "USER_CREATE",
            Self::UserRead => "USER_READ",
            Self::UserUpdate => "USER_UPDATE",
            Self::UserDelete => "USER_DELETE",

            Self::RoleCreate => "ROLE_CREATE",
            Self::RoleRead => "ROLE_READ",
            Self::RoleUpdate => "ROLE_UPDATE",
            Self::RoleDelete => "ROLE_DELETE",

            Self::PermissionCreate => "PERMISSION_CREATE",
            Self::PermissionRead => "PERMISSION_READ",
            Self::PermissionUpdate => "PERMISSION_UPDATE",
            Self::PermissionDelete => "PERMISSION_DELETE",

            Self::RolePermissionAssign => "ROLE_PERMISSION_ASSIGN",
            Self::RolePermissionRevoke => "ROLE_PERMISSION_REVOKE",
            Self::RolePermissionRead => "ROLE_PERMISSION_READ",

            Self::UserRoleAssign => "USER_ROLE_ASSIGN",
            Self::UserRoleRevoke => "USER_ROLE_REVOKE",
            Self::UserRoleRead => "USER_ROLE_READ",

            Self::AuditLogRead => "AUDIT_LOG_READ",

            Self::MenuCreate => "MENU_CREATE",
            Self::MenuRead => "MENU_READ",
            Self::MenuUpdate => "MENU_UPDATE",
            Self::MenuDelete => "MENU_DELETE",

            Self::MenuPermissionAssign => "MENU_PERMISSION_ASSIGN",
            Self::MenuPermissionRead => "MENU_PERMISSION_READ",
            Self::MenuPermissionRevoke => "MENU_PERMISSION_REVOKE",

            Self::SystemSettingUpdate => "SYSTEM_SETTING_UPDATE",
            Self::SystemSettingDelete => "SYSTEM_SETTING_DELETE",
            Self::SystemSettingRead => "SYSTEM_SETTING_READ",
        }
    }
}
