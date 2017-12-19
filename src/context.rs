use juniper;
use pool::Pool;
use settings::Settings;


pub struct Context {
    pub users_connection_pool: Pool,
    pub config: Settings,
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        let users_url = settings.users_microservice.url.clone();
        let users = Pool::new(users_url);
        Context {
            users_connection_pool: users,
            config: settings,
        }
    }
}

impl juniper::Context for Context {}
