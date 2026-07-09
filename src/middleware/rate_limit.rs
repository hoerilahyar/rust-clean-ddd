use axum::{extract::Request, middleware::Next, response::Response};

pub async fn rate_limit_middleware(request: Request, next: Next) -> Response {
    next.run(request).await
}
