use std::sync::Arc;

use juniper;

use super::schema;
use ::config::Config;
use ::http::client::Client;

#[derive(Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub schema: Arc<schema::Schema>,
    pub http_client: Client,
}

impl Context {
  pub fn new(config: Arc<Config>) -> Self {
    Context {
      config,
      schema: Arc::new(schema::create()),
      http_client: Client::new()
    }
  }
}

impl juniper::Context for Context {}
