extern crate gateway_lib;

fn main() {
    let config = gateway_lib::config::Config::new().expect("Can't load gateway configs. Please check your /config folder.");
    gateway_lib::start(config);
}
