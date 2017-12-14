extern crate gateway_lib;

fn main() {
    let rocket = gateway_lib::rocket_factory().unwrap();
    rocket.launch();
}
