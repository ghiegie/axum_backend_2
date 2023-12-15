use axum::async_trait;

use crate::error_mod::*;
use crate::routers::sales_order_router_mod::get_router_mod::model_mod::Customer;

#[async_trait]
pub trait SalesOrderGet {
    // TODO: Implement
    async fn get_customers(&self) -> Result<String>;

    async fn get_items(&self) -> Result<String>;

    async fn get_color_coats(&self) -> Result<String>;
}
