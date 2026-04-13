use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use super::_entities::products::{ActiveModel, Entity, Model};
pub type Products = Entity;

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> Result<Option<Self>, DbErr> {
        Entity::find()
            .filter(super::_entities::products::Column::Pid.eq(pid))
            .one(db)
            .await
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
