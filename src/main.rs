extern crate gateway_lib;

fn main() {
    let settings = gateway_lib::config::Config::new().expect("Can't load gateway settings!");    
    gateway_lib::start_server(settings);
}
