use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct PropertyResponse {
    pub id: Uuid,
    pub external_id: String,
    pub title: String,
    pub description: Option<String>,
    pub sqm: Option<i32>,
    pub city: String,
    pub municipality: Option<String>,
    pub market_type: String,
    pub listed_at: NaiveDate,
    pub latest_price: Option<PriceSnapshotResponse>,
}

#[derive(Serialize)]
pub struct PriceSnapshotResponse {
    pub amount: i64,
    pub currency: String,
    pub snapshot_at: DateTime<Utc>,
}
