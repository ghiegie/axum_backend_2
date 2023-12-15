use axum::async_trait;
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer},
    ConnectionOptions, Cursor,
};
use std::sync::Arc;

use crate::{error_mod::*, routers::sales_order_router_mod::model_mod::{Customer, Item, ColorCoat}};
use crate::{
    model_controller::ModelController,
    routers::sales_order_router_mod::get_router_mod::trait_mod::SalesOrderGet,
};

#[async_trait]
impl SalesOrderGet for ModelController {
    async fn get_customers(&self) -> Result<Vec<Customer>> {
        let env = Arc::clone(&self.get_env());
        let conn =
            env.connect_with_connection_string(&self.get_con_str(), ConnectionOptions::default())?;

        let buff_size = 100;
        let mut buff =
            ColumnarAnyBuffer::from_descs(buff_size, [BufferDesc::Text { max_str_len: 50 }; 9]);

        let mut init_val = 0;
        let mut looping_vec = Vec::new();
        loop {
            // create sql query that fetches a specific rows of db
            let sql_query = format!("select * from CustomerTbl order by CURRENT_TIMESTAMP offset {} rows fetch next {} rows only", init_val, buff_size);

            let option_cursor = conn.execute(&sql_query, ())?;

            let cursor = option_cursor.ok_or(Error::OdbcError)?;

            let mut buff_cursor = cursor.bind_buffer(&mut buff)?;
            //.expect("ERROR: FAILED TO CREATE BUFFER-CURSOR");

            let fetch = buff_cursor.fetch()?;
            if let Some(fetch_data) = fetch {
                for i in 0..fetch_data.num_cols() {
                    looping_vec.push(Vec::new());
                    let col = fetch_data.column(i);
                    let stream = col.as_text_view().ok_or(Error::OdbcError)?;

                    for optional_stream_slice in stream.iter() {
                        let stream_slice = optional_stream_slice.ok_or(Error::OdbcError)?;
                        let str_result = String::from_utf8(stream_slice.to_vec())?;
                        looping_vec[i].push(str_result);
                    }
                }
            } else {
                break;
            }

            init_val += buff_size;
        }

        let mut res_vec = Vec::new();
        if looping_vec.len() != 0 {
            // TODO Implement Logic on Creatin Customer
            for i in 0..looping_vec[0].len() {
                let id = looping_vec[0][i].clone();
                let name = looping_vec[1][i].clone();
                let addr = looping_vec[2][i].clone();
                let cont_pers = looping_vec[3][i].clone();
                let tin = looping_vec[4][i].clone();
                let tel_no = looping_vec[5][i].clone();
                let est = looping_vec[6][i].clone();
                let deliv_addr =looping_vec[7][i].clone();
                let email = looping_vec[8][i].clone();

                let cust = Customer::new(id, name, addr, cont_pers, tin, tel_no, est, deliv_addr, email);
                res_vec.push(cust);
            }
        }        

        Ok(res_vec)
    }

    async fn get_items(&self) -> Result<Vec<Item>> {
        let env = Arc::clone(&self.get_env());
        let conn =
            env.connect_with_connection_string(&self.get_con_str(), ConnectionOptions::default())?;

        let buff_size = 100;
        let mut buff =
            ColumnarAnyBuffer::from_descs(buff_size, [BufferDesc::Text { max_str_len: 50 }; 2]);

        let mut init_val = 0;
        let mut looping_vec = Vec::new();
        loop {
            let sql_query = format!("select * from ItemTbl order by CURRENT_TIMESTAMP offset {} rows fetch next {} rows only", init_val, buff_size);

            let option_cursor = conn.execute(&sql_query, ())?;

            let cursor = option_cursor.ok_or(Error::OdbcError)?;

            let mut buff_cursor = cursor.bind_buffer(&mut buff)?;
            //.expect("ERROR: FAILED TO CREATE BUFFER-CURSOR");

            let fetch = buff_cursor.fetch()?;
            if let Some(fetch_data) = fetch {
                for i in 0..fetch_data.num_cols() {
                    looping_vec.push(Vec::new());
                    let col = fetch_data.column(i);
                    let stream = col.as_text_view().ok_or(Error::OdbcError)?;

                    for optional_stream_slice in stream.iter() {
                        let stream_slice = optional_stream_slice.ok_or(Error::OdbcError)?;
                        let str_result = String::from_utf8(stream_slice.to_vec())?;
                        looping_vec[i].push(str_result);
                    }
                }
            } else {
                break;
            }

            init_val += buff_size;
        }

        let mut res_vec = Vec::new();
        if looping_vec.len() != 0 {
            // TODO Implement Logic on Creatin Customer
            for i in 0..looping_vec[0].len() {
                let id = looping_vec[0][i].clone();
                let name = looping_vec[1][i].clone();

                let item = Item::new(id, name);
                res_vec.push(item);
            }
        }              

        Ok(res_vec)
    }

    async fn get_color_coats(&self) -> Result<Vec<ColorCoat>> {
        let env = Arc::clone(&self.get_env());
        let conn =
            env.connect_with_connection_string(&self.get_con_str(), ConnectionOptions::default())?;

        let buff_size = 100;
        let mut buff =
            ColumnarAnyBuffer::from_descs(buff_size, [BufferDesc::Text { max_str_len: 50 }; 2]);

        let mut init_val = 0;
        let mut looping_vec = Vec::new();
        loop {
            let sql_query = format!("select * from ColorCoatTbl order by CURRENT_TIMESTAMP offset {} rows fetch next {} rows only", init_val, buff_size);

            let option_cursor = conn.execute(&sql_query, ())?;

            let cursor = option_cursor.ok_or(Error::OdbcError)?;

            let mut buff_cursor = cursor.bind_buffer(&mut buff)?;
            //.expect("ERROR: FAILED TO CREATE BUFFER-CURSOR");

            let fetch = buff_cursor.fetch()?;
            if let Some(fetch_data) = fetch {
                for i in 0..fetch_data.num_cols() {
                    looping_vec.push(Vec::new());
                    let col = fetch_data.column(i);
                    let stream = col.as_text_view().ok_or(Error::OdbcError)?;

                    for optional_stream_slice in stream.iter() {
                        let stream_slice = optional_stream_slice.ok_or(Error::OdbcError)?;
                        let str_result = String::from_utf8(stream_slice.to_vec())?;
                        looping_vec[i].push(str_result);
                    }
                }
            } else {
                break;
            }

            init_val += buff_size;
        }

        let mut res_vec = Vec::new();
        if looping_vec.len() != 0 {
            for i in 0..looping_vec.len() {
                let id = looping_vec[0][i].clone();
                let color_coat = looping_vec[1][i].clone();
    
                let color_coats = ColorCoat::new(id, color_coat);
                res_vec.push(color_coats);
            }
        }

        Ok(res_vec)
    }
}
