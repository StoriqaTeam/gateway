use std::sync::Arc;

use chrono::prelude::*;
use failure::Error as FailureError;
use failure::Fail;
use futures::future;
use futures::prelude::*;
use hyper::header::{Authorization, Bearer};
use hyper::server::Request;
use hyper::Method::{Get, Post};
use jsonwebtoken::{decode, Algorithm, Validation};
use juniper::http::GraphQLRequest;
use serde_json;
use uuid::Uuid;

use stq_http::client::ClientHandle;
use stq_http::controller::Controller;
use stq_http::controller::ControllerFuture;
use stq_http::request_util::parse_body;
use stq_http::request_util::serialize_future;
use stq_http::request_util::SessionId;

use self::context::Context;
use self::routes::Route;
use config;
use errors::Error;
use graphql::models::jwt::JWTPayload;

pub mod context;
pub mod graphiql;
pub mod routes;

pub struct ControllerImpl {
    context: Arc<Context>,
    jwt_public_key: Vec<u8>,
}

impl ControllerImpl {
    /// Create a new controller based on services
    pub fn new(config: config::Config, http_client: ClientHandle, jwt_public_key: Vec<u8>) -> Self {
        let context = Arc::new(Context::new(Arc::new(config), http_client));
        Self { context, jwt_public_key }
    }
}

impl Controller for ControllerImpl {
    fn call(&self, req: Request) -> ControllerFuture {
        let context = self.context.clone();

        let method = format!("{}", req.method());
        let path = format!("{}", req.path());
        let dt = Local::now();

        Box::new(
            match (&req.method().clone(), self.context.route_parser.test(req.path())) {
                (&Get, Some(Route::Root)) => Box::new(future::ok(graphiql::source("/graphql"))),

                (&Post, Some(Route::Graphql)) => {
                    let headers = req.headers().clone();
                    let auth_header = headers.get::<Authorization<Bearer>>();
                    let jwt_public_key = self.jwt_public_key.clone();
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

                    serialize_future::<_, FailureError, _>(
                        parse_body::<GraphQLRequest>(req.body())
                            .map_err(|e| {
                                e.context("Parsing body // POST /graphql in GraphQLRequest failed!")
                                    .context(Error::Parse)
                                    .into()
                            })
                            .and_then(move |graphql_req| {
                                let mut graphql_context = context.graphql_context.clone();
                                graphql_context.user = token_payload;
                                graphql_context.uuid = Uuid::new_v4().to_string();
                                graphql_context.session_id = session_id_header;
                                context
                                    .graphql_thread_pool
                                    .spawn_fn(move || {
                                        let resp = graphql_req.execute(&graphql_context.schema, &graphql_context);
                                        serde_json::to_value(resp)
                                    })
                                    .map_err(From::from)
                            }),
                    )
                }

                // Fallback
                (m, _) => Box::new(future::err(
                    format_err!("Request to non existing endpoint in notifications microservice! {:?} {:?}", m, path)
                        .context(Error::NotFound)
                        .into(),
                )),
            }.then(move |res| {
                let d = Local::now() - dt.clone();
                let message = match &res {
                    &Ok(_) => format!(
                        "Response with success: {} {}, elapsed time = {}.{:03}",
                        method,
                        path,
                        d.num_seconds(),
                        d.num_milliseconds()
                    ),
                    &Err(ref e) => format!(
                        "Response with error {}: {} {}, elapsed time = {}.{:03}",
                        e,
                        method,
                        path,
                        d.num_seconds(),
                        d.num_milliseconds()
                    ),
                };
                debug!("{}", message);
                res
            }),
        )
    }
}
