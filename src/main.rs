extern crate env_logger;
extern crate log;
use env_logger::LogBuilder;
extern crate gateway_lib;



fn main() {
    LogBuilder::new().init().unwrap();

    let config = gateway_lib::config::Config::new()
        .expect("Can't load gateway configs. Please check your /config folder.");
    gateway_lib::start(config);
}
