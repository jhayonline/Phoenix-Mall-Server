use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Categories::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Categories::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Categories::Name).string().not_null())
                .col(
                    ColumnDef::new(Categories::Slug)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .col(ColumnDef::new(Categories::ParentId).uuid())
                .col(ColumnDef::new(Categories::Level).integer().not_null())
                .col(ColumnDef::new(Categories::DisplayOrder).integer())
                .col(ColumnDef::new(Categories::IsActive).boolean().default(true))
                .col(ColumnDef::new(Categories::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_categories_parent_id")
                        .from(Categories::Table, Categories::ParentId)
                        .to(Categories::Table, Categories::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Categories {
    Table,
    Id,
    Name,
    Slug,
    ParentId,
    Level,
    DisplayOrder,
    IsActive,
    CreatedAt,
}
