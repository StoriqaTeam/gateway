extern crate gateway_lib;

use std::env;
use std::str::FromStr;

use gateway_lib::config::Env;

fn main() {
    let config_name = env::var("GATEWAY_CONFIG").expect("GATEWAY_CONFIG must be set!");
    let config_name = Env::from_str(&config_name).unwrap();
    gateway_lib::start_server(config_name);
}
