pub use sea_orm_migration::prelude::*;

mod m20260108_201259_create_property_listing;
mod m20260108_201538_create_price_snapshot;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260108_201259_create_property_listing::Migration),
            Box::new(m20260108_201538_create_price_snapshot::Migration),
        ]
    }
}
