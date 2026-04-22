use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub level: i32,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl CategoryResponse {
    pub fn from_model(category: &crate::models::_entities::categories::Model) -> Self {
        Self {
            id: category.id,
            name: category.name.clone(),
            slug: category.slug.clone(),
            parent_id: category.parent_id,
            level: category.level,
            description: category.description.clone(),
            is_active: category.is_active,
            created_at: category.created_at,
        }
    }
}
