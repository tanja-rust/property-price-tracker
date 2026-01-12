use crate::app::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::in_models::PropertyInput;
use crate::out_models::property_response::{PriceSnapshotResponse, PropertyResponse};
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

pub async fn get_property(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PropertyResponse>, StatusCode> {
    let result = PropertyRepository::find_by_id(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (property, price) = result.ok_or(StatusCode::NOT_FOUND)?;

    let response = PropertyResponse {
        id: property.id,
        external_id: property.external_id,
        title: property.title,
        description: property.description,
        sqm: property.sqm,
        city: property.city,
        municipality: property.municipality,
        market_type: property.market_type,
        listed_at: property.listed_at,
        latest_price: price.map(|p| PriceSnapshotResponse {
            amount: p.price_amount,
            currency: p.currency,
            snapshot_at: p.snapshot_at,
        }),
    };

    Ok(Json(response))
}
