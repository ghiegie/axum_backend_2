use odbc_api::{
    sys::{AttrConnectionPooling, AttrCpMatch},
    Environment,
};
use std::sync::Arc;

use crate::error_mod::*;

mod sales_order_impl_mod;

#[derive(Clone, Debug)]
pub struct ModelController {
    con_str: String,
    env: Arc<Environment>,
}

impl ModelController {
    pub fn new(con_str: String) -> Result<Self> {
        Ok(Self {
            con_str,
            env: Arc::new({
                unsafe {
                    Environment::set_connection_pooling(AttrConnectionPooling::DriverAware)
                        .unwrap();
                }

                let mut env = Environment::new().unwrap();
                env.set_connection_pooling_matching(AttrCpMatch::Strict)
                    .unwrap();
                env
            }),
        })
    }
}

impl ModelController {
    fn get_con_str(&self) -> String {
        self.con_str.clone()
    }

    fn get_env(&self) -> Arc<Environment> {
        Arc::clone(&self.env)
    }
}
