#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::categories::{self, Entity as Categories};
use loco_rs::prelude::*;

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let categories = Categories::find().all(&ctx.db).await?;

    format::json(categories)
}

#[debug_handler]
pub async fn get_by_slug(
    State(ctx): State<AppContext>,
    Path(slug): Path<String>,
) -> Result<Response> {
    let categories = Categories::find()
        .filter(categories::Column::Slug.eq(slug))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    format::json(categories)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/categories/")
        .add("/list", get(list))
        .add("/{slug}", get(get_by_slug))
}
