use axum::Router;

use crate::bootstrap::state::AppState;

pub fn apply(router: Router<AppState>) -> Router<AppState> {
    router
}
