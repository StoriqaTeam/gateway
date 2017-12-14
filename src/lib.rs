extern crate juniper;
extern crate futures;
extern crate hyper;
extern crate regex;
extern crate serde_json;

pub mod graphiql;

use futures::future::{FutureResult, Future};
use futures::{future, Stream};

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};
use hyper::error::Error;

use juniper::http::{GraphQLRequest};

struct WebService;

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

impl Service for WebService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let response = Response::new();
        let headers = hyper::header::Headers::new();
        match req.method() {
            &Get => {
                let source = graphiql::source("/graphql");
                let response = Response::new()
                        .with_header(ContentLength(source.len() as u64))
                        .with_body(source);
                Box::new(future::ok(response))
            },
            &Post => {
                Box::new(
                    read_body(req).map(|body| {
                        let graphqlReq: GraphQLRequest = serde_json::from_str(&body).unwrap();
                        response.with_headers(headers).with_body(body)
                    })
                )
            },
            _ => Box::new(futures::future::ok(response.with_headers(headers)))
        }
    }
}

    // fn call(&self, req: Request) -> Self::Future {
    //     match (req.method(), req.path()) {
    //         (&Get, "/") | (&Get, "/echo") => {
    //             let source = graphiql::source("/graphql");
    //             ok(
    //                 Response::new()
    //                     .with_header(ContentLength(source.len() as u64))
    //                     .with_body(source)
    //             )
    //         },
    //         (&Post, "/graphql") => {
    //             println!("FUNCTION CALLED?");
    //             let mut response = Response::new();
    //             // let mut headers = Headers::new();
    //             // let mut ContentType = Ascii::new("Content-Type".to_owned());
    //             // let mut AccessControlAH = Ascii::new("Access-Control-Allow-Headers".to_owned());
                
    //             // headers.set(AccessControlAllowHeaders(vec![ContentType, AccessControlAH]));
    //             // headers.set(AccessControlAllowOrigin::Any);

    //             let body = req.body()
    //                 .fold(Vec::new(), |mut acc, chunk| {
    //                     acc.extend_from_slice(&*chunk);
    //                     futures::future::ok::<_, Self::Error>(acc)
    //                 })
    //                 .and_then(|v| {
    //                     let stringify = String::from_utf8(v).unwrap();
    //                     Ok::<_, Self::Error>(stringify)
    //                 })
    //                 .and_then(|_| {
    //                     futures::future::ok(response)
    //                 });

    //             body
    //             // root_node: &RootNode<QueryT, MutationT>,
    //             // context: &CtxT,
    //             // http::GraphQLRequest
    //             // let response = self.0.execute(root_node, context);
    //             // let status = if response.is_ok() {
    //             //     Status::Ok
    //             // } else {
    //             //     Status::BadRequest
    //             // };
    //             // let json = serde_json::to_string_pretty(&response).unwrap();
    //             // let mut res = Response::new();
    //             // if let Some(len) = req.headers().get::<ContentLength>() {
    //             //     res.headers_mut().set(len.clone());
    //             // }
    //             // println!("Yo");
    //             // let v = req.body().concat2();
    //             // let res = v.map(|b| { res.with_body(b) });
    //             // res
    //             // println!("Yo1");
    //             // let x = v.wait();
    //             // println!("Yo2");
    //             // let reqBody = String::from_utf8(x.unwrap().as_ref().to_vec()).unwrap();
    //             // println!("Req: {}", reqBody);
    //             // let graphqlReq: GraphQLRequest = serde_json::from_str(&reqBody).unwrap();
    //             // println!("Req: {}", reqBody);
    //             // let resp = req.body().fold(Vec::new(), |mut v, chunk| {
    //             //     v.extend(&chunk[..]);
    //             //     future::ok::<_, hyper::Error>(v)
    //             // }).and_then(move |chunks| {
    //             //     let body = String::from_utf8(chunks).unwrap();
    //             //     future::ok(body)
    //             // }).wait();
    //             // let graphqlReq: GraphQLRequest = serde_json::from_str(req.body()).unwrap();

    //             // ok(res.with_body("123"))
    //         },
    //         (&Post, "/echo") => {
    //             let mut res = Response::new();
    //             if let Some(len) = req.headers().get::<ContentLength>() {
    //                 res.headers_mut().set(len.clone());
    //             }
    //             ok(res.with_body(req.body()))
    //         },

    //         _ => {
    //             ok (
    //                 Response::new()
    //                     .with_status(StatusCode::NotFound)
    //             )
    //         }
    //     }
    // }
// }


pub fn start_server() {
    let addr = "0.0.0.0:8000".parse().unwrap();

    let mut server = Http::new().bind(&addr, || Ok(WebService)).unwrap();
    server.no_proto();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();
}
