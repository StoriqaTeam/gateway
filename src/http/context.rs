use std::sync::Arc;

use futures_cpupool::CpuPool;

use stq_http::client::ClientHandle;
use stq_router::RouteParser;

use super::router::{self, Route};
use config::Config;
use graphql;

pub struct Context {
    pub router: Arc<RouteParser<Route>>,
    pub graphql_context: graphql::context::Context,
    pub graphql_thread_pool: Arc<CpuPool>,
}

impl Context {
    pub fn new(config: Arc<Config>, client_handle: ClientHandle) -> Self {
        Context {
            router: Arc::new(router::create_route_parser()),
            graphql_context: graphql::context::Context::new(config.clone(), client_handle),
            graphql_thread_pool: Arc::new(CpuPool::new(config.gateway.graphql_thread_pool_size)),
        }
    }
}
