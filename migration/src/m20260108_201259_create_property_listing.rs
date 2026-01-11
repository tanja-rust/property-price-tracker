use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PropertyListing::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PropertyListing::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PropertyListing::ExternalId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PropertyListing::Title).string().not_null())
                    .col(ColumnDef::new(PropertyListing::Description).string())
                    .col(ColumnDef::new(PropertyListing::Sqm).integer())
                    .col(ColumnDef::new(PropertyListing::City).string().not_null())
                    .col(ColumnDef::new(PropertyListing::Municipality).string())
                    .col(
                        ColumnDef::new(PropertyListing::MarketType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PropertyListing::ListedAt).date().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PropertyListing::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PropertyListing {
    Table,
    Id,
    ExternalId,
    Title,
    Description,
    Sqm,
    City,
    Municipality,
    MarketType,
    ListedAt,
}
