extern crate env_logger;
#[macro_use]
extern crate log;
use env_logger::{LogBuilder, LogTarget};
extern crate gateway_lib;



fn main() {
    LogBuilder::new().target(LogTarget::Stdout).init().unwrap();

    let config = gateway_lib::config::Config::new()
        .expect("Can't load gateway configs. Please check your /config folder.");
    gateway_lib::start(config);
}
