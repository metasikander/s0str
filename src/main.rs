#[macro_use]
extern crate tracing;

//-////////////////////////////////////////////////////////
mod services {
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

use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use tokio::io::Result;
use tracing_subscriber::FmtSubscriber;
use tracing::log::LevelFilter;
use crate::services::websocket::run_ws_listener;
use crate::types::config::Config;

//-////////////////////////////////////////////////////////////////////////////
//  Globals
//-////////////////////////////////////////////////////////////////////////////

#[static_init::dynamic]
static CONFIG: Config = Config::init();

//-////////////////////////////////////////////////////////////////////////////
//  Main
//-////////////////////////////////////////////////////////////////////////////
#[tokio::main]
pub async fn main() -> Result<()> {
    // -- Logger --
    FmtSubscriber::builder()
        .with_max_level(CONFIG.debug_lvl)
        .init();

    // -- Database --
    info!("-- INIT DB --");
    let mut options = match &CONFIG.postgres {
        Some(pg_conf) => {
            info!("Database: Postgres");
            ConnectOptions::from(pg_conf.to_url())
        },
        None => {
            info!("Database: in memory SQLite");
            ConnectOptions::from("sqlite::memory:")
        },
    };
    options.sqlx_logging_level(LevelFilter::Trace);
    let db = Database::connect(options).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    // -- Websocket --
    run_ws_listener(db).await;

    Ok(())
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
