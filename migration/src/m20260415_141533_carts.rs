use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Carts::Table)
                .if_not_exists()
                .col(ColumnDef::new(Carts::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(Carts::UserId).integer().not_null())
                .col(ColumnDef::new(Carts::ProductId).uuid().not_null())
                .col(
                    ColumnDef::new(Carts::Quantity)
                        .integer()
                        .not_null()
                        .default(1),
                )
                .col(ColumnDef::new(Carts::CreatedAt).timestamp_with_time_zone())
                .col(ColumnDef::new(Carts::UpdatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_carts_user_id")
                        .from(Carts::Table, Carts::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_carts_product_id")
                        .from(Carts::Table, Carts::ProductId)
                        .to(Products::Table, Products::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Carts::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Carts {
    Table,
    Id,
    UserId,
    ProductId,
    Quantity,
    CreatedAt,
    UpdatedAt,
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
