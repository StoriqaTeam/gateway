use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::sync::Arc;

use futures;
use futures::IntoFuture;
use futures::future;
use futures::{Future, Stream};
use hyper;
use hyper::Method::{Get, Options, Post};
use hyper::header::{AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlAllowOrigin, AccessControlMaxAge,
                    AccessControlRequestHeaders, Authorization, Bearer, ContentType, Headers};
use hyper::mime;
use hyper::server::{Http, Request, Response, Service};
use jsonwebtoken::{decode, Algorithm, Validation};
use juniper::http::GraphQLRequest;
use serde_json;
use tokio_core::reactor::Handle;
use uuid::Uuid;

use stq_http::client::ClientHandle;

use super::context::Context;
use super::graphiql;
use super::headers::SessionId;
use super::router;
use super::utils;
use config::Config;
use graphql::models::jwt::JWTPayload;

struct WebService {
    context: Arc<Context>,
    jwt_public_key: Vec<u8>,
}

impl Service for WebService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        info!("req {:?}", &req);
        let context = self.context.clone();
        match (req.method(), self.context.router.test(req.path())) {
            (&Get, Some(router::Route::Healthcheck)) => Box::new(future::ok(utils::response_with_body("Ok".to_string()))),

            (&Get, Some(router::Route::Root)) => {
                let source = graphiql::source("/graphql");
                Box::new(future::ok(utils::response_with_body(source)))
            }

            (&Post, Some(router::Route::Graphql)) => {
                let headers = req.headers().clone();
                let auth_header = headers.get::<Authorization<Bearer>>();
                let jwt_public_key = self.jwt_public_key.clone();
                let domain = context.graphql_context.config.cors.domain.clone();
                let leeway = context.graphql_context.config.jwt.leeway;

                let mut validation = Validation {
                    leeway,
                    ..Validation::new(Algorithm::RS256)
                };
                let token_payload = auth_header.and_then(move |auth| {
                    let token = auth.0.token.as_ref();
                    decode::<JWTPayload>(token, jwt_public_key.as_ref(), &validation)
                        .ok()
                        .map(|t| t.claims)
                });

                let session_id_header = headers.get::<SessionId>().and_then(|sid| sid.parse::<i32>().ok());

                Box::new(utils::read_body(req.body()).and_then(move |body| {
                    let mut graphql_context = context.graphql_context.clone();
                    graphql_context.user = token_payload;
                    graphql_context.uuid = Uuid::new_v4().to_string();
                    graphql_context.session_id = session_id_header;
                    serde_json::from_str::<GraphQLRequest>(&body)
                        .into_future()
                        .and_then(move |graphql_req| {
                            context.graphql_thread_pool.spawn_fn(move || {
                                let graphql_resp = graphql_req.execute(&graphql_context.schema, &graphql_context);
                                serde_json::to_string(&graphql_resp)
                            })
                        })
                        .then(|r| match r {
                            Ok(data) => future::ok(utils::response_with_body(data)),
                            Err(err) => future::ok(utils::response_with_error(err)),
                        })
                        .and_then(move |resp| {
                            let mut new_headers = Headers::new();
                            new_headers.set(AccessControlAllowOrigin::Value(domain.to_owned()));
                            Box::new(future::ok(utils::replace_response_headers(resp, new_headers)))
                        })
                }))
            }

            (&Options, Some(router::Route::Graphql)) => {
                let domain = context.graphql_context.config.cors.domain.clone();
                let max_age = context.graphql_context.config.cors.max_age;
                let req_headers = req.headers().clone();
                let acah = req_headers.get::<AccessControlRequestHeaders>();

                let resp = Response::new();
                let mut new_headers = Headers::new();
                new_headers.set(AccessControlAllowOrigin::Value(domain.to_owned()));
                new_headers.set(AccessControlAllowMethods(vec![Get, Post, Options]));
                if let Some(a) = acah {
                    new_headers.set(AccessControlAllowHeaders(a.to_vec()));
                };
                new_headers.set(AccessControlMaxAge(max_age));
                new_headers.set(ContentType(mime::TEXT_HTML));

                Box::new(future::ok(utils::replace_response_headers(resp, new_headers)))
            }

            _ => {
                error!("Received request to non existing endpoint of gateway.");
                Box::new(future::ok(utils::response_not_found()))
            }
        }
    }
}

pub fn start(config: Arc<Config>, tokio_handle: Arc<Handle>, client_handle: ClientHandle) {
    let addr = config.gateway.url.parse().expect("Cannot parse gateway url from config");

    debug!("Reading public key file {}", &config.jwt.public_key_path);
    let mut f = File::open(config.jwt.public_key_path.clone()).unwrap();
    let mut jwt_public_key: Vec<u8> = Vec::new();
    f.read_to_end(&mut jwt_public_key).unwrap();

    let config_arc = config.clone();
    let handle_arc = tokio_handle.clone();
    let serve = Http::new()
        .serve_addr_handle(&addr, &tokio_handle, move || {
            Ok(WebService {
                context: Arc::new(Context::new(config_arc.clone(), client_handle.clone())),
                jwt_public_key: jwt_public_key.clone(),
            })
        })
        .unwrap_or_else(|why| {
            error!("Http Server Initialization Error: {}", why);
            process::exit(1);
        });

    tokio_handle.spawn(
        serve
            .for_each(move |conn| {
                handle_arc.spawn(conn.map(|_| ()).map_err(|why| error!("Server Error: {:?}", why)));
                Ok(())
            })
            .map_err(|_| ()),
    );
}
