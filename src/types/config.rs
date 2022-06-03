use std::env;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use default_net::get_default_interface;
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
        dotenv().ok();

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
                let key = "port";
                let port: u16 = match env::var(key) {
                    Err(_) => 0,
                    Ok(value) => value.parse().expect(&invalid(value)),
                };
                let interface = get_default_interface().unwrap();
                for n in &interface.ipv4 {
                    dbg!(n.addr);
                }
                SocketAddr::new(IpAddr::V4(interface.ipv4.first().expect("No network configured").addr), port)
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
