#![feature(plugin)]
#![plugin(rocket_codegen)]

pub mod api;
pub mod schema;
pub mod context;
pub mod pool;
pub mod config;

extern crate futures;
extern crate hyper;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;
extern crate tokio_core;


use config::Config;
use context::Context;
use tokio_core::reactor::Core;


pub fn rocket_factory(config_name: String) -> Result<rocket::Rocket, String> {
    let config = Config::from(&config_name)?;
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let mut context = Context::new(config, &handle);
    let schema = schema::create();
    let rocket = rocket::ignite()
        // .manage(context)
        .manage(schema)
        .mount("/", routes![api::graph::ping, api::graph::graphql,]);

    Ok(rocket)
}
