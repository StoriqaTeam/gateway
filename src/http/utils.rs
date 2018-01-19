use std::mem;

use hyper;
use hyper::{StatusCode};
use hyper::header::{ContentLength, Headers};
use hyper::server::Response;
use hyper::error::Error;
use futures::future::{Future};
use futures::{future, Stream};

use super::error;

pub fn read_body(body: hyper::Body) -> Box<Future<Item=String, Error=hyper::Error>> {
    Box::new(
        body
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

pub fn response_with_body(body: String) -> Response {
    Response::new()
        .with_header(ContentLength(body.len() as u64))
        .with_body(body)
}

pub fn response_with_error(error: error::Error) -> Response {
    use super::error::Error::*;
    match error {
        Json(err) => response_with_body(err.to_string()).with_status(StatusCode::UnprocessableEntity)
    }
}

pub fn response_not_found() -> Response {
    response_with_body("Not found".to_string()).with_status(StatusCode::NotFound)
}

pub fn add_headers_to_response(mut res: Response, headers: Headers) -> Response {
    mem::replace(res.headers_mut(), headers);
    res
}
