use juniper;
use schema;
use router;
use futures_cpupool::CpuPool;
use settings::Settings;
use tokio_core::reactor::{Remote, Handle};

pub struct Context {
    pub config: Settings,
    pub schema: schema::Schema,
    pub router: router::Router,
    pub graphql_thread_pool: CpuPool,
    pub tokio_handle: Handle,
}

unsafe impl Sync for Context {}
unsafe impl Send for Context {}

impl Context {
    pub fn new(settings: Settings, tokio_handle: Handle) -> Self {
        let schema = schema::create();
        let router = router::Router::new();
        let graphql_thread_pool = CpuPool::new(settings.gateway.graphql_thread_pool_size);
        
        Context {
            config: settings,
            schema,
            router,
            graphql_thread_pool,
            tokio_handle,
        }
    }
}

impl juniper::Context for Context {}
