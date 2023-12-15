use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;

use crate::appstate_mod::AppState;
use crate::constants_mod::CON_STR;
use crate::error_mod::*;
use crate::model_controller::ModelController;
use crate::mw_mod::{main_response_mapper, mw_auth_mod::*};
use crate::routers::{
    login_router_mod::login_router, sales_order_router_mod::sales_order_router,
    test_router_mod::test_router,
};

pub mod login_router_mod;
pub mod sales_order_router_mod;
pub mod test_router_mod;

pub fn main_router() -> Result<Router> {
    let mc = ModelController::new(String::from(CON_STR))?;
    let app_state = AppState::new(mc.clone())?;

    Ok(Router::new()
        //.nest("/test_route", test_router())
        //.nest("/login_route", login_router())
        .nest("/sales_order_route", sales_order_router())
        .with_state(app_state)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new()))
}
