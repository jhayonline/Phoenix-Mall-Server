use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(ColumnDef::new(Users::LastSeen).timestamp_with_time_zone())
                .add_column(ColumnDef::new(Users::IsOnline).boolean().default(false))
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::LastSeen)
                .drop_column(Users::IsOnline)
                .to_owned(),
        )
        .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    LastSeen,
    IsOnline,
}
