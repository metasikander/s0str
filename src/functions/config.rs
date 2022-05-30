use std::env;
use dotenv::dotenv;
use crate::types::config::Config;

pub fn init_config() -> Config {
    dotenv().ok().unwrap();

    Config {
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

fn missing_variable(key: &'static str) -> String {
    format!("Missing or invalid '{}' environment variable!", key)
}
fn invalid(value: String) -> String {
    format!("\"{}\" is not a valid value for the 'Server_addr' environment variable!", value)
}
