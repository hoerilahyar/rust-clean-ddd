use axum::Router;

use crate::{
    bootstrap::state::AppState,
    domain::{auth, role, user},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes::routes())
        .nest("/users", user::routes::routes())
        .nest("/roles", role::routes::router())
}
