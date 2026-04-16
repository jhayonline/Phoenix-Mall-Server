use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(OrderItems::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(OrderItems::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(OrderItems::OrderId).uuid().not_null())
                .col(ColumnDef::new(OrderItems::ProductId).uuid().not_null())
                .col(ColumnDef::new(OrderItems::ProductName).string().not_null())
                .col(ColumnDef::new(OrderItems::Quantity).integer().not_null())
                .col(ColumnDef::new(OrderItems::Price).decimal().not_null())
                .col(ColumnDef::new(OrderItems::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_order_items_order_id")
                        .from(OrderItems::Table, OrderItems::OrderId)
                        .to(Orders::Table, Orders::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_order_items_product_id")
                        .from(OrderItems::Table, OrderItems::ProductId)
                        .to(Products::Table, Products::Id),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(OrderItems::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum OrderItems {
    Table,
    Id,
    OrderId,
    ProductId,
    ProductName,
    Quantity,
    Price,
    CreatedAt,
}

#[derive(Iden)]
pub enum Orders {
    Table,
    Id,
}

#[derive(Iden)]
pub enum Products {
    Table,
    Id,
}
