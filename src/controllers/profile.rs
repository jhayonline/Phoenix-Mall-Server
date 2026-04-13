#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::products::Column;
use crate::{
    models::{_entities::users, products::Entity as Products, users::UpdateProfileParams},
    views::profile::{ProfileResponse, UserStatsResponse},
};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, PaginatorTrait, QueryOrder, QuerySelect};

// Get current user profile
#[debug_handler]
pub async fn get_profile(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(ProfileResponse::new(&user))
}

// Update current user profile
#[debug_handler]
pub async fn update_profile(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateProfileParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let mut user_active: users::ActiveModel = user.into();

    if let Some(name) = params.name {
        user_active.name = ActiveValue::set(name);
    }
    if let Some(phone_number) = params.phone_number {
        user_active.phone_number = ActiveValue::set(Some(phone_number));
    }
    if let Some(location) = params.location {
        user_active.location = ActiveValue::set(Some(location));
    }
    if let Some(avatar_url) = params.avatar_url {
        user_active.avatar_url = ActiveValue::set(Some(avatar_url));
    }
    if let Some(whatsapp_enabled) = params.whatsapp_enabled {
        user_active.whatsapp_enabled = ActiveValue::set(Some(whatsapp_enabled));
    }
    if let Some(phone_enabled) = params.phone_enabled {
        user_active.phone_enabled = ActiveValue::set(Some(phone_enabled));
    }

    let updated_user = user_active.update(&ctx.db).await?;
    format::json(ProfileResponse::new(&updated_user))
}

// Get user's listings
#[debug_handler]
pub async fn my_listings(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let listings = Products::find()
        .filter(Column::SellerId.eq(user.id))
        .order_by_desc(Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    format::json(listings)
}

// Get user stats
#[debug_handler]
pub async fn my_stats(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let total_listings = Products::find()
        .filter(Column::SellerId.eq(user.id))
        .count(&ctx.db)
        .await?;

    let active_listings = Products::find()
        .filter(Column::SellerId.eq(user.id))
        .filter(Column::Status.eq("active"))
        .count(&ctx.db)
        .await?;

    let sold_listings = Products::find()
        .filter(Column::SellerId.eq(user.id))
        .filter(Column::Status.eq("sold"))
        .count(&ctx.db)
        .await?;

    // Fix: Use i64 instead of i32 for sum
    let total_views: i64 = Products::find()
        .filter(Column::SellerId.eq(user.id))
        .select_only()
        .column_as(Column::ViewsCount.sum(), "total_views")
        .into_tuple()
        .one(&ctx.db)
        .await?
        .unwrap_or(0);

    format::json(UserStatsResponse {
        total_listings,
        active_listings,
        sold_listings,
        total_favorites: 0,
        total_views: total_views,
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/profile")
        .add("/me", get(get_profile))
        .add("/me", put(update_profile))
        .add("/listings", get(my_listings))
        .add("/stats", get(my_stats))
}
