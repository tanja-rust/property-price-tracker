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

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<(property_listing::Model, Option<price_snapshot::Model>)>, DbErr> {
        let property = property_listing::Entity::find_by_id(id).one(db).await?;

        if let Some(property) = property {
            let latest_price = price_snapshot::Entity::find()
                .filter(price_snapshot::Column::PropertyId.eq(property.id))
                .order_by_desc(price_snapshot::Column::SnapshotAt)
                .one(db)
                .await?;

            Ok(Some((property, latest_price)))
        } else {
            Ok(None)
        }
    }
}
