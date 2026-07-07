use axum::Router;

use crate::{
    bootstrap::state::AppState,
    domain::{master_data::groups, master_data::items},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/groups", groups::routes::router())
        .nest("/items", items::routes::router())
}
