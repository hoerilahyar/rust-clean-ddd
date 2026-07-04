use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    bootstrap::state::AppState,
    domain::{user::handler, user_role},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create))
        .route("/", get(handler::list))
        .route("/{id}", get(handler::find_by_id))
        .route("/{id}", put(handler::update))
        .route("/{id}", delete(handler::delete))
        // Assignment Role
        .route("/{user_id}/roles", put(user_role::handler::assign))
        .route("/{user_id}/roles", get(user_role::handler::list))
        .route(
            "/{user_id}/roles/{role_id}",
            delete(user_role::handler::revoke),
        )
}
