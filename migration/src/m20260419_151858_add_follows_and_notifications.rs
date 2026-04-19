use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create follows table
        m.create_table(
            Table::create()
                .table(Follows::Table)
                .if_not_exists()
                .col(ColumnDef::new(Follows::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(Follows::FollowerId).integer().not_null())
                .col(ColumnDef::new(Follows::FollowingId).integer().not_null())
                .col(ColumnDef::new(Follows::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_follows_follower_id")
                        .from(Follows::Table, Follows::FollowerId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_follows_following_id")
                        .from(Follows::Table, Follows::FollowingId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // Add unique constraint to prevent duplicate follows
        m.create_index(
            Index::create()
                .name("idx_follows_unique")
                .table(Follows::Table)
                .col(Follows::FollowerId)
                .col(Follows::FollowingId)
                .unique()
                .to_owned(),
        )
        .await?;

        // Create notifications table
        m.create_table(
            Table::create()
                .table(Notifications::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Notifications::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Notifications::UserId).integer().not_null())
                .col(ColumnDef::new(Notifications::Type).string().not_null())
                .col(ColumnDef::new(Notifications::Title).string().not_null())
                .col(ColumnDef::new(Notifications::Message).text().not_null())
                .col(ColumnDef::new(Notifications::Data).json())
                .col(ColumnDef::new(Notifications::Read).boolean().default(false))
                .col(ColumnDef::new(Notifications::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_notifications_user_id")
                        .from(Notifications::Table, Notifications::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // Add username column to users table
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(ColumnDef::new(Users::Username).string().unique_key())
                .add_column(ColumnDef::new(Users::Bio).text())
                .add_column(ColumnDef::new(Users::FollowerCount).integer().default(0))
                .add_column(ColumnDef::new(Users::FollowingCount).integer().default(0))
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Follows::Table).to_owned())
            .await?;
        m.drop_table(Table::drop().table(Notifications::Table).to_owned())
            .await?;
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::Username)
                .drop_column(Users::Bio)
                .drop_column(Users::FollowerCount)
                .drop_column(Users::FollowingCount)
                .to_owned(),
        )
        .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Follows {
    Table,
    Id,
    FollowerId,
    FollowingId,
    CreatedAt,
}

#[derive(Iden)]
enum Notifications {
    Table,
    Id,
    UserId,
    Type,
    Title,
    Message,
    Data,
    Read,
    CreatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Username,
    Bio,
    FollowerCount,
    FollowingCount,
    Id,
}
