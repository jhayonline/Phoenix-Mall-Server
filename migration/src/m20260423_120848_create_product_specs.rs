use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(ProductSpecs::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductSpecs::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(ProductSpecs::ProductId).uuid().not_null())
                .col(ColumnDef::new(ProductSpecs::SpecId).uuid().not_null())
                .col(ColumnDef::new(ProductSpecs::SpecValue).text().not_null())
                .col(ColumnDef::new(ProductSpecs::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_product_specs_product_id")
                        .from(ProductSpecs::Table, ProductSpecs::ProductId)
                        .to(Products::Table, Products::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_product_specs_spec_id")
                        .from(ProductSpecs::Table, ProductSpecs::SpecId)
                        .to(CategorySpecs::Table, CategorySpecs::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // Create unique index to prevent duplicate specs per product
        m.create_index(
            Index::create()
                .name("idx_product_specs_unique")
                .table(ProductSpecs::Table)
                .col(ProductSpecs::ProductId)
                .col(ProductSpecs::SpecId)
                .unique()
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(ProductSpecs::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum ProductSpecs {
    Table,
    Id,
    ProductId,
    SpecId,
    SpecValue,
    CreatedAt,
}

#[derive(Iden)]
enum Products {
    Table,
    Id,
}

#[derive(Iden)]
enum CategorySpecs {
    Table,
    Id,
}
