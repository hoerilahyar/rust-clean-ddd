use axum::Router;

use crate::{
    bootstrap::state::AppState,
    domain::{
        api_key, audit_log, auth, authorization, master_data, menus, permission, role, session,
        system_settings, user, user_role, user_setting,
    },
};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/health",
            axum::routing::get(crate::routes::health::health_check),
        )
        .nest("/auth", auth::routes::router())
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user::routes::router())
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
        .nest("/system-settings", system_settings::routes::router())
        .nest("/user-settings", user_setting::routes::router())
        .nest("/sessions", session::routes::router())
        .nest("/master", master_data::routes::router())
        .nest("/api-keys", api_key::routes::router())
}
