use tokio::net::TcpListener;

use crate::constants_mod::IP_ADDR;
use crate::error_mod::*;
use routers::main_router;

mod appstate_mod;
mod constants_mod;
mod context_mod;
mod error_mod;
mod model_controller;
mod mw_mod;
mod routers;

#[tokio::main]
async fn main() -> Result<()> {
    let app = main_router()?;
    let listener = TcpListener::bind(IP_ADDR).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
