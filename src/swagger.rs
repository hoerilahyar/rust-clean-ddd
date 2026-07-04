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
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Authorization", description = "Authorization endpoints"),
        (name = "User", description = "User management"),
        (name = "Role", description = "Role management"),
        (name = "Permission", description = "Permission management"),
        (name = "Role Permission", description = "Role permission management"),
        (name = "User Role", description = "User role management"),
    )
)]
pub struct ApiDoc;
