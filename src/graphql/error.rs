use std::convert::Into;

use serde_json;

use ::http::client;
use juniper::FieldError;

impl Into<FieldError> for client::Error {
    fn into(self) -> FieldError {
        match self {
            client::Error::Api(status, Some(client::ErrorMessage { code, message })) => {
                let code = code.to_string();
                let status = status.to_string();
                FieldError::new(
                    "Error response from microservice",
                    graphql_value!({ "status": status, "code": code, "message": message }),
                )
            },
            client::Error::Api(status, None) => {
                let status = status.to_string();
                FieldError::new(
                    "Error response from microservice",
                    graphql_value!({ "status": status }),
                )
            },
            client::Error::Network(_) =>
                FieldError::new(
                    "Network error for microservice",
                    graphql_value!("See server logs for details."),
                ),
            client::Error::Parse(message) =>
                FieldError::new(
                    "Unexpected parsing error",
                    graphql_value!(message),
                ),
            _ =>
                FieldError::new(
                    "Unknown error for microservice",
                    graphql_value!("See server logs for details."),
                ),
        }
    }
}

