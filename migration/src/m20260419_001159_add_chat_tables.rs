use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create conversations table
        m.create_table(
            Table::create()
                .table(Conversations::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Conversations::Id)
                        .string()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Conversations::BuyerId).integer().not_null())
                .col(ColumnDef::new(Conversations::SellerId).integer().not_null())
                .col(ColumnDef::new(Conversations::ProductId).uuid())
                .col(ColumnDef::new(Conversations::LastMessage).text())
                .col(ColumnDef::new(Conversations::LastMessageTime).timestamp_with_time_zone())
                .col(ColumnDef::new(Conversations::CreatedAt).timestamp_with_time_zone())
                .col(ColumnDef::new(Conversations::UpdatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_conversations_buyer_id")
                        .from(Conversations::Table, Conversations::BuyerId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_conversations_seller_id")
                        .from(Conversations::Table, Conversations::SellerId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // Create messages table
        m.create_table(
            Table::create()
                .table(Messages::Table)
                .if_not_exists()
                .col(ColumnDef::new(Messages::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(Messages::ConversationId).string().not_null())
                .col(ColumnDef::new(Messages::SenderId).integer().not_null())
                .col(ColumnDef::new(Messages::ReceiverId).integer().not_null())
                .col(ColumnDef::new(Messages::Message).text().not_null())
                .col(ColumnDef::new(Messages::Read).boolean().default(false))
                .col(ColumnDef::new(Messages::CreatedAt).timestamp_with_time_zone())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_messages_conversation_id")
                        .from(Messages::Table, Messages::ConversationId)
                        .to(Conversations::Table, Conversations::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_messages_sender_id")
                        .from(Messages::Table, Messages::SenderId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_messages_receiver_id")
                        .from(Messages::Table, Messages::ReceiverId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Messages::Table).to_owned())
            .await?;
        m.drop_table(Table::drop().table(Conversations::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Conversations {
    Table,
    Id,
    BuyerId,
    SellerId,
    ProductId,
    LastMessage,
    LastMessageTime,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Messages {
    Table,
    Id,
    ConversationId,
    SenderId,
    ReceiverId,
    Message,
    Read,
    CreatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
