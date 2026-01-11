use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn get_cors_layer() -> CorsLayer {
    let origins = ["http://localhost:5173"]
        .into_iter()
        .map(|s| s.parse().expect("valid origin"))
        .collect::<Vec<_>>();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_http::cors::CorsLayer;

    #[test]
    fn test_get_cors_layer_returns_valid_layer() {
        let cors = get_cors_layer();
        let _typed: CorsLayer = cors;
    }
}
