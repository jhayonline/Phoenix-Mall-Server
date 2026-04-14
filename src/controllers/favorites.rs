#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::{favorites, products, users};
use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryOrder};

#[debug_handler]
pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(product_pid): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Check if already favorited
    let existing = favorites::Entity::find()
        .filter(favorites::Column::UserId.eq(user.id))
        .filter(favorites::Column::ProductId.eq(product.id))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return bad_request("Product already in favorites");
    }

    let favorite = favorites::ActiveModel {
        user_id: ActiveValue::set(user.id),
        product_id: ActiveValue::set(product.id),
        created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;

    format::json(favorite)
}

#[debug_handler]
pub async fn remove(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(product_pid): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(product_pid))
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

#[debug_handler]
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let favorites = favorites::Entity::find()
        .filter(favorites::Column::UserId.eq(user.id))
        .find_also_related(products::Entity)
        .order_by_desc(favorites::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    let products_with_favorite: Vec<products::Model> = favorites
        .into_iter()
        .filter_map(|(_, product)| product)
        .collect();

    format::json(products_with_favorite)
}

#[debug_handler]
pub async fn is_favorited(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(product_pid): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let is_fav = favorites::Entity::find()
        .filter(favorites::Column::UserId.eq(user.id))
        .filter(favorites::Column::ProductId.eq(product.id))
        .one(&ctx.db)
        .await?
        .is_some();

    format::json(serde_json::json!({ "favorited": is_fav }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/favorites")
        .add("/", get(list))
        .add("/{pid}", post(add))
        .add("/{pid}", delete(remove))
        .add("/{pid}/check", get(is_favorited))
}
