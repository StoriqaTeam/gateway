pub mod server;
mod context;
mod error;
mod graphiql;
pub mod router;
mod utils;

use std::sync::Arc;
use tokio_core::reactor::Handle;

use stq_http::ClientHandle;

use config::Config;

pub fn start_server(config: Arc<Config>, tokio_handle: Arc<Handle>, client_handle: ClientHandle) {
    server::start(config, tokio_handle, client_handle);
}
