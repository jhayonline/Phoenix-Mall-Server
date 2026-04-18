use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create reviews table
        m.create_table(
            Table::create()
                .table(ProductReviews::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductReviews::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(ProductReviews::ProductId).uuid().not_null())
                .col(ColumnDef::new(ProductReviews::UserId).integer().not_null())
                .col(ColumnDef::new(ProductReviews::Rating).integer().not_null())
                .col(ColumnDef::new(ProductReviews::Comment).text())
                .col(ColumnDef::new(ProductReviews::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_reviews_product_id")
                        .from(ProductReviews::Table, ProductReviews::ProductId)
                        .to(Products::Table, Products::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_reviews_user_id")
                        .from(ProductReviews::Table, ProductReviews::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // Create product_views table for tracking views
        m.create_table(
            Table::create()
                .table(ProductViews::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductViews::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(ProductViews::ProductId).uuid().not_null())
                .col(ColumnDef::new(ProductViews::UserId).integer())
                .col(ColumnDef::new(ProductViews::IpAddress).string())
                .col(ColumnDef::new(ProductViews::ViewedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_views_product_id")
                        .from(ProductViews::Table, ProductViews::ProductId)
                        .to(Products::Table, Products::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // Add rating columns to products table for caching
        m.alter_table(
            Table::alter()
                .table(Products::Table)
                .add_column(
                    ColumnDef::new(Products::AverageRating)
                        .double()
                        .default(0.0),
                )
                .add_column(ColumnDef::new(Products::TotalReviews).integer().default(0))
                .add_column(ColumnDef::new(Products::WishlistCount).integer().default(0))
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(ProductReviews::Table).to_owned())
            .await?;
        m.drop_table(Table::drop().table(ProductViews::Table).to_owned())
            .await?;
        m.alter_table(
            Table::alter()
                .table(Products::Table)
                .drop_column(Products::AverageRating)
                .drop_column(Products::TotalReviews)
                .drop_column(Products::WishlistCount)
                .to_owned(),
        )
        .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum ProductReviews {
    Table,
    Id,
    ProductId,
    UserId,
    Rating,
    Comment,
    CreatedAt,
}

#[derive(Iden)]
enum ProductViews {
    Table,
    Id,
    ProductId,
    UserId,
    IpAddress,
    ViewedAt,
}

#[derive(Iden)]
enum Products {
    Table,
    Id,
    AverageRating,
    TotalReviews,
    WishlistCount,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
