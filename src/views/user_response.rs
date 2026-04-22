use crate::models::_entities::users;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub pid: Uuid,
    pub email: String,
    pub name: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub follower_count: i32,
    pub following_count: i32,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl UserResponse {
    pub fn from_model(user: &users::Model) -> Self {
        Self {
            id: user.id,
            pid: user.pid,
            email: user.email.clone(),
            name: user.name.clone(),
            username: user.username.clone(),
            avatar_url: user.avatar_url.clone(),
            location: user.location.clone(),
            bio: user.bio.clone(),
            phone_number: user.phone_number.clone(),
            role: user.role.clone(),
            is_active: user.is_active,
            follower_count: user.follower_count.unwrap_or(0),
            following_count: user.following_count.unwrap_or(0),
            email_verified: user.email_verified_at.is_some(),
            created_at: user.created_at,
        }
    }
}
