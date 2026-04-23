use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create category_specs table
        m.create_table(
            Table::create()
                .table(CategorySpecs::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(CategorySpecs::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(CategorySpecs::CategoryId).uuid().not_null())
                .col(
                    ColumnDef::new(CategorySpecs::SpecName)
                        .string()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(CategorySpecs::SpecType)
                        .string()
                        .not_null(),
                )
                .col(ColumnDef::new(CategorySpecs::IsRequired).boolean().default(false))
                .col(ColumnDef::new(CategorySpecs::PresetOptions).json())
                .col(ColumnDef::new(CategorySpecs::ValidationRules).json())
                .col(ColumnDef::new(CategorySpecs::InputPlaceholder).string())
                .col(ColumnDef::new(CategorySpecs::HelperText).text())
                .col(ColumnDef::new(CategorySpecs::SortOrder).integer().default(0))
                .col(ColumnDef::new(CategorySpecs::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_category_specs_category_id")
                        .from(CategorySpecs::Table, CategorySpecs::CategoryId)
                        .to(Categories::Table, Categories::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // Create indexes
        m.create_index(
            Index::create()
                .name("idx_category_specs_category_id")
                .table(CategorySpecs::Table)
                .col(CategorySpecs::CategoryId)
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("idx_category_specs_sort_order")
                .table(CategorySpecs::Table)
                .col(CategorySpecs::SortOrder)
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(CategorySpecs::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum CategorySpecs {
    Table,
    Id,
    CategoryId,
    SpecName,
    SpecType,
    IsRequired,
    PresetOptions,
    ValidationRules,
    InputPlaceholder,
    HelperText,
    SortOrder,
    CreatedAt,
}

#[derive(Iden)]
enum Categories {
    Table,
    Id,
}
