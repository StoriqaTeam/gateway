#![recursion_limit = "128"]

extern crate stq_api;
extern crate stq_http;
extern crate stq_logging;
extern crate stq_router;
extern crate stq_routes;
extern crate stq_static_resources;
extern crate stq_types;

extern crate base64;
extern crate chrono;
extern crate config as config_crate;
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
#[macro_use]
extern crate serde_json;
extern crate tokio_core;
extern crate uuid;
#[macro_use]
extern crate failure;
extern crate geo;
#[macro_use]
extern crate sentry;

pub mod config;
pub mod controller;
pub mod errors;
pub mod graphql;
pub mod sentry_integration;

use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::sync::Arc;

use futures::future;
use futures::prelude::*;
use futures::stream::Stream;
use futures_cpupool::CpuPool;
use hyper::header::{AccessControlAllowOrigin, AccessControlMaxAge, ContentType};
use hyper::server::Http;
use tokio_core::reactor::Core;

use stq_http::controller::Application;

use config::Config;
use graphql::schema;

pub fn start(config: Config) {
    // Prepare reactor
    let mut core = Core::new().expect("Unexpected error creating event loop core");
    let handle = Arc::new(core.handle());

    let address = config.gateway.url.parse().expect("Address must be set in configuration");

    debug!("Reading public key file {}", &config.jwt.public_key_path);
    let mut f = File::open(config.jwt.public_key_path.clone()).unwrap();
    let mut jwt_public_key: Vec<u8> = Vec::new();
    f.read_to_end(&mut jwt_public_key).unwrap();

    let client = stq_http::client::Client::new(&config.to_http_config(), &handle);
    let client_handle = client.handle();
    let client_stream = client.stream();
    handle.spawn(client_stream.for_each(|_| Ok(())));
    let domain = config.cors.domain.clone();
    let max_age = config.cors.max_age;
    let cpu_pool = CpuPool::new(config.gateway.graphql_thread_pool_size);
    let jwt_leeway = config.jwt.leeway;
    let schema = Arc::new(schema::create());

    let serve = Http::new()
        .serve_addr_handle(&address, &*handle, {
            move || {
                let domain = domain.to_owned();
                // Prepare application
                let app = Application::<errors::Error>::new(controller::ControllerImpl::new(
                    client_handle.clone(),
                    jwt_public_key.clone(),
                    cpu_pool.clone(),
                    jwt_leeway,
                    config.clone(),
                    schema.clone(),
                )).with_middleware(move |mut resp| {
                    let contains_acao = resp.headers().has::<AccessControlAllowOrigin>();
                    if !contains_acao {
                        resp.headers_mut().set(AccessControlAllowOrigin::Value(domain.clone()));
                    }
                    resp.headers_mut().set(AccessControlMaxAge(max_age));
                    resp.headers_mut().set(ContentType::html());
                    resp
                });

                Ok(app)
            }
        }).unwrap_or_else(|reason| {
            eprintln!("Http Server Initialization Error: {}", reason);
            process::exit(1);
        });

    handle.spawn(
        serve
            .for_each({
                let handle = handle.clone();
                move |conn| {
                    handle.spawn(conn.map(|_| ()).map_err(|why| eprintln!("Server Error: {:?}", why)));
                    Ok(())
                }
            }).map_err(|_| ()),
    );

    //info!("Listening on http://{}, threads: {}", address, thread_count);
    core.run(future::empty::<(), ()>()).unwrap();
}
