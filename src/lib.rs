extern crate stq_http;
extern crate stq_router;
extern crate stq_routes;
extern crate stq_static_resources;

extern crate base64;
extern crate chrono;
extern crate config as config_crate;
extern crate env_logger;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate jsonwebtoken;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate log;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate uuid;

use std::env;
use std::io::Write;
use std::sync::Arc;

use chrono::prelude::*;
use env_logger::Builder as LogBuilder;
use futures::stream::Stream;
use log::LevelFilter as LogLevelFilter;
use tokio_core::reactor::Core;

use stq_http::client::Client as HttpClient;

use config::Config;

pub mod config;
pub mod graphql;
pub mod http;

pub fn start(config: Config) {
    let mut builder = LogBuilder::new();
    builder
        .format(|formatter, record| {
            let now = Utc::now();
            writeln!(
                formatter,
                "{} - {} - {}",
                now.to_rfc3339(),
                record.level(),
                record.args()
            )
        })
        .filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    // Prepare logger
    builder.init();

    let config = Arc::new(config);

    let mut core = Core::new().expect("Unexpected error creating main event loop");
    let handle = Arc::new(core.handle());

    let client = HttpClient::new(&config.to_http_config(), &handle);
    let client_handle = client.handle();
    let client_stream = client.stream();
    handle.spawn(client_stream.for_each(|_| Ok(())));

    http::start_server(config, handle, client_handle);

    core.run(futures::future::empty::<(), ()>()).unwrap();
}
