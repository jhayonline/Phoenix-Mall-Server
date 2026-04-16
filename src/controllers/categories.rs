#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::categories::{self, Entity as Categories};
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategoryParams {
    pub name: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub level: i32,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCategoryParams {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub parent_id: Option<Option<Uuid>>,
    pub level: Option<i32>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
}

// Public endpoints (no auth required)
#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let categories = Categories::find()
        .order_by_asc(categories::Column::Level)
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    format::json(categories)
}

#[debug_handler]
pub async fn get_by_slug(
    State(ctx): State<AppContext>,
    Path(slug): Path<String>,
) -> Result<Response> {
    let category = Categories::find()
        .filter(categories::Column::Slug.eq(slug))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    format::json(category)
}

#[debug_handler]
pub async fn get_category_tree(State(ctx): State<AppContext>) -> Result<Response> {
    let all_categories = Categories::find()
        .order_by_asc(categories::Column::Level)
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    // First, create a map of categories without children
    let mut category_map: std::collections::HashMap<Uuid, serde_json::Value> =
        std::collections::HashMap::new();

    for cat in &all_categories {
        category_map.insert(
            cat.id,
            serde_json::json!({
                "id": cat.id,
                "name": cat.name,
                "slug": cat.slug,
                "level": cat.level,
                "display_order": cat.display_order,
                "is_active": cat.is_active,
                "children": Vec::<serde_json::Value>::new(),
            }),
        );
    }

    // Build tree structure
    let mut tree: Vec<serde_json::Value> = Vec::new();

    for cat in &all_categories {
        if let Some(parent_id) = cat.parent_id {
            // This is a child category - clone the child value first
            let child_value = category_map.get(&cat.id).cloned();
            if let Some(child) = child_value {
                if let Some(parent) = category_map.get_mut(&parent_id) {
                    if let Some(children) = parent.get_mut("children") {
                        children.as_array_mut().unwrap().push(child);
                    }
                }
            }
        } else {
            // This is a root category
            if let Some(root_cat) = category_map.get(&cat.id) {
                tree.push(root_cat.clone());
            }
        }
    }

    format::json(tree)
}

// Admin-only endpoints
#[debug_handler]
pub async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateCategoryParams>,
) -> Result<Response> {
    // Verify admin role
    let user =
        crate::models::_entities::users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if user.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    // Check if slug already exists
    let existing = Categories::find()
        .filter(categories::Column::Slug.eq(&params.slug))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return bad_request("Category with this slug already exists");
    }

    let category = categories::ActiveModel {
        id: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(params.name),
        slug: ActiveValue::set(params.slug),
        parent_id: ActiveValue::set(params.parent_id),
        level: ActiveValue::set(params.level),
        display_order: ActiveValue::set(params.display_order.map(|v| v as i32)),
        is_active: ActiveValue::set(Some(true)),
        created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;

    format::json(category)
}

#[debug_handler]
pub async fn update(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Json(params): Json<UpdateCategoryParams>,
) -> Result<Response> {
    // Verify admin role
    let user =
        crate::models::_entities::users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if user.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    let category = Categories::find()
        .filter(categories::Column::Id.eq(id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let mut active_category: categories::ActiveModel = category.into();

    if let Some(name) = params.name {
        active_category.name = ActiveValue::set(name);
    }
    if let Some(slug) = params.slug {
        // Check if new slug already exists (excluding current)
        let existing = Categories::find()
            .filter(categories::Column::Slug.eq(&slug))
            .filter(categories::Column::Id.ne(id))
            .one(&ctx.db)
            .await?;

        if existing.is_some() {
            return bad_request("Category with this slug already exists");
        }
        active_category.slug = ActiveValue::set(slug);
    }
    if let Some(parent_id) = params.parent_id {
        active_category.parent_id = ActiveValue::set(parent_id);
    }
    if let Some(level) = params.level {
        active_category.level = ActiveValue::set(level);
    }
    if let Some(display_order) = params.display_order {
        active_category.display_order = ActiveValue::set(Some(display_order as i32));
    }
    if let Some(is_active) = params.is_active {
        active_category.is_active = ActiveValue::set(Some(is_active));
    }

    let updated = active_category.update(&ctx.db).await?;
    format::json(updated)
}

#[debug_handler]
pub async fn delete_category(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
) -> Result<Response> {
    // Verify admin role
    let user =
        crate::models::_entities::users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if user.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    // Check if category has children
    let children = Categories::find()
        .filter(categories::Column::ParentId.eq(id))
        .count(&ctx.db)
        .await?;

    if children > 0 {
        return bad_request("Cannot delete category that has subcategories");
    }

    let category = Categories::find()
        .filter(categories::Column::Id.eq(id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    category.delete(&ctx.db).await?;
    format::empty_json()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/categories")
        .add("/list", get(list))
        .add("/tree", get(get_category_tree))
        .add("/{slug}", get(get_by_slug))
        .add("/admin", post(create))
        .add("/admin/{id}", put(update))
        .add("/admin/{id}", delete(delete_category))
}
