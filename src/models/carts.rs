pub use super::_entities::carts::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type Carts = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<Self>, DbErr> {
        Entity::find()
            .filter(super::_entities::carts::Column::UserId.eq(user_id))
            .all(db)
            .await
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
