use axum::{Json, extract::State, http::StatusCode};

use crate::app::state::AppState;

use crate::in_models::PropertyInput;
use crate::repositories::PropertyRepository;

use serde::Serialize;

#[derive(Serialize)]
pub struct PropertyOut<T> {
    pub data: T,
}

pub async fn create_property(
    State(state): State<AppState>,
    Json(payload): Json<PropertyInput>,
) -> Result<(StatusCode, Json<PropertyOut<String>>), (StatusCode, Json<PropertyOut<String>>)> {
    let id = PropertyRepository::create(&state.db, payload)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PropertyOut {
                    data: format!("DB error: {}", e),
                }),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(PropertyOut {
            data: id.to_string(),
        }),
    ))
}
