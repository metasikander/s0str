use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use dotenv::dotenv;
use tracing::Level;

//-////////////////////////////////////////////////////////////////////////////
//  Config
//-////////////////////////////////////////////////////////////////////////////
pub struct Config {
    pub debug_lvl: Level,
    pub pubkey: String,
    pub server_addr: SocketAddr,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok().unwrap();

        Config {
            debug_lvl: {
                let key = "debug_level";
                match env::var(key).ok() {
                    None => Ok(Level::WARN),
                    Some(value) => Level::from_str(&value),
                }.expect(&missing_variable(key))
            },
            pubkey: {
                let key = "pubkey";
                env::var(key).expect(&missing_variable(key))
            },
            server_addr: {
                let key = "server_addr";
                let value = env::var(key).expect(&missing_variable(key));
                value.parse().expect(&invalid(value))
            },
        }
    }
}
//-////////////////////////////////////////////////////////////////////////////
//  Config Functions
//-////////////////////////////////////////////////////////////////////////////
fn missing_variable(key: &'static str) -> String {
    format!("Missing or invalid '{}' environment variable!", key)
}
fn invalid(value: String) -> String {
    format!("\"{}\" is not a valid value for the 'Server_addr' environment variable!", value)
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
