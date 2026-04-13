use crate::models::_entities::users;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub whatsapp_enabled: Option<bool>,
    pub phone_enabled: Option<bool>,
    pub is_active: Option<bool>,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ProfileResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
            phone_number: user.phone_number.clone(),
            avatar_url: user.avatar_url.clone(),
            location: user.location.clone(),
            whatsapp_enabled: user.whatsapp_enabled,
            phone_enabled: user.phone_enabled,
            is_active: user.is_active,
            email_verified: user.email_verified_at.is_some(),
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserStatsResponse {
    pub total_listings: u64,
    pub active_listings: u64,
    pub sold_listings: u64,
    pub total_favorites: u64,
    pub total_views: i64,
}
