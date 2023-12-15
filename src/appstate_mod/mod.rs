use axum::extract::FromRef;

use crate::error_mod::*;
use crate::model_controller::ModelController;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    mc: ModelController,
}

impl AppState {
    pub fn new(mc: ModelController) -> Result<Self> {
        Ok(Self { mc })
    }
}

impl AppState {
    pub fn get_mc(&self) -> ModelController {
        self.mc.clone()
    }
}
