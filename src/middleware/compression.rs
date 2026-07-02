use tower_http::compression::CompressionLayer;

pub fn layer() -> CompressionLayer {
    CompressionLayer::new()
}
