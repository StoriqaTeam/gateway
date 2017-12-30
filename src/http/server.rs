use std::sync::Arc;
use std::process;

use hyper;
use hyper::{Get, Post};
use hyper::server::{Http, Request, Response, Service};
use futures;
use futures::future;
use futures::{Future, Stream};
use serde_json;
use juniper::http::GraphQLRequest;
use tokio_core::reactor::{Handle};

use super::router;
use super::context::Context;
use super::graphiql;
use super::utils;
use super::error;
use ::config::Config;

struct WebService {
    context: Arc<Context>,
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
                Box::new(future::ok(utils::response_with_body(source)))
            }

            (&Post, Some(router::Route::Graphql)) => {
                Box::new(utils::read_body(req.body()).and_then(move |body| {
                    let graphql_context = context.graphql_context.clone();

                    let graphql_req = (serde_json::from_str(&body)
                        as Result<GraphQLRequest, serde_json::error::Error>)
                        .unwrap();

                    context.graphql_thread_pool.spawn_fn(move || {
                        let ctx = graphql_context.clone();
                        let graphql_resp = graphql_req.execute(&ctx.schema, &ctx);
                        serde_json::to_string(&graphql_resp)
                    }).then(|r| match r {
                        Ok(data) => future::ok(utils::response_with_body(data)),
                        Err(err) => {
                            future::ok(utils::response_with_error(error::Error::Json(err)))
                        }
                    })
                }))
            }

            (&Get, Some(router::Route::Users(user_id))) => Box::new(future::ok(
                utils::response_with_body(user_id.to_string()),
            )),

            _ => Box::new(future::ok(utils::response_not_found())),
        }
    }
}

pub fn start(config: Arc<Config>, tokio_handle: Arc<Handle>) {
    let addr = config.gateway.url.parse().expect("Cannot parse gateway url from config");

    let config_arc = config.clone();
    let handle_arc = tokio_handle.clone();
    let serve = Http::new()
        .serve_addr_handle(&addr, &tokio_handle, move || {
            Ok(
                WebService {
                    context: Arc::new(Context::new(config_arc.clone())),
                }
            )
        })
        .unwrap_or_else(|why| {
            error!("Http Server Initialization Error: {}", why);
            process::exit(1);
        });

    tokio_handle.spawn(
        serve
            .for_each(move |conn| {
                handle_arc.spawn(
                    conn.map(|_| ())
                        .map_err(|why| error!("Server Error: {:?}", why)),
                );
                Ok(())
            })
            .map_err(|_| ()),
    );
}