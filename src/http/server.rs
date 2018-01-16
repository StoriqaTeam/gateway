use std::sync::Arc;
use std::process;

use hyper;
use hyper::{Get, Post};
use hyper::server::{Http, Request, Response, Service};
use hyper::header::{Authorization, Bearer};
use futures;
use futures::future;
use futures::{Future, Stream};
use serde_json;
use juniper::http::GraphQLRequest;
use tokio_core::reactor::{Handle};
use jsonwebtoken::{decode, Validation};

use super::router;
use super::context::Context;
use super::graphiql;
use super::utils;
use super::error;
use super::client;
use ::config::Config;
use super::jwt::JWTPayload;


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
                let headers = req.headers().clone();
                let auth_header = headers.get::<Authorization<Bearer>>();
                let jwt_secret_key = context.graphql_context.config.jwt.secret_key.clone();
                let token_payload = auth_header.map (move |auth| {
                        let token = auth.0.token.as_ref();
                        decode::<JWTPayload>(token, jwt_secret_key.as_ref(), &Validation::default())
                            .ok()
                            .map(|t| t.claims)
                    })
                    .and_then(|x| x);
                
                Box::new(utils::read_body(req.body()).and_then(move |body| {
                    let mut graphql_context = context.graphql_context.clone();
                    graphql_context.user = token_payload;

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

            _ => Box::new(future::ok(utils::response_not_found())),
        }
    }
}

pub fn start(config: Arc<Config>, tokio_handle: Arc<Handle>, client_handle: client::ClientHandle) {
    let addr = config.gateway.url.parse().expect("Cannot parse gateway url from config");

    let config_arc = config.clone();
    let handle_arc = tokio_handle.clone();
    let serve = Http::new()
        .serve_addr_handle(&addr, &tokio_handle, move || {
            Ok(
                WebService {
                    context: Arc::new(Context::new(config_arc.clone(), client_handle.clone())),
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
