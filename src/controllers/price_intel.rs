use crate::models::_entities::{competitor_listings, price_recommendations, products, users};
use loco_rs::prelude::*;
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
    let user = match users::Entity::find()
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

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/price-intel")
        .add("/batch", post(receive_competitor_listings))
}
