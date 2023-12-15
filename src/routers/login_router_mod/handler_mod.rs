use axum::Json;
use serde_json::json;
use serde_json::Value;
use tower_cookies::{Cookie, Cookies};

use crate::constants_mod::*;
use crate::error_mod::*;
use crate::routers::login_router_mod::models_mod::LoginPayload;

pub async fn login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - loginRoute_login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.get_user() != "user" || payload.get_pwd() != "pwd" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    Ok(Json(json!({
        "result": {
            "success": true
        }
    })))
}
