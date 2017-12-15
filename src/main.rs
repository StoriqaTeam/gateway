extern crate gateway_lib;

fn main() {
    let config = "This is a temporary config";
    let rocket = gateway_lib::rocket_factory(config.to_string()).unwrap();
    rocket.launch();
}
