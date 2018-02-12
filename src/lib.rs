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

extern crate base64;
extern crate jsonwebtoken;

pub mod config;
mod graphql;
pub mod http;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use std::sync::Arc;

use config::Config;

pub fn start(config: Config) {
    let config = Arc::new(config);

    let mut core = Core::new().expect("Unexpected error creating main event loop");
    let handle = Arc::new(core.handle());

    let client = http::client::Client::new(&config, &handle);
    let client_handle = client.handle();
    let client_stream = client.stream();
    handle.spawn(client_stream.for_each(|_| Ok(())));

    http::start_server(config, handle, client_handle);

    core.run(futures::future::empty::<(), ()>()).unwrap();
}
