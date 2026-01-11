use crate::app::{middleware::check_secret_header, state::AppState};
use crate::handlers::{
    handler_setup::{health_handler, root_handler},
    property_handler::create_property,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn build_router(app_state: AppState) -> Router {
    // -----------------------
    // Public routes (no middleware)
    // -----------------------
    let public_routes = Router::new()
        .route("/", get(root_handler))
        .route("/_health", get(health_handler));

    // -----------------------
    // Protected routes
    // -----------------------
    let protected_routes = Router::new()
        .route("/api/property", post(create_property))
        .layer(axum::middleware::from_fn_with_state(
            app_state.config.clone(),
            check_secret_header,
        ));

    public_routes.merge(protected_routes).with_state(app_state)
}
