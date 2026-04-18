pub use super::_entities::product_views::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type ProductViews = Entity;

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
    // Add custom methods here
}

impl ActiveModel {}

impl Entity {}
