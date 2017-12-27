use futures_cpupool::CpuPool;
use tokio_core::reactor::{Handle};
use std::sync::Arc;

use super::router;
use config::Config;
use ::graphql;

pub struct Context {
    pub router: Arc<router::Router>,
    pub graphql_context: graphql::context::Context,
    pub graphql_thread_pool: Arc<CpuPool>,
}

impl Context {
    pub fn new(config: Arc<Config>) -> Self {
        Context {
            router: Arc::new(router::create_router()),
            graphql_context: graphql::context::Context::new(config.clone()),
            graphql_thread_pool: Arc::new(CpuPool::new(config.gateway.graphql_thread_pool_size)),
        }
    }
}
