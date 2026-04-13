use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Products::Table)
                .if_not_exists()
                .col(ColumnDef::new(Products::Id).uuid().not_null().primary_key())
                .col(
                    ColumnDef::new(Products::Pid)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .col(ColumnDef::new(Products::Title).string().not_null())
                .col(ColumnDef::new(Products::Description).text())
                .col(ColumnDef::new(Products::Price).decimal().not_null())
                .col(ColumnDef::new(Products::Condition).string())
                .col(ColumnDef::new(Products::Location).string())
                .col(ColumnDef::new(Products::CategoryId).uuid())
                .col(ColumnDef::new(Products::SellerId).integer().not_null())
                .col(ColumnDef::new(Products::Status).string().default("active"))
                .col(
                    ColumnDef::new(Products::WhatsappContact)
                        .boolean()
                        .default(false),
                )
                .col(
                    ColumnDef::new(Products::PhoneContact)
                        .boolean()
                        .default(false),
                )
                .col(ColumnDef::new(Products::ViewsCount).integer().default(0))
                .col(ColumnDef::new(Products::ExpiresAt).timestamp_with_time_zone())
                .col(ColumnDef::new(Products::CreatedAt).timestamp_with_time_zone())
                .col(ColumnDef::new(Products::UpdatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_products_category_id")
                        .from(Products::Table, Products::CategoryId)
                        .to(Categories::Table, Categories::Id),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_products_seller_id")
                        .from(Products::Table, Products::SellerId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Products::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Products {
    Table,
    Id,
    Pid,
    Title,
    Description,
    Price,
    Condition,
    Location,
    CategoryId,
    SellerId,
    Status,
    WhatsappContact,
    PhoneContact,
    ViewsCount,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Categories {
    Table,
    Id,
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
}
