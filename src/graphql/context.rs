use std::sync::Arc;

use juniper;

use super::schema;
use ::config::Config;
use ::http::client::ClientHandle;
use ::http::jwt::TokenPayload;

#[derive(Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub schema: Arc<schema::Schema>,
    pub http_client: ClientHandle,
    pub user: Option<TokenPayload>
}

impl Context {
  pub fn new(config: Arc<Config>, client_handle: ClientHandle) -> Self {
    Context {
      config,
      schema: Arc::new(schema::create()),
      http_client: client_handle,
      user: None
    }
  }
}

impl juniper::Context for Context {}
