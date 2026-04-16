#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::{favorites, products, users};
use loco_rs::prelude::*;
use num_traits::cast::ToPrimitive;
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddToWishlistParams {
    pub product_pid: String,
}

#[derive(Debug, Serialize)]
pub struct WishlistResponse {
    pub product_id: Uuid,
    pub product_pid: String,
    pub title: String,
    pub price: f64,
    pub image_url: Option<String>,
    pub condition: Option<String>,
    pub location: Option<String>,
}

// Get user's wishlist
#[debug_handler]
pub async fn get_wishlist(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let wishlist_items = favorites::Entity::find()
        .filter(favorites::Column::UserId.eq(user.id))
        .find_also_related(products::Entity)
        .all(&ctx.db)
        .await?;

    let mut response_items = Vec::new();
    for (_, product_opt) in wishlist_items {
        if let Some(product) = product_opt {
            let image_url = get_product_image(&ctx, &product.id).await;

            response_items.push(WishlistResponse {
                product_id: product.id,
                product_pid: product.pid.clone(),
                title: product.title,
                price: product.price.to_f64().unwrap_or(0.0),
                image_url,
                condition: product.condition.clone(),
                location: product.location.clone(),
            });
        }
    }

    format::json(response_items)
}

// Add item to wishlist
#[debug_handler]
pub async fn add_to_wishlist(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<AddToWishlistParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&params.product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Check if already in wishlist
    let existing = favorites::Entity::find()
        .filter(favorites::Column::UserId.eq(user.id))
        .filter(favorites::Column::ProductId.eq(product.id))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return bad_request("Item already in wishlist");
    }

    // Add to wishlist (no id field, composite key is user_id + product_id)
    let wishlist_item = favorites::ActiveModel {
        user_id: ActiveValue::set(user.id),
        product_id: ActiveValue::set(product.id),
        created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;

    format::empty_json()
}

// Remove item from wishlist
#[debug_handler]
pub async fn remove_from_wishlist(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(product_pid): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    favorites::Entity::delete_many()
        .filter(favorites::Column::UserId.eq(user.id))
        .filter(favorites::Column::ProductId.eq(product.id))
        .exec(&ctx.db)
        .await?;

    format::empty_json()
}

// Clear entire wishlist
#[debug_handler]
pub async fn clear_wishlist(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    favorites::Entity::delete_many()
        .filter(favorites::Column::UserId.eq(user.id))
        .exec(&ctx.db)
        .await?;

    format::empty_json()
}

// Check if product is in wishlist
#[debug_handler]
pub async fn check_in_wishlist(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(product_pid): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let exists = favorites::Entity::find()
        .filter(favorites::Column::UserId.eq(user.id))
        .filter(favorites::Column::ProductId.eq(product.id))
        .one(&ctx.db)
        .await?
        .is_some();

    format::json(serde_json::json!({ "favorited": exists }))
}

// Helper to get product image
async fn get_product_image(ctx: &AppContext, product_id: &Uuid) -> Option<String> {
    use crate::models::_entities::product_images;

    let primary = product_images::Entity::find()
        .filter(product_images::Column::ProductId.eq(*product_id))
        .filter(product_images::Column::IsPrimary.eq(true))
        .one(&ctx.db)
        .await
        .ok()
        .flatten();

    if let Some(img) = primary {
        return Some(img.image_url);
    }

    let any_image = product_images::Entity::find()
        .filter(product_images::Column::ProductId.eq(*product_id))
        .one(&ctx.db)
        .await
        .ok()
        .flatten();

    any_image.map(|img| img.image_url)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/wishlist")
        .add("/", get(get_wishlist))
        .add("/", post(add_to_wishlist))
        .add("/clear", delete(clear_wishlist))
        .add("/{pid}", delete(remove_from_wishlist))
        .add("/{pid}/check", get(check_in_wishlist))
}
