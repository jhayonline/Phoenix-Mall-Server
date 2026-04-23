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
mod m20260415_141543_orders;
mod m20260415_141552_create_order_items;
mod m20260416_095308_add_description_to_categories;
mod m20260417_235637_add_product_reviews;
mod m20260419_001159_add_chat_tables;
mod m20260419_151858_add_follows_and_notifications;
mod m20260422_160649_add_online_status;
mod m20260423_120640_create_category_specs;
mod m20260423_120848_create_product_specs;
mod m20260423_120930_locations;
mod m20260423_121240_alter_products_add_fields;
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
            Box::new(m20260415_141543_orders::Migration),
            Box::new(m20260415_141552_create_order_items::Migration),
            Box::new(m20260416_095308_add_description_to_categories::Migration),
            Box::new(m20260417_235637_add_product_reviews::Migration),
            Box::new(m20260419_001159_add_chat_tables::Migration),
            Box::new(m20260419_151858_add_follows_and_notifications::Migration),
            Box::new(m20260422_160649_add_online_status::Migration),
            Box::new(m20260423_120640_create_category_specs::Migration),
            Box::new(m20260423_120848_create_product_specs::Migration),
            Box::new(m20260423_120930_locations::Migration),
            Box::new(m20260423_121240_alter_products_add_fields::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}