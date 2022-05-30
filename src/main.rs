//-////////////////////////////////////////////////////////
mod functions {
    pub mod config;
}
mod services {
    pub mod database;
    pub mod websocket;
}
mod types {
    pub mod config;
    pub mod nostr {
        pub mod event;
        pub mod filter;
        pub mod kind;
        pub mod protocol;
        pub mod tag;
    }
}
//-////////////////////////////////////////////////////////
use static_init::dynamic;
use tokio::io::Result;
use crate::functions::config::init_config;
use crate::services::websocket::run_ws_listener;
use crate::types::config::Config;
use crate::services::database::init_database;

//-////////////////////////////////////////////////////////////////////////////
//  Globals
//-////////////////////////////////////////////////////////////////////////////

#[dynamic]
static CONFIG: Config = init_config();

//-////////////////////////////////////////////////////////////////////////////
//  Main
//-////////////////////////////////////////////////////////////////////////////
#[tokio::main]
pub async fn main() -> Result<()> {
    let db = init_database().await.unwrap();
    run_ws_listener(db).await;
    Ok(())
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
