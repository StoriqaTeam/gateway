extern crate gateway_lib;

fn main() {
    let config_name = "production";
    let rocket = gateway_lib::rocket_factory(config_name.to_string()).unwrap();
    rocket.launch();
}
