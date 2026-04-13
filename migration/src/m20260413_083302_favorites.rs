use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Favorites::Table)
                .if_not_exists()
                .col(ColumnDef::new(Favorites::UserId).integer().not_null())
                .col(ColumnDef::new(Favorites::ProductId).uuid().not_null())
                .col(ColumnDef::new(Favorites::CreatedAt).timestamp_with_time_zone())
                .primary_key(
                    Index::create()
                        .col(Favorites::UserId)
                        .col(Favorites::ProductId)
                        .primary(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_favorites_user_id")
                        .from(Favorites::Table, Favorites::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_favorites_product_id")
                        .from(Favorites::Table, Favorites::ProductId)
                        .to(Products::Table, Products::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Favorites::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Favorites {
    Table,
    UserId,
    ProductId,
    CreatedAt,
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
}

#[derive(Iden)]
pub enum Products {
    Table,
    Id,
}
