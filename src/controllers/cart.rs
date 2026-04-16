#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::{carts, products, users};
use loco_rs::prelude::*;
use num_traits::cast::ToPrimitive;
use sea_orm::{EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddToCartParams {
    pub product_pid: String,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCartParams {
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_pid: String,
    pub title: String,
    pub price: f64,
    pub quantity: i32,
    pub image_url: Option<String>,
    pub condition: Option<String>,
    pub location: Option<String>,
}

// Get user's cart
#[debug_handler]
pub async fn get_cart(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let cart_items = carts::Entity::find()
        .filter(carts::Column::UserId.eq(user.id))
        .find_also_related(products::Entity)
        .all(&ctx.db)
        .await?;

    let mut response_items = Vec::new();
    for (cart_item, product_opt) in cart_items {
        if let Some(product) = product_opt {
            // Get product image
            let image_url = get_product_image(&ctx, &product.id).await;

            response_items.push(CartResponse {
                id: cart_item.id,
                product_id: product.id,
                product_pid: product.pid.clone(),
                title: product.title,
                price: product.price.to_f64().unwrap_or(0.0),
                quantity: cart_item.quantity,
                image_url,
                condition: product.condition.clone(),
                location: product.location.clone(),
            });
        }
    }

    format::json(response_items)
}

// Add item to cart
#[debug_handler]
pub async fn add_to_cart(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<AddToCartParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&params.product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Check if product is active
    if product.status != Some("active".to_string()) {
        return bad_request("Product is not available");
    }

    // Check if item already in cart
    let existing_item = carts::Entity::find()
        .filter(carts::Column::UserId.eq(user.id))
        .filter(carts::Column::ProductId.eq(product.id))
        .one(&ctx.db)
        .await?;

    if let Some(item) = existing_item {
        // Update quantity - use item.quantity directly
        let new_quantity = item.quantity + params.quantity;
        let mut active_item: carts::ActiveModel = item.clone().into();
        active_item.quantity = Set(new_quantity);
        active_item.updated_at = Set(Some(chrono::Utc::now().into()));
        active_item.update(&ctx.db).await?;
    } else {
        // Add new item
        let _cart_item = carts::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user.id),
            product_id: Set(product.id),
            quantity: Set(params.quantity),
            created_at: Set(Some(chrono::Utc::now().into())),
            updated_at: Set(Some(chrono::Utc::now().into())),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await?;
    }

    format::empty_json()
}

// Update cart item quantity
#[debug_handler]
pub async fn update_cart_item(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(product_pid): Path<String>,
    Json(params): Json<UpdateCartParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let cart_item = carts::Entity::find()
        .filter(carts::Column::UserId.eq(user.id))
        .filter(carts::Column::ProductId.eq(product.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let mut active_item: carts::ActiveModel = cart_item.into();
    active_item.quantity = Set(params.quantity);
    active_item.updated_at = Set(Some(chrono::Utc::now().into()));
    active_item.update(&ctx.db).await?;

    format::empty_json()
}

// Remove item from cart
#[debug_handler]
pub async fn remove_from_cart(
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

    carts::Entity::delete_many()
        .filter(carts::Column::UserId.eq(user.id))
        .filter(carts::Column::ProductId.eq(product.id))
        .exec(&ctx.db)
        .await?;

    format::empty_json()
}

// Clear entire cart
#[debug_handler]
pub async fn clear_cart(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    carts::Entity::delete_many()
        .filter(carts::Column::UserId.eq(user.id))
        .exec(&ctx.db)
        .await?;

    format::empty_json()
}

// Helper to get product image
async fn get_product_image(ctx: &AppContext, product_id: &Uuid) -> Option<String> {
    use crate::models::_entities::product_images;

    // Try to get primary image first
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

    // Fallback to any image
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
        .prefix("api/cart")
        .add("/", get(get_cart))
        .add("/", post(add_to_cart))
        .add("/clear", delete(clear_cart))
        .add("/{pid}", put(update_cart_item))
        .add("/{pid}", delete(remove_from_cart))
}
