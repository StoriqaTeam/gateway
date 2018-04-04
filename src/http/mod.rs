pub mod server;
pub mod context;
pub mod graphiql;
pub mod router;
pub mod utils;

use std::sync::Arc;
use tokio_core::reactor::Handle;

use stq_http::client::ClientHandle;

use config::Config;

pub fn start_server(config: Arc<Config>, tokio_handle: Arc<Handle>, client_handle: ClientHandle) {
    server::start(config, tokio_handle, client_handle);
}
