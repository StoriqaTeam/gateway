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
use context::Context;
use settings::Settings;
use std::process;
use futures_cpupool::CpuPool;


struct WebService {
    context: Arc<context::Context>,
    schema: Arc<schema::Schema>,
    router: Arc<router::Router>,
    pool: CpuPool,
}

impl Service for WebService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let context = self.context.clone();
        let schema = self.schema.clone();
        let pool = self.pool.clone();
        match (req.method(), self.router.test(req.path())) {
            (&Get, Some(router::Route::Root)) => {
                let source = graphiql::source("/graphql");
                Box::new(future::ok(http_utils::response_with_body(source)))
            }

            (&Post, Some(router::Route::Graphql)) => {
                Box::new(http_utils::read_body(req).and_then(move |body| {
                    let graphql_req = (serde_json::from_str(&body)
                        as Result<GraphQLRequest, serde_json::error::Error>)
                        .unwrap();
                    let result = pool.spawn_fn(move || {
                        let graphql_resp = graphql_req.execute(&schema, &context);
                        serde_json::to_string(&graphql_resp)
                    }).then(|r| match r {
                        Ok(data) => future::ok(http_utils::response_with_body(data)),
                        Err(err) => {
                            future::ok(http_utils::response_with_error(error::Error::Json(err)))
                        }
                    });
                    result
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
    let n_workers = 4;
    // Create a worker thread pool with four threads
    let pool = CpuPool::new(n_workers);
    let mut core = Core::new().unwrap();
    let main_handle = core.handle();
    let remote = main_handle.remote().clone();

    let addr = settings.gateway.url.parse().unwrap();
    let server = Http::new()
        .serve_addr_handle(&addr, &main_handle, move || {
            let schema = schema::create();
            let context = Context::new(settings.clone(), remote.clone());
            let service = WebService {
                context: Arc::new(context),
                schema: Arc::new(schema),
                router: Arc::new(router::create_router()),
                pool: pool.clone(),
            };
            Ok(service)
        })
        .unwrap_or_else(|why| {
            error!("Http Server Initialization Error: {}", why);
            process::exit(1);
        });

    let server_handle = main_handle.clone();
    main_handle.spawn(
        server
            .for_each(move |conn| {
                server_handle.spawn(
                    conn.map(|_| ())
                        .map_err(|why| error!("Server Error: {:?}", why)),
                );
                Ok(())
            })
            .map_err(|_| ()),
    );

    core.run(futures::future::empty::<(), ()>()).unwrap();
}
