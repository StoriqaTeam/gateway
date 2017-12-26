use juniper;
use schema;
use router;
use futures_cpupool::CpuPool;
use config::Config;
use tokio_core::reactor::{Handle, Remote};
use std::sync::Arc;

pub struct Graphql {
    pub config: Arc<Config>,
    pub schema: Arc<schema::Schema>,
    pub tokio_remote: Arc<Remote>,
}

pub struct Http {
    pub router: Arc<router::Router>,
    pub graphql: Arc<Graphql>,
    pub graphql_thread_pool: Arc<CpuPool>,
}

impl Http {
    pub fn new(config: Arc<Config>, tokio_handle: Arc<Handle>) -> Self {
        let graphql = Graphql {
            config: config.clone(),
            schema: Arc::new(schema::create()),
            tokio_remote: Arc::new(tokio_handle.remote().clone()),
        };
        
        Http {
            router: Arc::new(router::create_router()),
            graphql: Arc::new(graphql),
            graphql_thread_pool: Arc::new(CpuPool::new(config.gateway.graphql_thread_pool_size)),
        }
    }
}

impl juniper::Context for Graphql {}
