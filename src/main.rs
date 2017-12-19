extern crate gateway_lib;

fn main() {
    let settings = gateway_lib::settings::Settings::new().expect("Can't load gateway settings!");    
    gateway_lib::start_server(settings);
}
