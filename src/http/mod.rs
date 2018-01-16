pub mod server;
pub mod client;
mod context;
mod error;
mod graphiql;
mod router;
mod utils;
pub mod jwt;

use std::sync::Arc;
use tokio_core::reactor::{Handle};

use ::config::Config;

pub fn start_server(config: Arc<Config>, tokio_handle: Arc<Handle>, client_handle: client::ClientHandle) {
  server::start(config, tokio_handle, client_handle);
}