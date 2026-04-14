#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::{product_images, products, users};
use axum::extract::Multipart;
use loco_rs::prelude::*;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use std::path::Path as StdPath;
use tokio::fs;
use uuid::Uuid;

#[debug_handler]
pub async fn upload(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    axum::extract::Path(product_pid): axum::extract::Path<String>,
    mut multipart: Multipart,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid)) // Borrow here
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if product.seller_id != user.id {
        return unauthorized("You can only upload images for your own products");
    }

    // Check image limit (max 5 images per product)
    let image_count = product_images::Entity::find()
        .filter(product_images::Column::ProductId.eq(product.id))
        .count(&ctx.db)
        .await?;

    if image_count >= 5 {
        return bad_request("Maximum 5 images per product");
    }

    // Process uploaded file
    let mut uploaded_file = None;
    let mut file_name = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        tracing::error!(error = ?err, "could not read multipart");
        Error::BadRequest("could not read multipart".into())
    })? {
        file_name = match field.file_name() {
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

        let content = field.bytes().await.map_err(|err| {
            tracing::error!(error = ?err, "could not read bytes");
            Error::BadRequest("could not read bytes".into())
        })?;

        uploaded_file = Some(content);
        break; // Only process first file
    }

    let content = uploaded_file.ok_or_else(|| Error::BadRequest("No file uploaded".into()))?;

    // Generate unique filename, clone product_pid here
    let ext = StdPath::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");

    let unique_name = format!("{}_{}.{}", product_pid.clone(), Uuid::new_v4(), ext);
    let upload_dir = StdPath::new("storage/uploads");

    if !upload_dir.exists() {
        fs::create_dir_all(upload_dir).await?;
    }

    let file_path = upload_dir.join(&unique_name);

    fs::write(&file_path, content).await?;

    let image_url = format!("/uploads/{}", unique_name);

    // Determine if this is the first image (primary)
    let is_primary = image_count == 0;

    let product_image = product_images::ActiveModel {
        id: ActiveValue::set(Uuid::new_v4()),
        product_id: ActiveValue::set(product.id),
        image_url: ActiveValue::set(image_url),
        is_primary: ActiveValue::set(Some(is_primary)),
        display_order: ActiveValue::set(Some(image_count as i32)),
        created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;

    format::json(serde_json::json!({
        "id": product_image.id,
        "image_url": product_image.image_url,
        "is_primary": product_image.is_primary,
        "display_order": product_image.display_order
    }))
}

// Get images for a product
#[debug_handler]
pub async fn get_product_images(
    State(ctx): State<AppContext>,
    axum::extract::Path(product_pid): axum::extract::Path<String>,
) -> Result<Response> {
    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid)) // Borrow here
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let images = product_images::Entity::find()
        .filter(product_images::Column::ProductId.eq(product.id))
        .order_by_asc(product_images::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    format::json(images)
}

// Set primary image
#[debug_handler]
pub async fn set_primary(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    axum::extract::Path((product_pid, image_id)): axum::extract::Path<(String, String)>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid)) // Borrow here
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if product.seller_id != user.id {
        return unauthorized("You can only modify your own products");
    }

    let image_id_uuid = Uuid::parse_str(&image_id)
        .map_err(|_| Error::BadRequest("Invalid image ID".to_string()))?;

    // Reset all images for this product to non-primary
    let all_images = product_images::Entity::find()
        .filter(product_images::Column::ProductId.eq(product.id))
        .all(&ctx.db)
        .await?;

    for mut img in all_images {
        img.is_primary = Some(false);
        let active_img: product_images::ActiveModel = img.into();
        active_img.update(&ctx.db).await?;
    }

    // Set the selected image as primary
    let mut image = product_images::Entity::find()
        .filter(product_images::Column::Id.eq(image_id_uuid))
        .filter(product_images::Column::ProductId.eq(product.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    image.is_primary = Some(true);
    let active_image: product_images::ActiveModel = image.into();
    active_image.update(&ctx.db).await?;

    format::empty_json()
}

// Delete image
#[debug_handler]
pub async fn delete_image(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    axum::extract::Path((product_pid, image_id)): axum::extract::Path<(String, String)>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid)) // Borrow here
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if product.seller_id != user.id {
        return unauthorized("You can only delete images from your own products");
    }

    let image_id_uuid = Uuid::parse_str(&image_id)
        .map_err(|_| Error::BadRequest("Invalid image ID".to_string()))?;

    let image = product_images::Entity::find()
        .filter(product_images::Column::Id.eq(image_id_uuid))
        .filter(product_images::Column::ProductId.eq(product.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Delete file from disk
    let file_name = image.image_url.trim_start_matches("/uploads/");
    let file_path = StdPath::new("storage/uploads").join(file_name);

    if file_path.exists() {
        let _ = fs::remove_file(file_path).await;
    }

    // Delete from database
    image.delete(&ctx.db).await?;

    format::empty_json()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products")
        .add("/{pid}/images", post(upload))
        .add("/{pid}/images", get(get_product_images))
        .add("/{pid}/images/{image_id}/primary", put(set_primary))
        .add("/{pid}/images/{image_id}", delete(delete_image))
}
