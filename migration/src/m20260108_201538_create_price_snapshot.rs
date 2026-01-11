use crate::m20260108_201259_create_property_listing::PropertyListing;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PriceSnapshot::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PriceSnapshot::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PriceSnapshot::PropertyId).uuid().not_null())
                    .col(
                        ColumnDef::new(PriceSnapshot::PriceAmount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PriceSnapshot::Currency).string().not_null())
                    .col(
                        ColumnDef::new(PriceSnapshot::SnapshotAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_property")
                            .from(PriceSnapshot::Table, PriceSnapshot::PropertyId)
                            .to(PropertyListing::Table, PropertyListing::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PriceSnapshot::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum PriceSnapshot {
    Table,
    Id,
    PropertyId,
    PriceAmount,
    Currency,
    SnapshotAt,
}
