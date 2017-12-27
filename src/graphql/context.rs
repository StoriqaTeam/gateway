use std::sync::Arc;

use juniper;
use tokio_core::reactor::{Handle, Remote};

use super::schema;
use ::config::Config;
// use ::http::client::Client;

pub struct Context {
    pub config: Arc<Config>,
    pub schema: Arc<schema::Schema>,
    pub tokio_remote: Arc<Remote>,
    // pub http_client: Client,
}

impl Context {
  pub fn new(config: Arc<Config>, handle: Arc<Handle>) -> Self {
    Context {
      config,
      schema: Arc::new(schema::create()),
      tokio_remote: Arc::new(handle.remote().clone()),
    }
  }
}

impl juniper::Context for Context {}
