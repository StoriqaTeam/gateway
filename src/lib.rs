extern crate config;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate juniper;
extern crate regex;
extern crate serde_json;
extern crate tokio_core;

pub mod graphiql;
pub mod context;
pub mod schema;
pub mod error;
pub mod router;
pub mod http_utils;
pub mod pool;
pub mod settings;


use futures::future;
use futures::future::Future;
use hyper::{Get, Post};
use hyper::server::{Http, Request, Response, Service};
use juniper::http::GraphQLRequest;
use std::sync::Arc;
use context::Context;


struct WebService {
    context: Arc<context::Context>,
    schema: Arc<schema::Schema>,
    router: Arc<router::Router>,
}

impl Service for WebService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let context = self.context.clone();
        let schema = self.schema.clone();
        match (req.method(), self.router.test(req.path())) {
            (&Get, Some(router::Route::Root)) => {
                let source = graphiql::source("/graphql");
                Box::new(future::ok(http_utils::response_with_body(source)))
            }

            (&Post, Some(router::Route::Graphql)) => {
                Box::new(http_utils::read_body(req).and_then(move |body| {
                    let result = (serde_json::from_str(&body)
                        as Result<GraphQLRequest, serde_json::error::Error>)
                        .and_then(|graphql_req| {
                            // TODO - do this on grpahql thread pool
                            let graphql_resp = graphql_req.execute(&schema, &context);
                            serde_json::to_string(&graphql_resp)
                        });
                    match result {
                        Ok(data) => future::ok(http_utils::response_with_body(data)),
                        Err(err) => {
                            future::ok(http_utils::response_with_error(error::Error::Json(err)))
                        }
                    }
                }))
            }

            (&Get, Some(router::Route::Users(user_id))) => Box::new(future::ok(
                http_utils::response_with_body(user_id.to_string()),
            )),

            _ => Box::new(future::ok(http_utils::response_not_found())),
        }
    }
}


pub fn start_server() {
    let addr = "0.0.0.0:8000".parse().unwrap();
    let mut server = Http::new()
        .bind(&addr, move || {
            let schema = schema::create();
            let context = Context::new();
            let service = WebService {
                context: Arc::new(context),
                schema: Arc::new(schema),
                router: Arc::new(router::create_router()),
            };
            Ok(service)
        })
        .unwrap();
    server.no_proto();
    println!(
        "Listening on http://{} with 1 thread.",
        server.local_addr().unwrap()
    );
    server.run().unwrap();
}
