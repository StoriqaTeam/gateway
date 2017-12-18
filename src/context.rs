use juniper;
use pool::Pool;
use config::Config;
use tokio_core::reactor::Handle;


pub struct Context {
    pub users: Pool,
}

impl Context {
    pub fn new(config: Config, handle: &Handle) -> Self {
        let users = Pool::new(config.users_url, handle);
        Context { users: users }
    }
}

impl juniper::Context for Context {}
