extern crate config as config_crate;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
#[macro_use]
extern crate juniper;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

pub mod config;
mod graphql;
mod http;

use tokio_core::reactor::Core;
use std::sync::Arc;

use config::Config;

pub fn start(config: Config) {
    let config = Arc::new(config);
    let mut core = Core::new().expect("Unexpected error creating event loop core");
    let handle = Arc::new(core.handle());

    http::start_server(config, handle);

    core.run(futures::future::empty::<(), ()>()).unwrap();
}
