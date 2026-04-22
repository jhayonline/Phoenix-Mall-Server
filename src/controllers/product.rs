#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::products;
use crate::models::_entities::{favorites, follows, notifications, product_reviews};
use crate::{
    models::{
        _entities::users,
        products::{
            CreateProductParams, PaginatedProductsResponse, ProductQueryParams, UpdateProductParams,
        },
    },
    views::product_response::ProductResponse,
};
use loco_rs::prelude::*;
use nanoid::nanoid;
use num_traits::cast::FromPrimitive;
use sea_orm::{Condition, PaginatorTrait, QuerySelect};
use uuid::Uuid;

// List all products with pagination, filters, and search
#[utoipa::path(
    get,
    path = "/api/products/list",
    params(ProductQueryParams),
    responses(
        (status = 200, description = "List of products", body = PaginatedProductsResponse),
        (status = 400, description = "Invalid query parameters")
    ),
    tag = "products"
)]
#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<ProductQueryParams>,
) -> Result<Response> {
    let paginated = products::Entity::paginate(&ctx.db, &params).await?;
    tracing::info!(
        "Found {} products, page {} of {}",
        paginated.items.len(),
        paginated.page,
        paginated.total_pages
    );

    format::json(paginated)
}

// Get seller info by ID (for product detail page)
#[debug_handler]
pub async fn get_seller(
    State(ctx): State<AppContext>,
    Path(seller_id): Path<i32>,
) -> Result<Response> {
    let user = users::Entity::find_by_id(seller_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let product_count = products::Entity::find()
        .filter(products::Column::SellerId.eq(user.id))
        .filter(products::Column::Status.eq("active"))
        .count(&ctx.db)
        .await?;

    format::json(serde_json::json!({
        "id": user.id,
        "pid": user.pid,
        "name": user.name,
        "username": user.username,
        "avatar_url": user.avatar_url,
        "location": user.location,
        "bio": user.bio,
        "phone_number": user.phone_number,
        "whatsapp_enabled": user.whatsapp_enabled,
        "phone_enabled": user.phone_enabled,
        "follower_count": user.follower_count.unwrap_or(0),
        "following_count": user.following_count.unwrap_or(0),
        "product_count": product_count,
    }))
}

// Get single product by pid (public)
#[utoipa::path(
    get,
    path = "/api/products/get/{pid}",
    params(
        ("pid" = String, Path, description = "Product public ID")
    ),
    responses(
        (status = 200, description = "Product found", body = ProductResponse),
        (status = 404, description = "Product not found")
    ),
    tag = "products"
)]
#[debug_handler]
pub async fn get_by_pid(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let product = products::Model::find_by_pid(&ctx.db, &pid)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Get wishlist count
    let wishlist_count = favorites::Entity::find()
        .filter(favorites::Column::ProductId.eq(product.id))
        .count(&ctx.db)
        .await?;

    // Get rating info from database
    let reviews = product_reviews::Entity::find()
        .filter(product_reviews::Column::ProductId.eq(product.id))
        .all(&ctx.db)
        .await?;

    let (average_rating, total_reviews) = if reviews.is_empty() {
        (0.0, 0)
    } else {
        let sum: i32 = reviews.iter().map(|r| r.rating).sum();
        let avg = sum as f64 / reviews.len() as f64;
        (avg, reviews.len())
    };

    // Increment view count
    let mut active_product: products::ActiveModel = product.clone().into();
    let current_views = product.views_count.unwrap_or(0);
    active_product.views_count = ActiveValue::set(Some(current_views + 1));
    let _ = active_product.update(&ctx.db).await;

    format::json(serde_json::json!({
        "id": product.id,
        "pid": product.pid,
        "title": product.title,
        "description": product.description,
        "price": product.price,
        "condition": product.condition,
        "location": product.location,
        "category_id": product.category_id,
        "seller_id": product.seller_id,
        "status": product.status,
        "whatsapp_contact": product.whatsapp_contact,
        "phone_contact": product.phone_contact,
        "views_count": current_views + 1,
        "created_at": product.created_at,
        "updated_at": product.updated_at,
        "wishlist_count": wishlist_count,
        "average_rating": average_rating,
        "total_reviews": total_reviews,
    }))
}

// Create new product (auth required)
#[debug_handler]
pub async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateProductParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let pid = nanoid!(21);
    let product_id = uuid::Uuid::new_v4();

    let decimal_price = loco_rs::prelude::Decimal::from_f64(params.price)
        .ok_or_else(|| Error::BadRequest("Invalid price".to_string()))?;

    let product = products::ActiveModel {
        id: ActiveValue::set(product_id),
        pid: ActiveValue::set(pid),
        title: ActiveValue::set(params.title),
        description: ActiveValue::set(params.description),
        price: ActiveValue::set(decimal_price),
        condition: ActiveValue::set(params.condition),
        location: ActiveValue::set(params.location),
        category_id: ActiveValue::set(params.category_id),
        seller_id: ActiveValue::set(user.id),
        status: ActiveValue::set(Some("active".to_string())),
        whatsapp_contact: ActiveValue::set(params.whatsapp_contact),
        phone_contact: ActiveValue::set(params.phone_contact),
        views_count: ActiveValue::set(Some(0)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;

    // Notify followers about new product
    let followers_list = follows::Entity::find()
        .filter(follows::Column::FollowingId.eq(user.id))
        .all(&ctx.db)
        .await?;

    for follower in followers_list {
        let notification_data = serde_json::json!({
            "product_id": product.id,
            "product_pid": product.pid,
            "product_title": product.title,
            "seller_id": user.id,
            "seller_name": user.name,
        });

        let notification = notifications::ActiveModel {
            id: ActiveValue::set(Uuid::new_v4()),
            user_id: ActiveValue::set(follower.follower_id),
            r#type: ActiveValue::set("new_product".to_string()),
            title: ActiveValue::set(format!("{} posted a new product", user.name)),
            message: ActiveValue::set(format!("Check out \"{}\"", product.title)),
            data: ActiveValue::set(Some(notification_data)),
            read: ActiveValue::set(Some(false)),
            created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
        };
        let _ = notification.insert(&ctx.db).await;
    }

    format::json(ProductResponse::from_model(&product))
}

// Update product (seller only)
#[debug_handler]
pub async fn update(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Json(params): Json<UpdateProductParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Model::find_by_pid(&ctx.db, &pid)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if product.seller_id != user.id {
        return unauthorized("You can only update your own listings");
    }

    let mut product_active: products::ActiveModel = product.into();

    if let Some(title) = params.title {
        product_active.title = ActiveValue::set(title);
    }
    if let Some(description) = params.description {
        product_active.description = ActiveValue::set(Some(description));
    }
    if let Some(price) = params.price {
        let decimal_price = loco_rs::prelude::Decimal::from_f64(price)
            .ok_or_else(|| Error::BadRequest("Invalid price".to_string()))?;
        product_active.price = ActiveValue::set(decimal_price);
    }
    if let Some(condition) = params.condition {
        product_active.condition = ActiveValue::set(Some(condition));
    }
    if let Some(location) = params.location {
        product_active.location = ActiveValue::set(Some(location));
    }
    if let Some(category_id) = params.category_id {
        product_active.category_id = ActiveValue::set(Some(category_id));
    }
    if let Some(status) = params.status {
        product_active.status = ActiveValue::set(Some(status));
    }
    if let Some(whatsapp_contact) = params.whatsapp_contact {
        product_active.whatsapp_contact = ActiveValue::set(Some(whatsapp_contact));
    }
    if let Some(phone_contact) = params.phone_contact {
        product_active.phone_contact = ActiveValue::set(Some(phone_contact));
    }

    let updated_product = product_active.update(&ctx.db).await?;
    format::json(ProductResponse::from_model(&updated_product))
}

// Delete product (seller only)
#[debug_handler]
pub async fn delete_product(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Model::find_by_pid(&ctx.db, &pid)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if product.seller_id != user.id {
        return unauthorized("You can only delete your own listings");
    }

    product.delete(&ctx.db).await?;
    format::empty_json()
}

// Mark product as sold (seller only)
#[debug_handler]
pub async fn mark_sold(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Model::find_by_pid(&ctx.db, &pid)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if product.seller_id != user.id {
        return unauthorized("You can only update your own listings");
    }

    let mut product_active: products::ActiveModel = product.into();
    product_active.status = ActiveValue::set(Some("sold".to_string()));
    let updated_product = product_active.update(&ctx.db).await?;

    format::json(ProductResponse::from_model(&updated_product))
}

// Get search suggestions (autocomplete)
#[debug_handler]
pub async fn search_suggestions(
    State(ctx): State<AppContext>,
    Query(params): Query<ProductQueryParams>,
) -> Result<Response> {
    let search_term = params.search.unwrap_or_default();

    if search_term.len() < 2 {
        return format::json(Vec::<String>::new());
    }

    let pattern = format!("%{}%", search_term);

    let suggestions = products::Entity::find()
        .filter(products::Column::Status.eq("active"))
        .filter(
            Condition::any()
                .add(products::Column::Title.like(&pattern))
                .add(products::Column::Description.like(&pattern)),
        )
        .limit(10)
        .all(&ctx.db)
        .await?;

    let mut unique_titles: Vec<String> = suggestions.into_iter().map(|p| p.title).collect();
    unique_titles.dedup();

    format::json(unique_titles)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products")
        .add("/list", get(list))
        .add("/create", post(create))
        .add("/get/{pid}", get(get_by_pid))
        .add("/update/{pid}", put(update))
        .add("/delete/{pid}", delete(delete_product))
        .add("/{pid}/mark-sold", post(mark_sold))
        .add("/search/suggestions", get(search_suggestions))
        .add("/seller/{seller_id}", get(get_seller))
}
