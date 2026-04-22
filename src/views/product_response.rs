use crate::models::_entities::products;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductResponse {
    pub id: Uuid,
    pub pid: String,
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub category_id: Option<Uuid>,
    pub seller_id: i32,
    pub status: Option<String>,
    pub whatsapp_contact: Option<bool>,
    pub phone_contact: Option<bool>,
    pub views_count: Option<i32>,
    pub average_rating: Option<f64>,
    pub total_reviews: Option<i32>,
    pub wishlist_count: Option<i32>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl ProductResponse {
    pub fn from_model(product: &products::Model) -> Self {
        Self {
            id: product.id,
            pid: product.pid.clone(),
            title: product.title.clone(),
            description: product.description.clone(),
            price: product.price.to_string().parse::<f64>().unwrap_or(0.0),
            condition: product.condition.clone(),
            location: product.location.clone(),
            category_id: product.category_id,
            seller_id: product.seller_id,
            status: product.status.clone(),
            whatsapp_contact: product.whatsapp_contact,
            phone_contact: product.phone_contact,
            views_count: product.views_count,
            average_rating: product.average_rating,
            total_reviews: product.total_reviews,
            wishlist_count: product.wishlist_count,
            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}
