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
