#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionCode {
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
}

impl PermissionCode {
    pub fn as_str(&self) -> &'static str {
        match self {
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
        }
    }
}
