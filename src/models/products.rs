use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

pub use super::_entities::products::{ActiveModel, Entity, Model};
pub type Products = Entity;

use crate::models::_entities::categories;
use crate::views::product_response::ProductResponse;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateProductParams {
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub category_id: Option<Uuid>,
    pub whatsapp_contact: Option<bool>,
    pub phone_contact: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateProductParams {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub category_id: Option<Uuid>,
    pub status: Option<String>,
    pub whatsapp_contact: Option<bool>,
    pub phone_contact: Option<bool>,
}

#[derive(Debug, Deserialize, Default, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProductQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub sort: Option<String>,
    pub category: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub location: Option<String>,
    pub condition: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedProductsResponse {
    pub items: Vec<ProductResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

impl Model {
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> Result<Option<Self>, DbErr> {
        Entity::find()
            .filter(super::_entities::products::Column::Pid.eq(pid))
            .one(db)
            .await
    }
}

impl ActiveModel {}

impl Entity {
    pub async fn paginate(
        db: &DatabaseConnection,
        params: &ProductQueryParams,
    ) -> Result<PaginatedProductsResponse, DbErr> {
        let page = params.page.unwrap_or(1);
        let per_page = params.limit.unwrap_or(20).min(100);
        let offset = (page - 1) * per_page;

        let mut query =
            Entity::find().filter(super::_entities::products::Column::Status.eq("active"));

        // Category filter
        if let Some(category_slug) = &params.category {
            let category = categories::Entity::find()
                .filter(categories::Column::Slug.eq(category_slug))
                .one(db)
                .await?;

            if let Some(cat) = category {
                query = query.filter(super::_entities::products::Column::CategoryId.eq(cat.id));
            }
        }

        // Price range filter
        if let Some(min_price) = params.min_price {
            let price_str = format!("{:.2}", min_price);
            if let Ok(min_decimal) = price_str.parse::<loco_rs::prelude::Decimal>() {
                query = query.filter(super::_entities::products::Column::Price.gte(min_decimal));
            }
        }

        if let Some(max_price) = params.max_price {
            let price_str = format!("{:.2}", max_price);
            if let Ok(max_decimal) = price_str.parse::<loco_rs::prelude::Decimal>() {
                query = query.filter(super::_entities::products::Column::Price.lte(max_decimal));
            }
        }

        // Location filter
        if let Some(location) = &params.location {
            query = query.filter(super::_entities::products::Column::Location.contains(location));
        }

        // Condition filter
        if let Some(condition) = &params.condition {
            query = query.filter(super::_entities::products::Column::Condition.eq(condition));
        }

        // Search filter
        if let Some(search_term) = &params.search {
            use sea_orm::sea_query::Cond;
            query = query.filter(
                Cond::any()
                    .add(super::_entities::products::Column::Title.contains(search_term))
                    .add(super::_entities::products::Column::Description.contains(search_term)),
            );
        }

        // Sorting
        match params.sort.as_deref() {
            Some("price_asc") => {
                query = query.order_by_asc(super::_entities::products::Column::Price)
            }
            Some("price_desc") => {
                query = query.order_by_desc(super::_entities::products::Column::Price)
            }
            Some("oldest") => {
                query = query.order_by_asc(super::_entities::products::Column::CreatedAt)
            }
            Some("most_viewed") => {
                query = query.order_by_desc(super::_entities::products::Column::ViewsCount)
            }
            _ => query = query.order_by_desc(super::_entities::products::Column::CreatedAt),
        }

        let total = query.clone().count(db).await?;

        let items = query.limit(per_page).offset(offset).all(db).await?;

        let total_pages = (total as f64 / per_page as f64).ceil() as u64;

        let response_items: Vec<ProductResponse> = items
            .iter()
            .map(|item| ProductResponse::from_model(item))
            .collect();

        Ok(PaginatedProductsResponse {
            items: response_items,
            total,
            page,
            per_page,
            total_pages,
        })
    }
}
