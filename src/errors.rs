use hyper::StatusCode;
use serde_json;

use stq_http::errors::{Codeable, PayloadCarrier};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Not found")]
    NotFound,
    #[fail(display = "Parse error")]
    Parse,
    #[fail(display = "Server is refusing to fullfil the reqeust")]
    Forbidden,
    #[fail(display = "Http client error")]
    HttpClient,
}

impl Codeable for Error {
    fn code(&self) -> StatusCode {
        match *self {
            Error::NotFound => StatusCode::NotFound,
            Error::Parse => StatusCode::UnprocessableEntity,
            Error::HttpClient => StatusCode::InternalServerError,
            Error::Forbidden => StatusCode::Forbidden,
        }
    }
}

impl PayloadCarrier for Error {
    fn payload(&self) -> Option<serde_json::Value> {
        None
    }
}
