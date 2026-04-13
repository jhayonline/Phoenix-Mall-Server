use crate::models::_entities::products;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductResponse {
    pub pid: String,
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub category_id: Option<uuid::Uuid>,
    pub seller_id: i32,
    pub status: Option<String>,
    pub whatsapp_contact: Option<bool>,
    pub phone_contact: Option<bool>,
    pub views_count: Option<i32>,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl ProductResponse {
    pub fn new(product: &products::Model) -> Self {
        Self {
            pid: product.pid.clone(),
            title: product.title.clone(),
            description: product.description.clone(),
            price: product.price.to_f64().unwrap_or(0.0),
            condition: product.condition.clone(),
            location: product.location.clone(),
            category_id: product.category_id,
            seller_id: product.seller_id,
            status: product.status.clone(),
            whatsapp_contact: product.whatsapp_contact,
            phone_contact: product.phone_contact,
            views_count: product.views_count,
            created_at: product.created_at,
        }
    }
}
