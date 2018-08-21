use hyper::StatusCode;
use juniper::FieldError;
use serde_json;

use stq_api::errors::{Error as ApiError, ErrorMessage};
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

pub(crate) fn into_graphql(e: ApiError) -> FieldError {
    match e {
        ApiError::Api(
            status,
            Some(ErrorMessage {
                code,
                description,
                payload,
            }),
        ) => {
            let payload = serde_json::to_string(&payload).unwrap();
            let message = payload.clone();
            let code = code.to_string();
            let status = status.to_string();
            FieldError::new(
                "Error response from microservice",
                graphql_value!({ "code": 100, "details": {"status": status, "code": code, "description": description, "message": message, "payload": payload }}),
            )
        }
        ApiError::Api(status, None) => {
            let status = status.to_string();
            FieldError::new(
                "Error response from microservice",
                graphql_value!({ "code": 100, "details": { "status": status }}),
            )
        }
        ApiError::Network(_) => FieldError::new(
            "Network error for microservice",
            graphql_value!({ "code": 200, "details": { "See server logs for details." }}),
        ),
        ApiError::Parse(message) => FieldError::new("Unexpected parsing error", graphql_value!({ "code": 300, "details": { message }})),
        _ => FieldError::new(
            "Unknown error for microservice",
            graphql_value!({ "code": 400, "details": { "See server logs for details." }}),
        ),
    }
}
