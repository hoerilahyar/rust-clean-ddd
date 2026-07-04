use crate::swagger::ApiDoc;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::bootstrap::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", crate::routes::api::routes())
        // .nest("/health", crate::routes::health::routes())
        .merge(SwaggerUi::new("/").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
