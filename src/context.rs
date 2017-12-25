use juniper;
use schema;
use router;
use futures_cpupool::CpuPool;
use settings::Settings;
use tokio_core::reactor::{Handle, Remote};
use std::sync::Arc;

pub struct Graphql {
    pub settings: Arc<Settings>,
    pub schema: Arc<schema::Schema>,
    pub tokio_remote: Arc<Remote>,
}

pub struct Http {
    pub router: Arc<router::Router>,
    pub tokio_handle: Arc<Handle>,
    pub graphql: Arc<Graphql>,
    pub thread_pool: Arc<CpuPool>,
}

impl Http {
    pub fn new(settings: Arc<Settings>, tokio_handle: Arc<Handle>) -> Self {
        let graphql = Graphql {
            settings: settings.clone(),
            schema: Arc::new(schema::create()),
            tokio_remote: Arc::new(tokio_handle.remote().clone()),
        };
        
        Http {
            router: Arc::new(router::Router::new()),
            tokio_handle,
            graphql: Arc::new(graphql),
            thread_pool: Arc::new(CpuPool::new(settings.gateway.graphql_thread_pool_size)),
        }
    }
}

impl juniper::Context for Graphql {}
