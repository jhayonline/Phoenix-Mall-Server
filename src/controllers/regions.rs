#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::{regions, towns};
use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegionResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub display_order: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TownResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub display_order: i32,
}

// Get all regions
#[utoipa::path(
    get,
    path = "/api/regions",
    responses(
        (status = 200, description = "List of regions", body = Vec<RegionResponse>),
    ),
    tag = "locations"
)]
#[debug_handler]
pub async fn get_regions(State(ctx): State<AppContext>) -> Result<Response> {
    let regions = regions::Entity::find()
        .order_by_asc(regions::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    let response: Vec<RegionResponse> = regions
        .into_iter()
        .map(|r| RegionResponse {
            id: r.id,
            name: r.name,
            display_order: r.display_order.unwrap_or(0),
        })
        .collect();

    format::json(response)
}

// Get towns by region ID
#[utoipa::path(
    get,
    path = "/api/regions/{region_id}/towns",
    params(
        ("region_id" = String, Path, description = "Region UUID")
    ),
    responses(
        (status = 200, description = "List of towns", body = Vec<TownResponse>),
        (status = 404, description = "Region not found")
    ),
    tag = "locations"
)]
#[debug_handler]
pub async fn get_towns_by_region(
    State(ctx): State<AppContext>,
    Path(region_id): Path<uuid::Uuid>,
) -> Result<Response> {
    // Verify region exists
    let region = regions::Entity::find_by_id(region_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let towns = towns::Entity::find()
        .filter(towns::Column::RegionId.eq(region.id))
        .order_by_asc(towns::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    let response: Vec<TownResponse> = towns
        .into_iter()
        .map(|t| TownResponse {
            id: t.id,
            name: t.name,
            display_order: t.display_order.unwrap_or(0),
        })
        .collect();

    format::json(response)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/regions")
        .add("/", get(get_regions))
        .add("/{region_id}/towns", get(get_towns_by_region))
}
