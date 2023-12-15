use axum::{routing::get, Router};

use crate::appstate_mod::AppState;

use self::handler_mod::test;

mod handler_mod;

pub fn test_router() -> Router<AppState> {
    Router::new().route("/test", get(test))
}
