use axum::{routing::post, Router};

use crate::{appstate_mod::AppState, routers::login_router_mod::handler_mod::login};

mod handler_mod;
mod models_mod;

pub fn login_router() -> Router<AppState> {
    Router::new().route("/login", post(login))
}
