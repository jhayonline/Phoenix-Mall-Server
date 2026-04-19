#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::products::Column;
use crate::models::_entities::{favorites, products};
use crate::{
    models::{_entities::users, products::Entity as Products, users::UpdateProfileParams},
    views::profile::{ProfileResponse, UserStatsResponse},
};
use axum::extract::Multipart;
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
    if let Some(username) = params.username {
        user_active.username = ActiveValue::set(Some(username));
    }
    if let Some(bio) = params.bio {
        user_active.bio = ActiveValue::set(Some(bio));
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
        total_views,
    })
}

// Get seller's products with stats
#[debug_handler]
pub async fn seller_products(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let products_list = products::Entity::find()
        .filter(products::Column::SellerId.eq(user.id))
        .order_by_desc(products::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    // Get stats for each product
    let mut products_with_stats = Vec::new();
    for product in products_list {
        // Get favorite count for this product
        let favorite_count = favorites::Entity::find()
            .filter(favorites::Column::ProductId.eq(product.id))
            .count(&ctx.db)
            .await?;

        products_with_stats.push(serde_json::json!({
            "product": product,
            "favorite_count": favorite_count,
        }));
    }

    format::json(products_with_stats)
}

// Add this function to handle avatar upload
#[debug_handler]
pub async fn upload_avatar(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Process uploaded file
    let mut uploaded_file = None;
    let mut file_extension = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        tracing::error!(error = ?err, "could not read multipart");
        Error::BadRequest("could not read multipart".into())
    })? {
        let file_name = match field.file_name() {
            Some(name) => name.to_string(),
            None => return Err(Error::BadRequest("file name not found".into())),
        };

        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        // Validate content type
        if !content_type.starts_with("image/") {
            return bad_request("Only image files are allowed");
        }

        // Get file extension
        file_extension = std::path::Path::new(&file_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("jpg")
            .to_string();

        let content = field.bytes().await.map_err(|err| {
            tracing::error!(error = ?err, "could not read bytes");
            Error::BadRequest("could not read bytes".into())
        })?;

        uploaded_file = Some(content);
        break; // Only process first file
    }

    let content = uploaded_file.ok_or_else(|| Error::BadRequest("No file uploaded".into()))?;

    // Generate unique filename
    let unique_name = format!("avatar_{}_{}.{}", user.id, Uuid::new_v4(), file_extension);
    let upload_dir = std::path::Path::new("storage/uploads/avatars");

    if !upload_dir.exists() {
        tokio::fs::create_dir_all(upload_dir).await?;
    }

    let file_path = upload_dir.join(&unique_name);
    tokio::fs::write(&file_path, content).await?;

    let avatar_url = format!("/uploads/avatars/{}", unique_name);

    // Update user's avatar_url
    let mut user_active: users::ActiveModel = user.into();
    user_active.avatar_url = ActiveValue::set(Some(avatar_url.clone()));
    user_active.update(&ctx.db).await?;

    format::json(serde_json::json!({
        "avatar_url": avatar_url,
        "message": "Avatar uploaded successfully"
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/profile")
        .add("/me", get(get_profile))
        .add("/me", put(update_profile))
        .add("/listings", get(my_listings))
        .add("/stats", get(my_stats))
        .add("/seller/products", get(seller_products))
        .add("/avatar", post(upload_avatar))
}
