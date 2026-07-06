use axum::Router;

use crate::{
    bootstrap::state::AppState,
    domain::{audit_log, auth, authorization, menus, permission, role, user, user_role},
};

pub fn public_routes() -> Router<AppState> {
    Router::new().nest("/auth", auth::routes::routes())
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user::routes::routes())
        .nest(
            "/iam",
            Router::new()
                .nest("/roles", role::routes::router())
                .nest("/permissions", permission::routes::router())
                .nest("/menus", menus::routes::router())
                .nest("/authorization", authorization::routes::router())
                .nest("/users", user_role::routes::router()),
        )
        .nest("/audit-logs", audit_log::routes::router())
}
