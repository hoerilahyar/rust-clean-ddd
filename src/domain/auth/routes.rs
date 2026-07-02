use axum::{Router, routing::post};

use crate::{bootstrap::state::AppState, domain::auth::handler};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(handler::login))
        .route("/refresh", post(handler::refresh_token))
        .route("/logout", post(handler::logout))
        .route("/logout-all", post(handler::logout_all))
}
