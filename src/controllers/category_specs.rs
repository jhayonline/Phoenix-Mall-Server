#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::category_specs;
use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SpecOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidationRules {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategorySpecResponse {
    pub id: uuid::Uuid,
    pub spec_name: String,
    pub spec_type: String, // text, select, number, boolean
    pub is_required: bool,
    pub preset_options: Option<Vec<String>>,
    pub validation_rules: Option<ValidationRules>,
    pub input_placeholder: Option<String>,
    pub helper_text: Option<String>,
    pub sort_order: i32,
}

// Get category specifications by category ID
#[utoipa::path(
    get,
    path = "/api/categories/{category_id}/specs",
    params(
        ("category_id" = String, Path, description = "Category UUID")
    ),
    responses(
        (status = 200, description = "Category specifications", body = Vec<CategorySpecResponse>),
        (status = 404, description = "Category not found")
    ),
    tag = "categories"
)]
#[debug_handler]
pub async fn get_category_specs(
    State(ctx): State<AppContext>,
    Path(category_id): Path<uuid::Uuid>,
) -> Result<Response> {
    // Verify category exists (optional, but good for error handling)
    let category = crate::models::_entities::categories::Entity::find_by_id(category_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let specs = category_specs::Entity::find()
        .filter(category_specs::Column::CategoryId.eq(category.id))
        .order_by_asc(category_specs::Column::SortOrder)
        .all(&ctx.db)
        .await?;

    let response: Vec<CategorySpecResponse> = specs
        .into_iter()
        .map(|s| {
            // Parse preset_options from JSONB
            let preset_options = s.preset_options.as_ref().and_then(|json| {
                serde_json::from_value::<Vec<String>>(json.clone()).ok()
            });
            
            // Parse validation_rules from JSONB
            let validation_rules = s.validation_rules.as_ref().and_then(|json| {
                serde_json::from_value::<ValidationRules>(json.clone()).ok()
            });
            
            CategorySpecResponse {
                id: s.id,
                spec_name: s.spec_name,
                spec_type: s.spec_type,
                is_required: s.is_required.unwrap_or(false),
                preset_options,
                validation_rules,
                input_placeholder: s.input_placeholder,
                helper_text: s.helper_text,
                sort_order: s.sort_order.unwrap_or(0),
            }
        })
        .collect();

    format::json(response)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/categories")
        .add("/{category_id}/specs", get(get_category_specs))
}
