use axum::Router;

use crate::{
    appstate_mod::AppState,
    routers::sales_order_router_mod::{
        get_router_mod::get_router,
        post_router_mod::post_router,
    },
};

pub mod get_router_mod;
pub mod post_router_mod;
pub mod model_mod;

pub fn sales_order_router() -> Router<AppState> {
    Router::new()
        .nest("/get_router", get_router())
        .nest("/post_router", post_router())
}
