use std::sync::Arc;

use futures::prelude::*;
use hyper;
use hyper::header::{Authorization, Cookie, Headers};
use juniper;
use juniper::FieldError;
use serde::de::DeserializeOwned;

use super::schema;
use config::Config;

use stq_http::client::{ClientHandle, Error};

use graphql::models::jwt::JWTPayload;

#[derive(Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub schema: Arc<schema::Schema>,
    pub http_client: ClientHandle,
    pub user: Option<JWTPayload>,
    pub session_id: Option<i32>,
    pub uuid: String,
}

impl Context {
    pub fn new(config: Arc<Config>, client_handle: ClientHandle) -> Self {
        Context {
            config,
            schema: Arc::new(schema::create()),
            http_client: client_handle,
            user: None,
            session_id: None,
            uuid: "".to_string(),
        }
    }

    pub fn request<T>(&self, method: hyper::Method, url: String, body: Option<String>) -> Box<Future<Item = T, Error = FieldError> + Send>
    where
        T: DeserializeOwned + 'static + Send,
    {
        let mut headers = Headers::new();
        if let Some(ref user) = self.user {
            headers.set(Authorization(user.to_string()));
        };
        let mut cookie = Cookie::new();
        cookie.append("UUID", self.uuid.clone());
        if let Some(ref session_id) = self.session_id {
            cookie.append("SESSION_ID", session_id.to_string());
        };
        headers.set(cookie);

        Box::new(
            self.http_client
                .request(method, url, body, Some(headers))
                .map_err(Error::into_graphql),
        )
    }
}

impl juniper::Context for Context {}
