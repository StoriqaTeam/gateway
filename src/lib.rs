extern crate config;
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

pub mod graphiql;
pub mod context;
pub mod schema;
pub mod error;
pub mod router;
pub mod http_utils;
pub mod settings;


use futures::future;
use futures::{Future, Stream};
use tokio_core::reactor::Core;

use hyper::{Get, Post};
use hyper::server::{Http, Request, Response, Service};
use juniper::http::GraphQLRequest;
use std::sync::Arc;
use settings::Settings;
use std::process;


struct WebService {
    context: Arc<context::Http>,
}

impl Service for WebService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let context = self.context.clone();
        match (req.method(), self.context.router.test(req.path())) {
            (&Get, Some(router::Route::Root)) => {
                let source = graphiql::source("/graphql");
                Box::new(future::ok(http_utils::response_with_body(source)))
            }

            (&Post, Some(router::Route::Graphql)) => {
                Box::new(http_utils::read_body(req).and_then(move |body| {
                    let graphql_context = context.graphql.clone();

                    let graphql_req = (serde_json::from_str(&body)
                        as Result<GraphQLRequest, serde_json::error::Error>)
                        .unwrap();

                    context.thread_pool.spawn_fn(move || {
                        let graphql_resp = graphql_req.execute(&graphql_context.schema, &graphql_context);
                        serde_json::to_string(&graphql_resp)
                    }).then(|r| match r {
                        Ok(data) => future::ok(http_utils::response_with_body(data)),
                        Err(err) => {
                            future::ok(http_utils::response_with_error(error::Error::Json(err)))
                        }
                    })
                }))
            }

            (&Get, Some(router::Route::Users(user_id))) => Box::new(future::ok(
                http_utils::response_with_body(user_id.to_string()),
            )),

            _ => Box::new(future::ok(http_utils::response_not_found())),
        }
    }
}

pub fn start_server(settings: Settings) {
    let settings = Arc::new(settings);
    let mut core = Core::new().expect("Unexpected error creating event loop core");
    let handle = Arc::new(core.handle());
    let addr = settings.gateway.url.parse().expect("Cannot parse gateway url from config");

    let settings_arc = settings.clone();
    let handle_arc = handle.clone();
    let serve = Http::new()
        .serve_addr_handle(&addr, &handle, move || {
            Ok(
                WebService {
                    context: Arc::new(context::Http::new(settings_arc.clone(), handle_arc.clone())),
                }
            )
        })
        .unwrap_or_else(|why| {
            error!("Http Server Initialization Error: {}", why);
            process::exit(1);
        });

    let handle_arc2 = handle.clone();
    handle.spawn(
        serve
            .for_each(move |conn| {
                handle_arc2.spawn(
                    conn.map(|_| ())
                        .map_err(|why| error!("Server Error: {:?}", why)),
                );
                Ok(())
            })
            .map_err(|_| ()),
    );

    core.run(futures::future::empty::<(), ()>()).unwrap();
}
