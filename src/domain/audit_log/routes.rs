use axum::{Router, routing::get};

use crate::{
    bootstrap::state::AppState,
    domain::audit_log::handler::{get_audit_log, list_audit_logs},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_audit_logs))
        .route("/{id}", get(get_audit_log))
}
