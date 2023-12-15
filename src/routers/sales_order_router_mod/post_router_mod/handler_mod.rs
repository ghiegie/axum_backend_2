use axum::{extract::State, Json};
use serde_json::{Value, json};

use crate::{
    model_controller::ModelController, 
    routers::sales_order_router_mod::{
        model_mod::{Customer, SalesOrder},
        post_router_mod::trait_mod::SalesOrderPost
    },
    error_mod::*,
};

pub async fn post_customer(State(mc): State<ModelController>, Json(cust): Json<Customer>) -> Result<Json<Customer>> {
    println!("->> {:<12} - post_customer", "HANDLER");

    let result = mc.post_customer(cust).await?;

    Ok(Json(result))
}

pub async fn post_sales_order(State(mc): State<ModelController>, Json(so): Json<SalesOrder>) -> Result<Json<Value>> {
    println!("->> {:<12} - post_sales_order", "HANDLER");

    mc.post_sales_order(so).await?;

    Ok(Json(json!({
        "result": {
            "success": true
        }
    })))
}