use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
    id: String,
    name: String,
    addr: String,
    cont_pers: String,
    tin: String,
    tel_no: String,
    est: String,
    deliv_addr: String,
    email: String,
}

impl Customer {
    pub fn new(
        id: String,
        name: String,
        addr: String,
        cont_pers: String,
        tin: String,
        tel_no: String,
        est: String,
        deliv_addr: String,
        email: String,
    ) -> Self {
        Self {
            id,
            name,
            addr,
            cont_pers,
            tin,
            tel_no,
            est,
            deliv_addr,
            email,
        }
    }
}

impl Customer {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }

    pub fn get_cont_pers(&self) -> String {
        self.cont_pers.clone()
    }

    pub fn get_tin(&self) -> String {
        self.tin.clone()
    }

    pub fn get_tel_no(&self) -> String {
        self.tel_no.clone()
    }

    pub fn get_est(&self) -> String {
        self.est.clone()
    }

    pub fn get_deliv_addr(&self) -> String {
        self.deliv_addr.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    id: String,
    name: String,
}

impl Item {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

impl Item {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorCoat {
    id: String,
    color_coat: String,
}

impl ColorCoat {
    pub fn new(id: String, color_coat: String) -> Self {
        Self { id, color_coat }
    }
}

impl ColorCoat {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_color_coat(&self) -> String {
        self.color_coat.clone()
    }
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SalesOrder {
    id: String,
    cust_id: String,
    prd_ord_id: String,
    item_id: String,
    item_desc: String,
    qty: i32,
    prd_comp_date: String,
    deliv_commt_date: String,
    sales_req_date: String,
    color_coat_id: String,
}

impl SalesOrder {
    pub fn new(
        id: String,
        cust_id: String,
        prd_ord_id: String,
        item_id: String,
        item_desc: String,
        qty: i32,
        prd_comp_date: String,
        deliv_commt_date: String,
        sales_req_date: String,
        color_coat_id: String,
    ) -> Self {
        Self { id, cust_id, prd_ord_id, item_id, item_desc, qty, prd_comp_date, deliv_commt_date, sales_req_date, color_coat_id }
    }
}

impl SalesOrder {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_cust_id(&self) -> String {
        self.cust_id.clone()
    }

    pub fn get_prd_ord_id(&self) -> String {
        self.prd_ord_id.clone()
    }

    pub fn get_item_id(&self) -> String {
        self. item_id.clone()
    }

    pub fn get_item_desc(&self) -> String {
        self.item_desc.clone()
    }

    pub fn get_qty(&self) -> i32 {
        self.qty
    }

    pub fn get_prd_comp_date(&self) -> String {
        self.prd_comp_date.clone()
    }

    pub fn get_deliv_commt_date(&self) -> String {
        self.deliv_commt_date.clone()
    }

    pub fn get_sales_req_date(&self) -> String {
        self.sales_req_date.clone()
    }

    pub fn get_color_coat_id(&self) -> String {
        self.color_coat_id.clone()
    }
}