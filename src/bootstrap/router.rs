use axum::Router;

use crate::bootstrap::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", crate::routes::api::routes())
        // .nest("/health", crate::routes::health::routes())
        .with_state(state)
}
