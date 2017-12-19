use juniper;
use pool::Pool;
use config::Config;
use settings;


pub struct Context {
    pub users_connection_pool: Pool,
    pub config: Config,
}

impl Context {
    pub fn new() -> Self {
        let config = settings::get().unwrap();
        let users_url = config.get::<String>("users_microservice.url").unwrap();
        let users = Pool::new(users_url);
        Context {
            users_connection_pool: users,
            config: config,
        }
    }
}

impl juniper::Context for Context {}
