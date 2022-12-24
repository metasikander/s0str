use std::env;
use std::net::IpAddr;
use std::net::Ipv4Addr;
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
    pub web_socket_addr: SocketAddr,
    pub postgres: Option<PgConfig>,
}

pub struct PgConfig {
    pub host: String,
    pub user: String,
    pub pass: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        Config {
            debug_lvl: {
                let key = "debug_level";
                match env::var("debug_level").ok() {
                    None => Ok(Level::INFO),
                    Some(value) => Level::from_str(&value),
                }.expect(&missing(key))
            },
            pubkey: {
                let key = "pubkey";
                env::var(key).expect(&missing(key))
            },
            web_socket_addr: {
                let key = "ws_ip";
                let ip_addr: IpAddr = match env::var(key) {
                    Err(_) => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                    Ok(ip_addr) => ip_addr.parse::<IpAddr>().expect(&invalid(key, ip_addr)),
                };
                let key = "ws_port";
                let port: u16 = match env::var(key) {
                    Err(_) => 8080,
                    Ok(value) => value.parse().expect(&invalid(key, value)),
                };
                SocketAddr::new(ip_addr, port)
            },
            postgres: {
                let host = env::var("pg_host");
                let user = env::var("pg_user");
                let pass = env::var("pg_pass");

                let is_some = [&host, &user, &pass].into_iter().fold(false, |acc, val| acc || val.is_ok());
                let is_all  = [&host, &user, &pass].into_iter().fold(true , |acc, val| acc && val.is_ok());

                if is_all {
                    Some(PgConfig{
                        host: host.unwrap(),
                        user: user.unwrap(),
                        pass: pass.unwrap(),
                    })
                } else if is_some {
                    let msg = [
                        ("pg_host", host),
                        ("pg_user", user),
                        ("pg_pass", pass),
                    ].into_iter()
                        .flat_map(|(key, result)|
                            if result.is_err() {vec![missing(key)]}
                            else {vec![]}
                        )
                        .reduce(|acc, var| format!("{}\n{}", acc, var))
                        .unwrap();
                    panic!("{}", msg);
                } else {
                    None
                }
            }
        }
    }
}

impl PgConfig {
    pub fn to_url(&self) -> String {
        format!("postgres://{}:{}@{}/s0ster", self.user, self.pass, self.host)
    }
}
//-////////////////////////////////////////////////////////////////////////////
//  Config Functions
//-////////////////////////////////////////////////////////////////////////////
fn missing(key: &'static str) -> String {
    format!("Missing the '{}' environment variable!", key)
}
fn invalid(key: &'static str, value: String) -> String {
    format!("\"{}\" is not a valid value for the '{}' environment variable!", value, key)
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
