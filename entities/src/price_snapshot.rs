use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "price_snapshot")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub property_id: Uuid,

    pub price_amount: i64,
    pub currency: String,

    pub snapshot_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    PropertyListing,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::PropertyListing => Entity::belongs_to(super::property_listing::Entity)
                .from(Column::PropertyId)
                .to(super::property_listing::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

impl Related<super::property_listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PropertyListing.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
