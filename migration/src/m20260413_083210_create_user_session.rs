use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(UserSessions::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserSessions::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(UserSessions::UserId).integer().not_null())
                .col(
                    ColumnDef::new(UserSessions::TokenHash)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .col(ColumnDef::new(UserSessions::DeviceName).string())
                .col(ColumnDef::new(UserSessions::IpAddress).inet())
                .col(ColumnDef::new(UserSessions::UserAgent).text())
                .col(
                    ColumnDef::new(UserSessions::ExpiresAt)
                        .timestamp_with_time_zone()
                        .not_null(),
                )
                .col(ColumnDef::new(UserSessions::LastActivityAt).timestamp_with_time_zone())
                .col(ColumnDef::new(UserSessions::PushToken).string())
                .col(
                    ColumnDef::new(UserSessions::IsActive)
                        .boolean()
                        .default(true),
                )
                .col(ColumnDef::new(UserSessions::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_user_sessions_user_id")
                        .from(UserSessions::Table, UserSessions::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(UserSessions::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum UserSessions {
    Table,
    Id,
    UserId,
    TokenHash,
    DeviceName,
    IpAddress,
    UserAgent,
    ExpiresAt,
    LastActivityAt,
    PushToken,
    IsActive,
    CreatedAt,
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
}
