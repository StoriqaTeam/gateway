pub mod server;
pub mod client;
mod context;
mod error;
mod graphiql;
mod router;
mod utils;

use tokio_core::reactor::{Handle};
use std::sync::Arc;
use ::config::Config;

pub fn start_server(config: Arc<Config>, tokio_handle: Arc<Handle>) {
  server::start(config, tokio_handle);
}