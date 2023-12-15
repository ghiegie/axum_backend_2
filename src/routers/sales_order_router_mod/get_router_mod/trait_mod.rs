use axum::async_trait;

use crate::{error_mod::*, routers::sales_order_router_mod::model_mod::{Customer, Item, ColorCoat}};

#[async_trait]
pub trait SalesOrderGet {
    // TODO: Implement
    async fn get_customers(&self) -> Result<Vec<Customer>>;

    async fn get_items(&self) -> Result<Vec<Item>>;

    async fn get_color_coats(&self) -> Result<Vec<ColorCoat>>;
}
