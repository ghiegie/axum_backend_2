use axum::{extract::State, Json};

use crate::routers::sales_order_router_mod::get_router_mod::trait_mod::*;
use crate::{error_mod::*, model_controller::ModelController};

pub async fn get_customers(State(mc): State<ModelController>) -> Result<Json<String>> {
    println!("->> {:<12} - get_customers", "HANDLER");

    let result_str = mc.get_customers().await?;

    Ok(Json(result_str))
}

pub async fn get_items(State(mc): State<ModelController>) -> Result<Json<String>> {
    println!("->> {:<12} - get_items", "HANDLER");

    let result_str = mc.get_items().await?;

    Ok(Json(result_str))
}

pub async fn get_color_coats(State(mc): State<ModelController>) -> Result<Json<String>> {
    println!("->> {:<12} - get_color_coats", "HANDLER");

    let result_str = mc.get_items().await?;

    Ok(Json(result_str))
}
