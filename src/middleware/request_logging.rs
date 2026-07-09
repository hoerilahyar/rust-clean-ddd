use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn log_requests(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status();

    tracing::info!(
        "HTTP {} {} -> {} ({:.2}ms)",
        method,
        uri,
        status,
        duration.as_secs_f64() * 1000.0
    );

    response
}
