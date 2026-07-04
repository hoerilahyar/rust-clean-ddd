use axum::Router;

use crate::{
    bootstrap::state::AppState,
    domain::{auth, authorization, role, user},
};

pub fn public_routes() -> Router<AppState> {
    Router::new().nest("/auth", auth::routes::routes())
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user::routes::routes())
        .nest("/roles", role::routes::router())
        .nest("/authorize", authorization::routes::router())
}
