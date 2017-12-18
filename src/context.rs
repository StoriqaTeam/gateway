use juniper;
use pool::Pool;
use config::Config;
use tokio_core::reactor::Handle;


pub struct Context {
    pub users: Pool,
    pub store: Pool,
    pub order: Pool,
    pub billing: Pool,
}

impl Context {
    pub fn new(config: Config, handle: &Handle) -> Self {
        let users = Pool::new(config.users_url, handle);
        let store = Pool::new(config.store_url, handle);
        let order = Pool::new(config.orders_url, handle);
        let billing = Pool::new(config.billing_url, handle);
        Context {
            users: users,
            store: store,
            order: order,
            billing: billing,
        }
    }
}

impl juniper::Context for Context {}
