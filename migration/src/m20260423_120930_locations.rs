use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create regions table
        m.create_table(
            Table::create()
                .table(Regions::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Regions::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Regions::Name).string().not_null().unique_key())
                .col(ColumnDef::new(Regions::DisplayOrder).integer().default(0))
                .col(ColumnDef::new(Regions::CreatedAt).timestamp_with_time_zone())
                .to_owned(),
        )
        .await?;

        // Create towns table
        m.create_table(
            Table::create()
                .table(Towns::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Towns::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Towns::RegionId).uuid().not_null())
                .col(ColumnDef::new(Towns::Name).string().not_null())
                .col(ColumnDef::new(Towns::DisplayOrder).integer().default(0))
                .col(ColumnDef::new(Towns::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_towns_region_id")
                        .from(Towns::Table, Towns::RegionId)
                        .to(Regions::Table, Regions::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // Create unique index for town names within a region
        m.create_index(
            Index::create()
                .name("idx_towns_region_name_unique")
                .table(Towns::Table)
                .col(Towns::RegionId)
                .col(Towns::Name)
                .unique()
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Towns::Table).to_owned())
            .await?;
        m.drop_table(Table::drop().table(Regions::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Regions {
    Table,
    Id,
    Name,
    DisplayOrder,
    CreatedAt,
}

#[derive(Iden)]
enum Towns {
    Table,
    Id,
    RegionId,
    Name,
    DisplayOrder,
    CreatedAt,
}
