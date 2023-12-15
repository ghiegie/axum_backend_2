use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    user: String,
    pwd: String,
}

impl LoginPayload {
    pub fn get_user(&self) -> String {
        self.user.clone()
    }

    pub fn get_pwd(&self) -> String {
        self.pwd.clone()
    }
}
