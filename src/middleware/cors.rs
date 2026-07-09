use axum::http::{HeaderName, Method, header};
use std::str::FromStr;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::config::CorsConfig;

pub fn layer(cors_config: &CorsConfig) -> CorsLayer {
    // Parse allowed origins dari config
    let allow_origin: AllowOrigin = if cors_config.allow_origin == "*" {
        // Development only!
        AllowOrigin::any()
    } else {
        // Production: specific origins
        let origins: Vec<_> = cors_config
            .allow_origin
            .split(',')
            .filter_map(|origin| origin.trim().parse().ok())
            .collect();

        if origins.is_empty() {
            AllowOrigin::any()
        } else {
            AllowOrigin::list(origins)
        }
    };

    // Parse allowed methods
    let methods: Vec<Method> = cors_config
        .allow_methods
        .iter()
        .filter_map(|m| m.to_uppercase().parse().ok())
        .collect();

    // Parse allowed headers
    let headers: Vec<HeaderName> = cors_config
        .allow_headers
        .iter()
        .filter_map(|h| HeaderName::from_str(h).ok())
        .collect();

    let mut cors = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods(methods)
        .allow_headers(headers)
        .allow_credentials(cors_config.allow_credentials)
        .max_age(std::time::Duration::from_secs(3600));

    cors = cors.expose_headers([header::CONTENT_LENGTH, header::CONTENT_TYPE]);

    cors
}
