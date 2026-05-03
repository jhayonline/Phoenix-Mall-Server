#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::products;
use crate::models::_entities::{favorites, follows, notifications, product_reviews};
use crate::models::{
    _entities::users,
    products::{PaginatedProductsResponse, ProductQueryParams},
};
use loco_rs::prelude::*;
use nanoid::nanoid;
use num_traits::cast::{FromPrimitive, ToPrimitive};
use sea_orm::{Condition, PaginatorTrait, QuerySelect};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::_entities::categories;
use crate::models::_entities::category_specs;
use crate::models::_entities::product_specs;
use crate::models::_entities::regions;
use crate::models::_entities::towns;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductSpecResponse {
    pub spec_id: uuid::Uuid,
    pub spec_name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateProductParamsV2 {
    pub title: String,
    pub description: Option<String>,
    pub category_id: uuid::Uuid,
    pub region_id: uuid::Uuid,
    pub town_id: uuid::Uuid,
    pub price: f64,
    pub negotiation: String,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub whatsapp_contact: Option<bool>,
    pub phone_contact: Option<bool>,
    pub specs: Vec<ProductSpecParam>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProductSpecParam {
    pub spec_id: uuid::Uuid,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateProductParamsV2 {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub category_id: Option<uuid::Uuid>,
    pub status: Option<String>,
    pub whatsapp_contact: Option<bool>,
    pub phone_contact: Option<bool>,
    pub region_id: Option<uuid::Uuid>,
    pub town_id: Option<uuid::Uuid>,
    pub negotiation: Option<String>,
    pub specs: Option<Vec<ProductSpecParam>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductV2Response {
    pub id: uuid::Uuid,
    pub pid: String,
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub category_id: Option<uuid::Uuid>,
    pub seller_id: i32,
    pub status: Option<String>,
    pub whatsapp_contact: Option<bool>,
    pub phone_contact: Option<bool>,
    pub views_count: Option<i32>,
    pub negotiation: String,
    pub promotion_type: String,
    pub region_id: Option<uuid::Uuid>,
    pub town_id: Option<uuid::Uuid>,
    pub specs: Vec<ProductSpecResponse>,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

// ============ API Endpoints ============

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

#[utoipa::path(
    get,
    path = "/api/products/seller/{seller_id}",
    params(
        ("seller_id" = i32, Path, description = "Seller user ID")
    ),
    responses(
        (status = 200, description = "Seller information", body = serde_json::Value),
        (status = 404, description = "Seller not found")
    ),
    tag = "products"
)]
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

#[utoipa::path(
    get,
    path = "/api/products/get/{pid}",
    params(
        ("pid" = String, Path, description = "Product public ID")
    ),
    responses(
        (status = 200, description = "Product found", body = ProductV2Response),
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

    let product_id = product.id;
    let product_pid = product.pid.clone();
    let product_title = product.title.clone();
    let product_description = product.description.clone();
    let product_condition = product.condition.clone();
    let product_location = product.location.clone();
    let product_price = product.price;
    let product_category_id = product.category_id;
    let product_seller_id = product.seller_id;
    let product_status = product.status.clone();
    let product_whatsapp_contact = product.whatsapp_contact;
    let product_phone_contact = product.phone_contact;
    let product_views_count = product.views_count;
    let product_negotiation = product.negotiation.clone();
    let product_promotion_type = product.promotion_type.clone();
    let product_region_id = product.region_id;
    let product_town_id = product.town_id;
    let product_created_at = product.created_at;

    let _wishlist_count = favorites::Entity::find()
        .filter(favorites::Column::ProductId.eq(product_id))
        .count(&ctx.db)
        .await?;

    let reviews = product_reviews::Entity::find()
        .filter(product_reviews::Column::ProductId.eq(product_id))
        .all(&ctx.db)
        .await?;

    let (_average_rating, _total_reviews) = if reviews.is_empty() {
        (0.0, 0)
    } else {
        let sum: i32 = reviews.iter().map(|r| r.rating).sum();
        let avg = sum as f64 / reviews.len() as f64;
        (avg, reviews.len())
    };

    let product_specs_list: Vec<(product_specs::Model, Option<category_specs::Model>)> =
        product_specs::Entity::find()
            .filter(product_specs::Column::ProductId.eq(product_id))
            .find_also_related(category_specs::Entity)
            .all(&ctx.db)
            .await?;

    let specs: Vec<ProductSpecResponse> = product_specs_list
        .into_iter()
        .filter_map(|(ps, spec_opt)| {
            spec_opt.map(|spec| ProductSpecResponse {
                spec_id: ps.spec_id,
                spec_name: spec.spec_name,
                value: ps.spec_value,
            })
        })
        .collect();

    let _region_name = if let Some(region_id) = product_region_id {
        regions::Entity::find_by_id(region_id)
            .one(&ctx.db)
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    let _town_name = if let Some(town_id) = product_town_id {
        towns::Entity::find_by_id(town_id)
            .one(&ctx.db)
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    // Increment view count
    let current_views = product_views_count.unwrap_or(0);

    let product_to_update = products::Entity::find_by_id(product_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let mut active_product: products::ActiveModel = product_to_update.into();
    active_product.views_count = ActiveValue::set(Some(current_views + 1));
    let _ = active_product.update(&ctx.db).await;

    format::json(ProductV2Response {
        id: product_id,
        pid: product_pid,
        title: product_title,
        description: product_description,
        price: product_price.to_f64().unwrap_or(0.0),
        condition: product_condition,
        location: product_location,
        category_id: product_category_id,
        seller_id: product_seller_id,
        status: product_status,
        whatsapp_contact: product_whatsapp_contact,
        phone_contact: product_phone_contact,
        views_count: Some(current_views + 1),
        negotiation: product_negotiation.unwrap_or_else(|| "negotiable".to_string()),
        promotion_type: product_promotion_type.unwrap_or_else(|| "standard".to_string()),
        region_id: product_region_id,
        town_id: product_town_id,
        specs,
        created_at: product_created_at,
    })
}

#[utoipa::path(
    post,
    path = "/api/products/create",
    security(("bearer_auth" = [])),
    request_body = CreateProductParamsV2,
    responses(
        (status = 200, description = "Product created successfully", body = ProductV2Response),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Category not found")
    ),
    tag = "products"
)]
#[debug_handler]
pub async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateProductParamsV2>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let category = categories::Entity::find_by_id(params.category_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let pid = nanoid!(21);
    let product_id = uuid::Uuid::new_v4();

    let decimal_price = loco_rs::prelude::Decimal::from_f64(params.price)
        .ok_or_else(|| Error::BadRequest("Invalid price".to_string()))?;

    let _product = products::ActiveModel {
        id: ActiveValue::set(product_id),
        pid: ActiveValue::set(pid.clone()),
        title: ActiveValue::set(params.title.clone()),
        description: ActiveValue::set(params.description.clone()),
        price: ActiveValue::set(decimal_price),
        condition: ActiveValue::set(params.condition.clone()),
        location: ActiveValue::set(params.location.clone()),
        category_id: ActiveValue::set(Some(params.category_id)),
        seller_id: ActiveValue::set(user.id),
        status: ActiveValue::set(Some("active".to_string())),
        whatsapp_contact: ActiveValue::set(params.whatsapp_contact),
        phone_contact: ActiveValue::set(params.phone_contact),
        views_count: ActiveValue::set(Some(0)),
        region_id: ActiveValue::set(Some(params.region_id)),
        town_id: ActiveValue::set(Some(params.town_id)),
        negotiation: ActiveValue::set(Some(params.negotiation.clone())),
        promotion_type: ActiveValue::set(Some("standard".to_string())),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;

    for spec_param in &params.specs {
        let spec = category_specs::Entity::find_by_id(spec_param.spec_id)
            .one(&ctx.db)
            .await?;

        if let Some(spec) = spec {
            if spec.category_id == category.id {
                let product_spec = product_specs::ActiveModel {
                    id: ActiveValue::set(uuid::Uuid::new_v4()),
                    product_id: ActiveValue::set(product_id),
                    spec_id: ActiveValue::set(spec_param.spec_id),
                    spec_value: ActiveValue::set(spec_param.value.clone()),
                    created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
                };
                product_spec.insert(&ctx.db).await?;
            }
        }
    }

    let followers_list = follows::Entity::find()
        .filter(follows::Column::FollowingId.eq(user.id))
        .all(&ctx.db)
        .await?;

    for follower in followers_list {
        let notification_data = serde_json::json!({
            "product_id": product_id,
            "product_pid": pid,
            "product_title": params.title,
            "seller_id": user.id,
            "seller_name": user.name,
        });

        let notification = notifications::ActiveModel {
            id: ActiveValue::set(Uuid::new_v4()),
            user_id: ActiveValue::set(follower.follower_id),
            r#type: ActiveValue::set("new_product".to_string()),
            title: ActiveValue::set(format!("{} posted a new product", user.name)),
            message: ActiveValue::set(format!("Check out \"{}\"", params.title)),
            data: ActiveValue::set(Some(notification_data)),
            read: ActiveValue::set(Some(false)),
            created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
        };
        let _ = notification.insert(&ctx.db).await;
    }

    let product_specs_list: Vec<(product_specs::Model, Option<category_specs::Model>)> =
        product_specs::Entity::find()
            .filter(product_specs::Column::ProductId.eq(product_id))
            .find_also_related(category_specs::Entity)
            .all(&ctx.db)
            .await?;

    let specs_response: Vec<ProductSpecResponse> = product_specs_list
        .into_iter()
        .filter_map(|(product_spec, spec_opt)| {
            spec_opt.map(|spec| ProductSpecResponse {
                spec_id: product_spec.spec_id,
                spec_name: spec.spec_name,
                value: product_spec.spec_value.clone(),
            })
        })
        .collect();

    format::json(ProductV2Response {
        id: product_id,
        pid,
        title: params.title,
        description: params.description,
        price: params.price,
        condition: params.condition,
        location: params.location,
        category_id: Some(params.category_id),
        seller_id: user.id,
        status: Some("active".to_string()),
        whatsapp_contact: params.whatsapp_contact,
        phone_contact: params.phone_contact,
        views_count: Some(0),
        negotiation: params.negotiation,
        promotion_type: "standard".to_string(),
        region_id: Some(params.region_id),
        town_id: Some(params.town_id),
        specs: specs_response,
        created_at: Some(chrono::Utc::now().into()),
    })
}

#[utoipa::path(
    put,
    path = "/api/products/update/{pid}",
    security(("bearer_auth" = [])),
    params(
        ("pid" = String, Path, description = "Product public ID")
    ),
    request_body = UpdateProductParamsV2,
    responses(
        (status = 200, description = "Product updated successfully", body = ProductV2Response),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Not your product"),
        (status = 404, description = "Product not found")
    ),
    tag = "products"
)]
#[debug_handler]
pub async fn update(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Json(params): Json<UpdateProductParamsV2>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let product = products::Model::find_by_pid(&ctx.db, &pid)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if product.seller_id != user.id {
        return unauthorized("You can only update your own listings");
    }

    let product_id = product.id;
    let product_pid = product.pid.clone();

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
    if let Some(region_id) = params.region_id {
        product_active.region_id = ActiveValue::set(Some(region_id));
    }
    if let Some(town_id) = params.town_id {
        product_active.town_id = ActiveValue::set(Some(town_id));
    }
    if let Some(negotiation) = params.negotiation {
        product_active.negotiation = ActiveValue::set(Some(negotiation));
    }

    let updated_product = product_active.update(&ctx.db).await?;

    if let Some(specs) = params.specs {
        product_specs::Entity::delete_many()
            .filter(product_specs::Column::ProductId.eq(product_id))
            .exec(&ctx.db)
            .await?;

        for spec_param in specs {
            let product_spec = product_specs::ActiveModel {
                id: ActiveValue::set(uuid::Uuid::new_v4()),
                product_id: ActiveValue::set(product_id),
                spec_id: ActiveValue::set(spec_param.spec_id),
                spec_value: ActiveValue::set(spec_param.value),
                created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
            };
            let _ = product_spec.insert(&ctx.db).await;
        }
    }

    let product_specs_list: Vec<(product_specs::Model, Option<category_specs::Model>)> =
        product_specs::Entity::find()
            .filter(product_specs::Column::ProductId.eq(product_id))
            .find_also_related(category_specs::Entity)
            .all(&ctx.db)
            .await?;

    let specs_response: Vec<ProductSpecResponse> = product_specs_list
        .into_iter()
        .filter_map(|(ps, spec_opt)| {
            spec_opt.map(|spec| ProductSpecResponse {
                spec_id: ps.spec_id,
                spec_name: spec.spec_name,
                value: ps.spec_value,
            })
        })
        .collect();

    format::json(ProductV2Response {
        id: product_id,
        pid: product_pid,
        title: updated_product.title,
        description: updated_product.description,
        price: updated_product.price.to_f64().unwrap_or(0.0),
        condition: updated_product.condition,
        location: updated_product.location,
        category_id: updated_product.category_id,
        seller_id: updated_product.seller_id,
        status: updated_product.status,
        whatsapp_contact: updated_product.whatsapp_contact,
        phone_contact: updated_product.phone_contact,
        views_count: updated_product.views_count,
        negotiation: updated_product
            .negotiation
            .unwrap_or_else(|| "negotiable".to_string()),
        promotion_type: updated_product
            .promotion_type
            .unwrap_or_else(|| "standard".to_string()),
        region_id: updated_product.region_id,
        town_id: updated_product.town_id,
        specs: specs_response,
        created_at: updated_product.created_at,
    })
}

#[utoipa::path(
    delete,
    path = "/api/products/delete/{pid}",
    security(("bearer_auth" = [])),
    params(
        ("pid" = String, Path, description = "Product public ID")
    ),
    responses(
        (status = 200, description = "Product deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Not your product"),
        (status = 404, description = "Product not found")
    ),
    tag = "products"
)]
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

#[utoipa::path(
    post,
    path = "/api/products/{pid}/mark-sold",
    security(("bearer_auth" = [])),
    params(
        ("pid" = String, Path, description = "Product public ID")
    ),
    responses(
        (status = 200, description = "Product marked as sold", body = ProductV2Response),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Not your product"),
        (status = 404, description = "Product not found")
    ),
    tag = "products"
)]
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

    let product_id = product.id;
    let product_pid = product.pid.clone();
    let product_title = product.title.clone();
    let product_description = product.description.clone();
    let product_price = product.price;
    let product_condition = product.condition.clone();
    let product_location = product.location.clone();
    let product_category_id = product.category_id;
    let product_seller_id = product.seller_id;
    let product_whatsapp_contact = product.whatsapp_contact;
    let product_phone_contact = product.phone_contact;
    let product_views_count = product.views_count;
    let product_negotiation = product.negotiation.clone();
    let product_promotion_type = product.promotion_type.clone();
    let product_region_id = product.region_id;
    let product_town_id = product.town_id;
    let product_created_at = product.created_at;

    let mut product_active: products::ActiveModel = product.into();
    product_active.status = ActiveValue::set(Some("sold".to_string()));
    let _ = product_active.update(&ctx.db).await;

    format::json(ProductV2Response {
        id: product_id,
        pid: product_pid,
        title: product_title,
        description: product_description,
        price: product_price.to_f64().unwrap_or(0.0),
        condition: product_condition,
        location: product_location,
        category_id: product_category_id,
        seller_id: product_seller_id,
        status: Some("sold".to_string()),
        whatsapp_contact: product_whatsapp_contact,
        phone_contact: product_phone_contact,
        views_count: product_views_count,
        negotiation: product_negotiation.unwrap_or_else(|| "negotiable".to_string()),
        promotion_type: product_promotion_type.unwrap_or_else(|| "standard".to_string()),
        region_id: product_region_id,
        town_id: product_town_id,
        specs: vec![],
        created_at: product_created_at,
    })
}

#[utoipa::path(
    get,
    path = "/api/products/search/suggestions",
    params(
        ("search" = Option<String>, Query, description = "Search term")
    ),
    responses(
        (status = 200, description = "Search suggestions", body = Vec<String>),
        (status = 400, description = "Search term too short")
    ),
    tag = "products"
)]
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

// Also add endpoint to get category product counts
#[utoipa::path(
    get,
    path = "/api/categories/counts",
    responses(
        (status = 200, description = "Category product counts", body = serde_json::Value),
    ),
    tag = "categories"
)]
#[debug_handler]
pub async fn get_category_counts(State(ctx): State<AppContext>) -> Result<Response> {
    use sea_orm::QuerySelect;

    // Get all active products with their category IDs
    let products_with_categories = products::Entity::find()
        .filter(products::Column::Status.eq("active"))
        .select_only()
        .column(products::Column::CategoryId)
        .all(&ctx.db)
        .await?;

    // Count products per category
    let mut counts: std::collections::HashMap<uuid::Uuid, i64> = std::collections::HashMap::new();
    for product in products_with_categories {
        if let Some(category_id) = product.category_id {
            *counts.entry(category_id).or_insert(0) += 1;
        }
    }

    format::json(counts)
}

#[debug_handler]
pub async fn get_related_products(
    State(ctx): State<AppContext>,
    Path(product_pid): Path<String>,
) -> Result<Response> {
    // Find the current product
    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Extract keywords from product title (e.g., "iPhone 14 Pro Max" -> "iPhone")
    let title_keywords: Vec<&str> = product.title.split_whitespace().collect();
    let mut search_terms = Vec::new();

    // Use the first meaningful word (brand name) and maybe the model
    if !title_keywords.is_empty() {
        // Add brand name (first word)
        search_terms.push(title_keywords[0].to_string());

        // If there's a model number like "14" or "Pro", add it too
        for word in title_keywords.iter().skip(1).take(2) {
            if word.len() > 1 && !word.chars().all(|c| c.is_ascii_digit()) {
                search_terms.push(word.to_string());
                break;
            }
        }
    }

    // Build search condition
    let mut condition = Condition::any();
    for term in search_terms {
        condition = condition.add(products::Column::Title.contains(&term));
    }

    // Find related products by title similarity, excluding current product
    let related_products = products::Entity::find()
        .filter(condition)
        .filter(products::Column::Pid.ne(&product_pid))
        .filter(products::Column::Status.eq("active"))
        .limit(6)
        .all(&ctx.db)
        .await?;

    // If not enough results, fall back to category-based products
    if related_products.len() < 3 && product.category_id.is_some() {
        let category_products = products::Entity::find()
            .filter(products::Column::CategoryId.eq(product.category_id.unwrap()))
            .filter(products::Column::Pid.ne(&product_pid))
            .filter(products::Column::Status.eq("active"))
            .limit(6)
            .all(&ctx.db)
            .await?;

        // Merge and deduplicate
        let mut all_products = related_products;
        for cat_prod in category_products {
            if !all_products.iter().any(|p| p.pid == cat_prod.pid) {
                all_products.push(cat_prod);
            }
        }
        all_products.truncate(6);

        return format::json(all_products);
    }

    format::json(related_products)
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
        .add("/categories/counts", get(get_category_counts))
        .add("/{pid}/related", get(get_related_products))
}
