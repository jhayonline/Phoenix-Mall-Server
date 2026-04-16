use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Orders::Table)
                .if_not_exists()
                .col(ColumnDef::new(Orders::Id).uuid().not_null().primary_key())
                .col(
                    ColumnDef::new(Orders::OrderNumber)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .col(ColumnDef::new(Orders::UserId).integer().not_null())
                .col(ColumnDef::new(Orders::TotalAmount).decimal().not_null())
                .col(
                    ColumnDef::new(Orders::ShippingAmount)
                        .decimal()
                        .not_null()
                        .default(0),
                )
                .col(
                    ColumnDef::new(Orders::TaxAmount)
                        .decimal()
                        .not_null()
                        .default(0),
                )
                .col(
                    ColumnDef::new(Orders::Status)
                        .string()
                        .not_null()
                        .default("pending"),
                )
                .col(ColumnDef::new(Orders::ShippingAddress).text())
                .col(ColumnDef::new(Orders::PaymentMethod).string())
                .col(
                    ColumnDef::new(Orders::PaymentStatus)
                        .string()
                        .default("pending"),
                )
                .col(ColumnDef::new(Orders::CreatedAt).timestamp_with_time_zone())
                .col(ColumnDef::new(Orders::UpdatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_orders_user_id")
                        .from(Orders::Table, Orders::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Orders::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Orders {
    Table,
    Id,
    OrderNumber,
    UserId,
    TotalAmount,
    ShippingAmount,
    TaxAmount,
    Status,
    ShippingAddress,
    PaymentMethod,
    PaymentStatus,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
}
