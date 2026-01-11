use crate::config::AppConfig;
use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

fn is_authorized(headers: &HeaderMap, config: &AppConfig) -> bool {
    headers
        .get("X-Secret-Header")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v == config.api_secret_key)
}

pub async fn check_secret_header(
    State(config): State<AppConfig>,
    req: Request<axum::body::Body>,
    next: Next,
) -> impl IntoResponse {
    if is_authorized(req.headers(), &config) {
        next.run(req).await
    } else {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}
