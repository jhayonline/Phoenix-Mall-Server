use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create competitor_listings table
        m.create_table(
            Table::create()
                .table(CompetitorListings::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(CompetitorListings::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(CompetitorListings::ProductTitle).string().not_null())
                .col(ColumnDef::new(CompetitorListings::Price).decimal().not_null())
                .col(ColumnDef::new(CompetitorListings::Condition).string())
                .col(ColumnDef::new(CompetitorListings::Platform).string().not_null())
                .col(ColumnDef::new(CompetitorListings::Location).string())
                .col(ColumnDef::new(CompetitorListings::Url).string().not_null())
                .col(ColumnDef::new(CompetitorListings::ScrapedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        )
        .await?;

        // Create price_recommendations table
        m.create_table(
            Table::create()
                .table(PriceRecommendations::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(PriceRecommendations::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(PriceRecommendations::ProductId).uuid().not_null())
                .col(ColumnDef::new(PriceRecommendations::MarketAvgPrice).decimal().not_null())
                .col(ColumnDef::new(PriceRecommendations::RecommendedPrice).decimal())
                .col(ColumnDef::new(PriceRecommendations::CompetitorCount).integer().not_null())
                .col(ColumnDef::new(PriceRecommendations::IsViewed).boolean().default(false))
                .col(ColumnDef::new(PriceRecommendations::CreatedAt).timestamp_with_time_zone().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_price_rec_product_id")
                        .from(PriceRecommendations::Table, PriceRecommendations::ProductId)
                        .to(Products::Table, Products::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(PriceRecommendations::Table).to_owned()).await?;
        m.drop_table(Table::drop().table(CompetitorListings::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum CompetitorListings {
    Table,
    Id,
    ProductTitle,
    Price,
    Condition,
    Platform,
    Location,
    Url,
    ScrapedAt,
}

#[derive(Iden)]
enum PriceRecommendations {
    Table,
    Id,
    ProductId,
    MarketAvgPrice,
    RecommendedPrice,
    CompetitorCount,
    IsViewed,
    CreatedAt,
}

#[derive(Iden)]
enum Products {
    Table,
    Id,
}
