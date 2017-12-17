#[macro_use]
extern crate juniper;
extern crate futures;
extern crate hyper;
extern crate regex;
extern crate serde_json;

pub mod graphiql;
pub mod context;
pub mod schema;
pub mod error;
pub mod router;

use futures::future::{Future};
use futures::{future, Stream};

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};
use hyper::error::Error;

use juniper::http::{GraphQLRequest};

use std::sync::Arc;

fn read_body(request: Request) -> Box<Future<Item=String, Error=hyper::Error>> {
    Box::new(
        request.body()
            .fold(Vec::new(), |mut acc, chunk| {
                acc.extend_from_slice(&*chunk);
                future::ok::<_, hyper::Error>(acc)
            })
            .and_then(|bytes| {
                match String::from_utf8(bytes) {
                    Ok(data) => future::ok(data),
                    Err(err) => future::err(Error::Utf8(err.utf8_error()))
                }
            })
    )
}

fn response_with_body(body: String) -> Response {
    Response::new()
        .with_header(ContentLength(body.len() as u64))
        .with_body(body)
}

fn response_with_error(error: error::Error) -> Response {
    use error::Error::*;
    match error {
        Json(err) => response_with_body(err.to_string()).with_status(StatusCode::UnprocessableEntity)
    }
}

fn response_not_found() -> Response {
    response_with_body("Not found".to_string()).with_status(StatusCode::NotFound)
}

struct WebService {
    context: Arc<context::Context>,
    schema: Arc<schema::Schema>,
    router: Arc<router::Router>
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
                Box::new(future::ok(response_with_body(source)))
            },
            (&Post, Some(router::Route::Graphql)) => {
                Box::new(
                    read_body(req)
                        .and_then(move |body| {
                            let result = (serde_json::from_str(&body) as Result<GraphQLRequest, serde_json::error::Error>)
                                .and_then(|graphql_req| {
                                    let graphql_resp = graphql_req.execute(&schema, &context);
                                    serde_json::to_string(&graphql_resp)
                                });
                            match result {
                                Ok(data) => future::ok(response_with_body(data)),
                                Err(err) => future::ok(response_with_error(error::Error::Json(err)))
                            }
                        })
                )
            },
            (&Get, Some(router::Route::Users(user_id))) =>
                Box::new(future::ok(response_with_body(user_id.to_string()))),

            _ => Box::new(future::ok(response_not_found()))
        }
    }
}

pub fn start_server() {
    let addr = "0.0.0.0:8000".parse().unwrap();
    let mut server = Http::new().bind(&addr, || {
        let schema = schema::create();
        let context = context::Context {};
        let mut router = router::Router::new();
        router.add_route(r"^/$", router::Route::Root);
        router.add_route(r"^/graphql$", router::Route::Graphql);
        router.add_route_with_params(r"^/users/(\d+)$", |matches| {
            Some(router::Route::Users(1))
        });
        let service = WebService {
            context: Arc::new(context),
            schema: Arc::new(schema),
            router: Arc::new(router)
        };
        Ok(service)
    }).unwrap();
    server.no_proto();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();
}
