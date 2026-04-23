use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(Products::Table)
                .add_column(ColumnDef::new(Products::RegionId).uuid())
                .add_column(ColumnDef::new(Products::TownId).uuid())
                .add_column(
                    ColumnDef::new(Products::Negotiation)
                        .string()
                        .default("negotiable"),
                )
                .add_column(
                    ColumnDef::new(Products::PromotionType)
                        .string()
                        .default("standard"),
                )
                .add_column(
                    ColumnDef::new(Products::PromotionExpiresAt).timestamp_with_time_zone(),
                )
                .add_foreign_key(
                    TableForeignKey::new()
                        .name("fk_products_region_id")
                        .from_tbl(Products::Table)
                        .from_col(Products::RegionId)
                        .to_tbl(Regions::Table)
                        .to_col(Regions::Id)
                        .on_delete(ForeignKeyAction::SetNull),
                )
                .add_foreign_key(
                    TableForeignKey::new()
                        .name("fk_products_town_id")
                        .from_tbl(Products::Table)
                        .from_col(Products::TownId)
                        .to_tbl(Towns::Table)
                        .to_col(Towns::Id)
                        .on_delete(ForeignKeyAction::SetNull),
                )
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(Products::Table)
                .drop_column(Products::RegionId)
                .drop_column(Products::TownId)
                .drop_column(Products::Negotiation)
                .drop_column(Products::PromotionType)
                .drop_column(Products::PromotionExpiresAt)
                .to_owned(),
        )
        .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Products {
    Table,
    RegionId,
    TownId,
    Negotiation,
    PromotionType,
    PromotionExpiresAt,
}

#[derive(Iden)]
enum Regions {
    Table,
    Id,
}

#[derive(Iden)]
enum Towns {
    Table,
    Id,
}
