use crate::models::_entities::{competitor_listings, price_recommendations, products, users};
use loco_rs::prelude::*;
use num_traits::FromPrimitive;
use sea_orm::{ActiveValue, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CompetitorListingData {
    pub product_title: String,
    pub price: rust_decimal::Decimal,
    pub condition: Option<String>,
    pub platform: String,
    pub location: Option<String>,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PriceIntelData {
    pub product_id: Uuid,
    pub product_title: String,
    pub seller_current_price: rust_decimal::Decimal,
    pub market_average_price: rust_decimal::Decimal,
    pub market_median_price: rust_decimal::Decimal,
    pub market_lowest_price: rust_decimal::Decimal,
    pub market_highest_price: rust_decimal::Decimal,
    pub competitor_count: usize,
    pub percentile_25: rust_decimal::Decimal,
    pub percentile_75: rust_decimal::Decimal,
    pub recommendation: String,
    pub analyzed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BatchPriceIntel {
    pub listings: Vec<CompetitorListingData>,
    pub analysis: Vec<PriceIntelData>,
}

// Receive competitor listings data
#[debug_handler]
pub async fn receive_competitor_listings(
    headers: axum::http::HeaderMap,
    State(ctx): State<AppContext>,
    Json(data): Json<BatchPriceIntel>,
) -> Result<Response> {
    // Get API key from header
    let api_key = match headers.get("X-API-Key").and_then(|v| v.to_str().ok()) {
        Some(key) => key,
        None => return unauthorized("API key required"),
    };

    // Verify API key belongs to an admin user
    let _user = match users::Entity::find()
        .filter(users::Column::ApiKey.eq(api_key))
        .filter(users::Column::Role.eq("admin"))
        .one(&ctx.db)
        .await?
    {
        Some(user) => user,
        None => return unauthorized("Invalid API key"),
    };

    let listings_count = data.listings.len();
    let analysis_count = data.analysis.len();

    // Store competitor listings
    for listing in &data.listings {
        let _ = competitor_listings::ActiveModel {
            id: ActiveValue::set(Uuid::new_v4()),
            product_title: ActiveValue::set(listing.product_title.clone()),
            price: ActiveValue::set(listing.price),
            condition: ActiveValue::set(listing.condition.clone()),
            platform: ActiveValue::set(listing.platform.clone()),
            location: ActiveValue::set(listing.location.clone()),
            url: ActiveValue::set(listing.url.clone()),
            scraped_at: ActiveValue::set(chrono::Utc::now().into()),
        }
        .insert(&ctx.db)
        .await?;
    }

    // Store price recommendations and create notifications
    for analysis in &data.analysis {
        // Store recommendation
        let _ = price_recommendations::ActiveModel {
            id: ActiveValue::set(Uuid::new_v4()),
            product_id: ActiveValue::set(analysis.product_id),
            market_avg_price: ActiveValue::set(analysis.market_average_price),
            recommended_price: ActiveValue::set(Some(analysis.market_average_price)),
            competitor_count: ActiveValue::set(analysis.competitor_count as i32),
            is_viewed: ActiveValue::set(Some(false)),
            created_at: ActiveValue::set(chrono::Utc::now().into()),
        }
        .insert(&ctx.db)
        .await?;

        // Create notification for the seller
        let product = products::Entity::find_by_id(analysis.product_id)
            .one(&ctx.db)
            .await?;

        if let Some(product) = product {
            if let Some(seller) = users::Entity::find_by_id(product.seller_id)
                .one(&ctx.db)
                .await?
            {
                let notification_data = serde_json::json!({
                    "product_id": product.id,
                    "product_title": analysis.product_title,
                    "market_avg": analysis.market_average_price,
                    "current_price": analysis.seller_current_price,
                });

                let _ = crate::models::_entities::notifications::ActiveModel {
                    id: ActiveValue::set(Uuid::new_v4()),
                    user_id: ActiveValue::set(seller.id),
                    r#type: ActiveValue::set("price_recommendation".to_string()),
                    title: ActiveValue::set("Price Recommendation Available".to_string()),
                    message: ActiveValue::set(format!(
                        "Market analysis shows similar items average GHS {}. Consider adjusting your price for \"{}\".",
                        analysis.market_average_price, analysis.product_title
                    )),
                    data: ActiveValue::set(Some(notification_data)),
                    read: ActiveValue::set(Some(false)),
                    created_at: ActiveValue::set(Some(chrono::Utc::now().into())),
                }
                .insert(&ctx.db)
                .await?;
            }
        }
    }

    format::json(serde_json::json!({
        "success": true,
        "listings_stored": listings_count,
        "recommendations_created": analysis_count,
    }))
}

// Get price recommendations for a seller's products
#[debug_handler]
pub async fn get_recommendations(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Get all products belonging to this seller
    let products_list = products::Entity::find()
        .filter(products::Column::SellerId.eq(user.id))
        .all(&ctx.db)
        .await?;

    let mut recommendations = Vec::new();

    for product in products_list {
        // Find price recommendations for this product
        let recs = price_recommendations::Entity::find()
            .filter(price_recommendations::Column::ProductId.eq(product.id))
            .all(&ctx.db)
            .await?;

        recommendations.extend(recs);
    }

    format::json(recommendations)
}

// Get market intelligence for a specific product
#[debug_handler]
pub async fn get_market_intel(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(product_id): Path<Uuid>,
) -> Result<Response> {
    // Verify the product belongs to the user or user is admin
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let product = products::Entity::find_by_id(product_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if product.seller_id != user.id && user.role != Some("admin".to_string()) {
        return unauthorized("You don't have permission to view this data");
    }

    // Get competitor listings for this product title
    let competitor_listings_vec: Vec<competitor_listings::Model> =
        competitor_listings::Entity::find()
            .filter(competitor_listings::Column::ProductTitle.contains(&product.title))
            .all(&ctx.db)
            .await?;

    if competitor_listings_vec.is_empty() {
        return format::json(serde_json::json!({
            "product_id": product_id,
            "product_title": product.title,
            "seller_current_price": product.price,
            "market_average_price": product.price,
            "market_median_price": product.price,
            "market_lowest_price": product.price,
            "market_highest_price": product.price,
            "competitor_count": 0,
            "percentile_25": product.price,
            "percentile_75": product.price,
            "recommendation": "Not enough market data for analysis yet",
            "analyzed_at": chrono::Utc::now(),
        }));
    }

    // Calculate statistics
    let mut prices: Vec<rust_decimal::Decimal> =
        competitor_listings_vec.iter().map(|c| c.price).collect();

    prices.sort();

    let count = prices.len();
    let sum: rust_decimal::Decimal = prices.iter().sum();
    let mean = sum / rust_decimal::Decimal::from_usize(count).unwrap();

    let median = if count % 2 == 0 {
        (prices[count / 2 - 1] + prices[count / 2]) / rust_decimal::Decimal::from_u8(2).unwrap()
    } else {
        prices[count / 2]
    };

    // Calculate percentiles (simplified)
    let p25_index = ((count - 1) as f64 * 0.25) as usize;
    let p75_index = ((count - 1) as f64 * 0.75) as usize;
    let percentile_25 = prices[p25_index];
    let percentile_75 = prices[p75_index];

    let recommendation = if product.price > percentile_75 {
        format!(
            "Lower your price to GHS {} to be competitive",
            percentile_75
        )
    } else if product.price < percentile_25 {
        "Your price is very competitive!".to_string()
    } else {
        "Your price is within market range".to_string()
    };

    format::json(serde_json::json!({
        "product_id": product_id,
        "product_title": product.title,
        "seller_current_price": product.price,
        "market_average_price": mean,
        "market_median_price": median,
        "market_lowest_price": prices[0],
        "market_highest_price": prices[count - 1],
        "competitor_count": count,
        "percentile_25": percentile_25,
        "percentile_75": percentile_75,
        "recommendation": recommendation,
        "analyzed_at": chrono::Utc::now(),
    }))
}

// Get competitor listings by product title
#[debug_handler]
pub async fn get_competitor_listings(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Response> {
    let default_title = String::new();
    let title = params.get("title").unwrap_or(&default_title);

    if title.is_empty() {
        let empty: Vec<competitor_listings::Model> = Vec::new();
        return format::json(empty);
    }

    let listings: Vec<competitor_listings::Model> = competitor_listings::Entity::find()
        .filter(competitor_listings::Column::ProductTitle.contains(title))
        .all(&ctx.db)
        .await?;

    format::json(listings)
}

// Mark a recommendation as viewed
#[debug_handler]
pub async fn mark_recommendation_viewed(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(rec_id): Path<Uuid>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let recommendation = price_recommendations::Entity::find_by_id(rec_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Verify this recommendation belongs to a product of this seller
    let product = products::Entity::find_by_id(recommendation.product_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if product.seller_id != user.id && user.role != Some("admin".to_string()) {
        return unauthorized("You don't have permission to modify this");
    }

    let mut active: price_recommendations::ActiveModel = recommendation.into();
    active.is_viewed = ActiveValue::set(Some(true));
    active.update(&ctx.db).await?;

    format::empty_json()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/price-intel")
        .add("/batch", post(receive_competitor_listings))
        .add("/recommendations", get(get_recommendations))
        .add("/market/{product_id}", get(get_market_intel))
        .add("/competitors", get(get_competitor_listings))
        .add(
            "/recommendations/{rec_id}/view",
            put(mark_recommendation_viewed),
        )
}
