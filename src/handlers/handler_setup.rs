use axum::{http::StatusCode, response::Json};

use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn root_handler() -> &'static str {
    "Hello, Property Tracker 🏠"
}

pub async fn health_handler() -> (StatusCode, Json<HealthResponse>) {
    (StatusCode::OK, Json(HealthResponse { status: "OK" }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_root_handler_returns_hello() {
        let result = root_handler().await;
        assert_eq!(result, "Hello, Property Tracker 🏠");
    }

    #[tokio::test]
    async fn test_health_handler_ok() {
        let (status, Json(body)) = health_handler().await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.status, "OK");
    }
}
