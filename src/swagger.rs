use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        crate::domain::auth::handler::login,
        crate::domain::auth::handler::refresh_token,
        crate::domain::auth::handler::logout,
        crate::domain::auth::handler::logout_all,

        // Authorization
        crate::domain::authorization::handler::me,

        // User
        crate::domain::user::handler::create,
        crate::domain::user::handler::update,
        crate::domain::user::handler::delete,
        crate::domain::user::handler::find_by_id,
        crate::domain::user::handler::list,

        // Role
        crate::domain::role::handler::create,
        crate::domain::role::handler::update,
        crate::domain::role::handler::delete,
        crate::domain::role::handler::find_by_id,
        crate::domain::role::handler::list,

        // Permission
        crate::domain::permission::handler::create,
        crate::domain::permission::handler::update,
        crate::domain::permission::handler::delete,
        crate::domain::permission::handler::find_by_id,
        crate::domain::permission::handler::list,

        // Role Permission
        crate::domain::role_permission::handler::assign,
        crate::domain::role_permission::handler::revoke,
        crate::domain::role_permission::handler::list,

        // User Role
        crate::domain::user_role::handler::assign,
        crate::domain::user_role::handler::revoke,
        crate::domain::user_role::handler::list,

        // Audit Log
        crate::domain::audit_log::handler::list_audit_logs,
        crate::domain::audit_log::handler::get_audit_log,

        // Menu
        crate::domain::menus::handler::create,
        crate::domain::menus::handler::update,
        crate::domain::menus::handler::delete,
        crate::domain::menus::handler::find_by_id,
        crate::domain::menus::handler::list,

        // Menu Permission
        crate::domain::menu_permissions::handler::assign,
        crate::domain::menu_permissions::handler::revoke,
        crate::domain::menu_permissions::handler::list,

        // System Setting
        crate::domain::system_settings::handler::upsert,
        crate::domain::system_settings::handler::set_active,
        crate::domain::system_settings::handler::delete,
        crate::domain::system_settings::handler::find_by_key,
        crate::domain::system_settings::handler::list,

        // User Setting
        crate::domain::user_setting::handler::upsert,
        crate::domain::user_setting::handler::set_active,
        crate::domain::user_setting::handler::delete,
        crate::domain::user_setting::handler::find_by_key,
        crate::domain::user_setting::handler::list,

        // Session
        crate::domain::session::handler::list,
        crate::domain::session::handler::revoke,
        crate::domain::session::handler::revoke_others,

    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Authorization", description = "Authorization endpoints"),
        (name = "User", description = "User management"),
        (name = "Role", description = "Role management"),
        (name = "Permission", description = "Permission management"),
        (name = "Role Permission", description = "Role permission management"),
        (name = "User Role", description = "User role management"),
        (name = "Audit Log", description = "Audit log"),
        (name = "Menu", description = "Menu management"),
        (name = "Menu Permission", description = "Menu permission"),
        (name = "System Setting", description = "System Setting"),
        (name = "User Setting", description = "User Setting"),
        (name = "Session", description = "Session"),
    )
)]
pub struct ApiDoc;
