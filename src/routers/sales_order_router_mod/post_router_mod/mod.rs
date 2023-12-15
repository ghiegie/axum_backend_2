use axum::{Router, routing::post};

use crate::appstate_mod::AppState;
use self::handler_mod::{post_customer, post_sales_order};

pub mod handler_mod;
pub mod trait_mod;

pub fn post_router() -> Router<AppState> {
    Router::new()
        .route("/post_customer", post(post_customer))
        .route("/post_sales_order", post(post_sales_order))
}