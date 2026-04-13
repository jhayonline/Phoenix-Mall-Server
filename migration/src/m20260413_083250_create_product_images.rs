use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(ProductImages::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductImages::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(ProductImages::ProductId).uuid().not_null())
                .col(ColumnDef::new(ProductImages::ImageUrl).text().not_null())
                .col(
                    ColumnDef::new(ProductImages::IsPrimary)
                        .boolean()
                        .default(false),
                )
                .col(ColumnDef::new(ProductImages::DisplayOrder).integer())
                .col(ColumnDef::new(ProductImages::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_product_images_product_id")
                        .from(ProductImages::Table, ProductImages::ProductId)
                        .to(Products::Table, Products::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(ProductImages::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum ProductImages {
    Table,
    Id,
    ProductId,
    ImageUrl,
    IsPrimary,
    DisplayOrder,
    CreatedAt,
}

#[derive(Iden)]
pub enum Products {
    Table,
    Id,
}
