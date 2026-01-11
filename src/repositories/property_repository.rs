use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use crate::in_models::PropertyInput;
use entities::{price_snapshot, property_listing};

pub struct PropertyRepository;

impl PropertyRepository {
    pub async fn create(db: &DatabaseConnection, req: PropertyInput) -> Result<Uuid, DbErr> {
        let property_id = Uuid::new_v4();

        // -------------------------
        // Insert property listing
        // -------------------------
        let property = property_listing::ActiveModel {
            id: Set(property_id),
            external_id: Set(req.external_id),
            title: Set(req.title),
            description: Set(req.description),
            sqm: Set(req.sqm),
            city: Set(req.city),
            municipality: Set(req.municipality),
            market_type: Set(req.market_type.into()),
            listed_at: Set(req.listed_at),
            ..Default::default()
        };

        property.insert(db).await?;

        // -------------------------
        // Insert initial price snapshot
        // -------------------------
        let snapshot = price_snapshot::ActiveModel {
            id: Set(Uuid::new_v4()),
            property_id: Set(property_id),
            price_amount: Set(req.price_amount),
            currency: Set(req.currency),
            snapshot_at: Set(Utc::now()),
            ..Default::default()
        };

        snapshot.insert(db).await?;

        Ok(property_id)
    }
}
