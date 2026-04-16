use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(Categories::Table)
                .add_column(ColumnDef::new(Categories::Description).string())
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(Categories::Table)
                .drop_column(Categories::Description)
                .to_owned(),
        )
        .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Categories {
    Table,
    Description,
}
