use std::sync::Arc;
use axum::async_trait;
use odbc_api::{ConnectionOptions, IntoParameter};

use crate::{
    routers::sales_order_router_mod::{
        post_router_mod::trait_mod::SalesOrderPost, 
        model_mod::{Customer, SalesOrder}
    },
    error_mod::*,
    model_controller::ModelController
};

#[async_trait]
impl SalesOrderPost for ModelController {
    async fn post_customer(&self, cust: Customer) -> Result<Customer> {
        let env = Arc::clone(&self.get_env());
        let conn =
            env.connect_with_connection_string(&self.get_con_str(), ConnectionOptions::default())?;
        println!("pass1");
        
        let query = 
            "insert into CustomerTbl(CustomerID, Name, Address, ContactPerson, TIN, TelNo, Establishment, DelivAddress, Email) values(?, ?, ?, ?, ?, ?, ?, ?, ?)";
        
        println!("{:?}", cust);

        conn.execute(
            query, (
                &cust.get_id().into_parameter(),
                &cust.get_name().into_parameter(),
                &cust.get_addr().into_parameter(),
                &cust.get_cont_pers().into_parameter(),
                &cust.get_tin().into_parameter(),
                &cust.get_tel_no().into_parameter(),
                &cust.get_est().into_parameter(),
                &cust.get_deliv_addr().into_parameter(),
                &cust.get_email().into_parameter(),
            ),
        )?;
        println!("pass2");

        Ok(cust)
    }

    async fn post_sales_order(&self, so: SalesOrder) -> Result<SalesOrder> {
        let env = Arc::clone(&self.get_env());
        let conn =
            env.connect_with_connection_string(&self.get_con_str(), ConnectionOptions::default())?;
        
        let query = 
            "insert into SalesOrderTbl(SalesOrderID, CustomerID, ProdOrderID, ItemID, ItemDesc, Qty, ProdCompletionDate, DelivCommitDate, SalesReqDate, ColorCoatID) values(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        
            conn.execute(
                query, (
                    &so.get_id().into_parameter(),
                    &so.get_cust_id().into_parameter(),
                    &so.get_prd_ord_id().into_parameter(),
                    &so.get_item_id().into_parameter(),
                    &so.get_item_desc().into_parameter(),
                    &so.get_qty(),
                    &so.get_prd_comp_date().into_parameter(),
                    &so.get_deliv_commt_date().into_parameter(),
                    &so.get_sales_req_date().into_parameter(),
                    &so.get_color_coat_id().into_parameter(),
                ),
            )?;

        Ok(so)
    }
}