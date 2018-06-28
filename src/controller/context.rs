use std::sync::Arc;

use futures_cpupool::CpuPool;

use stq_http::client::ClientHandle;
use stq_router::RouteParser;

use super::routes::{self, Route};
use config::Config;
use graphql;

#[derive(Clone)]
pub struct Context {
    pub route_parser: Arc<RouteParser<Route>>,
    pub graphql_context: graphql::context::Context,
    pub graphql_thread_pool: CpuPool,
}

impl Context {
    pub fn new(config: Config, client_handle: ClientHandle) -> Self {
        Context {
            route_parser: Arc::new(routes::create_route_parser()),
            graphql_thread_pool: CpuPool::new(config.gateway.graphql_thread_pool_size),
            graphql_context: graphql::context::Context::new(config, client_handle),
        }
    }
}
