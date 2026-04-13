use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(ColumnDef::new(Users::PhoneNumber).string())
                .add_column(ColumnDef::new(Users::AvatarUrl).string())
                .add_column(ColumnDef::new(Users::Location).string())
                .add_column(
                    ColumnDef::new(Users::WhatsappEnabled)
                        .boolean()
                        .default(false),
                )
                .add_column(ColumnDef::new(Users::PhoneEnabled).boolean().default(false))
                .add_column(ColumnDef::new(Users::Role).string().default("user"))
                .add_column(ColumnDef::new(Users::IsActive).boolean().default(true))
                .add_column(ColumnDef::new(Users::LastActive).timestamp_with_time_zone())
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::PhoneNumber)
                .drop_column(Users::AvatarUrl)
                .drop_column(Users::Location)
                .drop_column(Users::WhatsappEnabled)
                .drop_column(Users::PhoneEnabled)
                .drop_column(Users::Role)
                .drop_column(Users::IsActive)
                .drop_column(Users::LastActive)
                .to_owned(),
        )
        .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Users {
    Table,
    PhoneNumber,
    AvatarUrl,
    Location,
    WhatsappEnabled,
    PhoneEnabled,
    Role,
    IsActive,
    LastActive,
}
