use axum::async_trait;

use crate::{
    error_mod::*,
    routers::sales_order_router_mod::model_mod::{Customer, SalesOrder}
};

#[async_trait]
pub trait SalesOrderPost {
    async fn post_customer(&self, cust: Customer) -> Result<Customer>;

    async fn post_sales_order(&self, so: SalesOrder) -> Result<SalesOrder>;
}
