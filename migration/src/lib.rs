#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20260413_083123_extend_users_table;
mod m20260413_083210_create_user_session;
mod m20260413_083227_categories;
mod m20260413_083240_products;
mod m20260413_083250_create_product_images;
mod m20260413_083302_favorites;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260413_083123_extend_users_table::Migration),
            Box::new(m20260413_083210_create_user_session::Migration),
            Box::new(m20260413_083227_categories::Migration),
            Box::new(m20260413_083240_products::Migration),
            Box::new(m20260413_083250_create_product_images::Migration),
            Box::new(m20260413_083302_favorites::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}