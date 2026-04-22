#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::{products, users};
use loco_rs::prelude::*;
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserStatusParams {
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserRoleParams {
    pub role: String,
}

// Get platform stats
#[debug_handler]
pub async fn get_stats(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    // Verify admin role
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if user.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    let total_users = users::Entity::find().count(&ctx.db).await?;
    let total_products = products::Entity::find().count(&ctx.db).await?;
    let active_products = products::Entity::find()
        .filter(products::Column::Status.eq("active"))
        .count(&ctx.db)
        .await?;
    let sold_products = products::Entity::find()
        .filter(products::Column::Status.eq("sold"))
        .count(&ctx.db)
        .await?;

    format::json(serde_json::json!({
        "total_users": total_users,
        "total_products": total_products,
        "active_products": active_products,
        "sold_products": sold_products,
    }))
}

// Get all users (admin only)
#[debug_handler]
pub async fn list_users(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let admin = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if admin.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    let all_users = users::Entity::find()
        .order_by_desc(users::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    format::json(all_users)
}

// Update user status (suspend/activate)
#[debug_handler]
pub async fn update_user_status(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(params): Json<UpdateUserStatusParams>,
) -> Result<Response> {
    let admin = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if admin.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    let user = users::Entity::find()
        .filter(users::Column::Id.eq(user_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let mut active_user: users::ActiveModel = user.into();
    active_user.is_active = ActiveValue::set(Some(params.is_active));
    active_user.update(&ctx.db).await?;

    format::empty_json()
}

// Update user role
#[debug_handler]
pub async fn update_user_role(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(params): Json<UpdateUserRoleParams>,
) -> Result<Response> {
    let admin = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if admin.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    let user = users::Entity::find()
        .filter(users::Column::Id.eq(user_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let mut active_user: users::ActiveModel = user.into();
    active_user.role = ActiveValue::set(Some(params.role));
    active_user.update(&ctx.db).await?;

    format::empty_json()
}

// Delete user
#[debug_handler]
pub async fn delete_user(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Response> {
    let admin = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if admin.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    let user = users::Entity::find()
        .filter(users::Column::Id.eq(user_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    user.delete(&ctx.db).await?;
    format::empty_json()
}

// Get all products (admin only)
#[debug_handler]
pub async fn list_products(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let admin = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if admin.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    let all_products = products::Entity::find()
        .order_by_desc(products::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    format::json(all_products)
}

// Delete product (admin only)
#[debug_handler]
pub async fn delete_product(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(product_id): Path<Uuid>,
) -> Result<Response> {
    let admin = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if admin.role != Some("admin".to_string()) {
        return unauthorized("Admin access required");
    }

    let product = products::Entity::find()
        .filter(products::Column::Id.eq(product_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    product.delete(&ctx.db).await?;
    format::empty_json()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin")
        .add("/stats", get(get_stats))
        .add("/users", get(list_users))
        .add("/users/{id}/status", put(update_user_status))
        .add("/users/{id}/role", put(update_user_role))
        .add("/users/{id}", delete(delete_user))
        .add("/products", get(list_products))
        .add("/products/{id}", delete(delete_product))
}
