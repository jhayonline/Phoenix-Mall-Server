#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::{conversations, messages, users};
use loco_rs::prelude::*;
use sea_orm::{Condition, PaginatorTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageParams {
    pub receiver_id: i32,
    pub message: String,
    pub product_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub conversation_id: String,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub read: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub id: String,
    pub other_user_id: i32,
    pub other_user_name: String,
    pub other_user_avatar: Option<String>,
    pub last_message: String,
    pub last_message_time: String,
    pub unread_count: u64,
}

// Send a message
#[debug_handler]
pub async fn send_message(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<SendMessageParams>,
) -> Result<Response> {
    let sender = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Find or create conversation
    let conversation =
        find_or_create_conversation(&ctx.db, sender.id, params.receiver_id, params.product_id)
            .await?;

    // Clone message for later use
    let message_text = params.message.clone();

    // Create message
    let new_message = messages::ActiveModel {
        id: ActiveValue::set(Uuid::new_v4()),
        conversation_id: ActiveValue::set(conversation.id.clone()),
        sender_id: ActiveValue::set(sender.id),
        receiver_id: ActiveValue::set(params.receiver_id),
        message: ActiveValue::set(params.message),
        read: ActiveValue::set(Some(false)),
        created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
    };

    let saved_message = new_message.insert(&ctx.db).await?;

    // Update conversation's last message
    let mut conv_active: conversations::ActiveModel = conversation.into();
    conv_active.last_message = ActiveValue::set(Some(message_text));
    conv_active.last_message_time = ActiveValue::set(Some(chrono::Utc::now().into()));
    conv_active.updated_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    conv_active.update(&ctx.db).await?;

    format::json(MessageResponse {
        id: saved_message.id,
        conversation_id: saved_message.conversation_id,
        sender_id: saved_message.sender_id,
        receiver_id: saved_message.receiver_id,
        message: saved_message.message,
        read: saved_message.read.unwrap_or(false),
        created_at: saved_message.created_at.unwrap().to_string(),
    })
}

// Get user's conversations
#[debug_handler]
pub async fn get_conversations(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let convs = conversations::Entity::find()
        .filter(
            Condition::any()
                .add(conversations::Column::BuyerId.eq(user.id))
                .add(conversations::Column::SellerId.eq(user.id)),
        )
        .order_by_desc(conversations::Column::UpdatedAt)
        .all(&ctx.db)
        .await?;

    let mut responses = Vec::new();
    for conv in convs {
        let other_user_id = if conv.buyer_id == user.id {
            conv.seller_id
        } else {
            conv.buyer_id
        };
        let other_user = users::Entity::find_by_id(other_user_id)
            .one(&ctx.db)
            .await?;

        let unread_count = messages::Entity::find()
            .filter(messages::Column::ConversationId.eq(&conv.id))
            .filter(messages::Column::ReceiverId.eq(user.id))
            .filter(messages::Column::Read.eq(false))
            .count(&ctx.db)
            .await?;

        if let Some(other) = other_user {
            responses.push(ConversationResponse {
                id: conv.id,
                other_user_id: other.id,
                other_user_name: other.name,
                other_user_avatar: other.avatar_url,
                last_message: conv.last_message.unwrap_or_default(),
                last_message_time: conv
                    .last_message_time
                    .unwrap_or_else(|| chrono::Utc::now().into())
                    .to_string(),
                unread_count,
            });
        }
    }

    format::json(responses)
}

// Get messages for a conversation
#[debug_handler]
pub async fn get_messages(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(conversation_id): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Verify user is part of this conversation
    let conversation = conversations::Entity::find_by_id(&conversation_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if conversation.buyer_id != user.id && conversation.seller_id != user.id {
        return unauthorized("You are not part of this conversation");
    }

    let msgs = messages::Entity::find()
        .filter(messages::Column::ConversationId.eq(&conversation_id))
        .order_by_asc(messages::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    // Mark messages as read
    for msg in &msgs {
        if msg.receiver_id == user.id && msg.read == Some(false) {
            let mut active_msg: messages::ActiveModel = msg.clone().into();
            active_msg.read = ActiveValue::set(Some(true));
            let _ = active_msg.update(&ctx.db).await;
        }
    }

    let message_responses: Vec<MessageResponse> = msgs
        .into_iter()
        .map(|msg| MessageResponse {
            id: msg.id,
            conversation_id: msg.conversation_id,
            sender_id: msg.sender_id,
            receiver_id: msg.receiver_id,
            message: msg.message,
            read: msg.read.unwrap_or(false),
            created_at: msg.created_at.unwrap().to_string(),
        })
        .collect();

    format::json(message_responses)
}

// Helper function to find or create conversation
async fn find_or_create_conversation(
    db: &DatabaseConnection,
    buyer_id: i32,
    seller_id: i32,
    product_id: Option<Uuid>,
) -> Result<conversations::Model, DbErr> {
    let existing = conversations::Entity::find()
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(conversations::Column::BuyerId.eq(buyer_id))
                        .add(conversations::Column::SellerId.eq(seller_id)),
                )
                .add(
                    Condition::all()
                        .add(conversations::Column::BuyerId.eq(seller_id))
                        .add(conversations::Column::SellerId.eq(buyer_id)),
                ),
        )
        .one(db)
        .await?;

    if let Some(conv) = existing {
        Ok(conv)
    } else {
        let new_conversation = conversations::ActiveModel {
            id: ActiveValue::set(Uuid::new_v4().to_string()),
            buyer_id: ActiveValue::set(buyer_id),
            seller_id: ActiveValue::set(seller_id),
            product_id: ActiveValue::set(product_id),
            last_message: ActiveValue::set(None),
            last_message_time: ActiveValue::set(None),
            created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
            updated_at: ActiveValue::set(Some(chrono::Utc::now().into())),
        };
        new_conversation.insert(db).await
    }
}

// Get total unread messages count for the user
#[debug_handler]
pub async fn get_unread_count(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let unread_count = messages::Entity::find()
        .filter(messages::Column::ReceiverId.eq(user.id))
        .filter(messages::Column::Read.eq(false))
        .count(&ctx.db)
        .await?;

    format::json(serde_json::json!({ "total": unread_count }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/chat")
        .add("/send", post(send_message))
        .add("/conversations", get(get_conversations))
        .add("/messages/{conversation_id}", get(get_messages))
        .add("/unread-count", get(get_unread_count))
}
