#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::{product_reviews, products, users};
use loco_rs::prelude::*;
use sea_orm::{ActiveValue, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateReviewParams {
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub user_name: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

// Submit a review for a product
#[debug_handler]
pub async fn create_review(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(product_pid): Path<String>,
    Json(params): Json<CreateReviewParams>,
) -> Result<Response> {
    // Get the user
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Find the product by PID
    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Check if user already reviewed this product
    let existing_review = product_reviews::Entity::find()
        .filter(product_reviews::Column::ProductId.eq(product.id))
        .filter(product_reviews::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?;

    if existing_review.is_some() {
        return bad_request("You have already reviewed this product");
    }

    // Create the review
    let review = product_reviews::ActiveModel {
        id: ActiveValue::set(Uuid::new_v4()),
        product_id: ActiveValue::set(product.id),
        user_id: ActiveValue::set(user.id),
        rating: ActiveValue::set(params.rating),
        comment: ActiveValue::set(params.comment),
        created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
    }
    .insert(&ctx.db)
    .await?;

    // Update product average rating
    let all_reviews = product_reviews::Entity::find()
        .filter(product_reviews::Column::ProductId.eq(product.id))
        .all(&ctx.db)
        .await?;

    let total_reviews = all_reviews.len() as i32;
    let sum_ratings: i32 = all_reviews.iter().map(|r| r.rating).sum();
    let average_rating = sum_ratings as f64 / total_reviews as f64;

    let mut product_active: products::ActiveModel = product.into();
    product_active.average_rating = ActiveValue::set(Some(average_rating));
    product_active.total_reviews = ActiveValue::set(Some(total_reviews));
    product_active.update(&ctx.db).await?;

    format::json(serde_json::json!({
        "success": true,
        "message": "Review submitted successfully",
        "data": {
            "id": review.id,
            "rating": review.rating,
            "comment": review.comment,
            "created_at": review.created_at,
        }
    }))
}

// Get reviews for a product
#[debug_handler]
pub async fn get_product_reviews(
    State(ctx): State<AppContext>,
    Path(product_pid): Path<String>,
) -> Result<Response> {
    // Find the product by PID
    let product = products::Entity::find()
        .filter(products::Column::Pid.eq(&product_pid))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Get all reviews with user info
    let reviews_with_users = product_reviews::Entity::find()
        .filter(product_reviews::Column::ProductId.eq(product.id))
        .find_also_related(users::Entity)
        .order_by_desc(product_reviews::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    let review_responses: Vec<ReviewResponse> = reviews_with_users
        .into_iter()
        .filter_map(|(review, user_opt)| {
            user_opt.map(|user| ReviewResponse {
                id: review.id,
                rating: review.rating,
                comment: review.comment,
                user_name: user.name,
                created_at: review.created_at.unwrap(),
            })
        })
        .collect();

    format::json(review_responses)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products")
        .add("/{pid}/reviews", post(create_review))
        .add("/{pid}/reviews", get(get_product_reviews))
}
