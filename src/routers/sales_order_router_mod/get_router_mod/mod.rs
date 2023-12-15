use axum::{routing::get, Router};

use crate::{
    appstate_mod::AppState, model_controller::ModelController,
    routers::sales_order_router_mod::get_router_mod::handler_mod::*,
};

mod handler_mod;
pub mod model_mod;
pub mod trait_mod;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_customers", get(get_customers))
        .route("/get_items", get(get_items))
        .route("/get_color_coats", get(get_color_coats))
}
