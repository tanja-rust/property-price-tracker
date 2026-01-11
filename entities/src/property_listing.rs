use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "property_listing")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub external_id: String,

    pub title: String,
    pub description: Option<String>,

    pub sqm: Option<i32>,

    pub city: String,
    pub municipality: Option<String>,

    /// "New" | "Resale"
    pub market_type: String,

    pub listed_at: Date,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    PriceSnapshot,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::PriceSnapshot => Entity::has_many(super::price_snapshot::Entity)
                .from(Column::Id)
                .to(super::price_snapshot::Column::PropertyId)
                .into(),
        }
    }
}

impl Related<super::price_snapshot::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PriceSnapshot.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
