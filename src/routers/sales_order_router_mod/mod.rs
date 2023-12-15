use axum::Router;

use crate::{
    appstate_mod::AppState, model_controller::ModelController,
    routers::sales_order_router_mod::get_router_mod::get_router,
};

pub mod get_router_mod;

pub fn sales_order_router() -> Router<AppState> {
    Router::new().nest("/get_router", get_router())
}
