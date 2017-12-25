use std::env;
use config::{Config, ConfigError, Environment, File};

enum Env {
    Development,
    Test,
    Production
}

impl Env {
    fn new() -> Self {
        match env::var("RUN_MODE") {
            Ok(ref s) if s == "test" => Env::Test,
            Ok(ref s) if s == "production" => Env::Production,
            _ => Env::Development
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            &Env::Development => "development",
            &Env::Production => "production",
            &Env::Test => "test"
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Microservice {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Gateway {
    pub url: String,
    pub graphql_thread_pool_size: u32
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub gateway: Gateway,
    pub users_microservice: Microservice,
    pub store_microservice: Microservice,
    pub orders_microservice: Microservice,
    pub billing_microservice: Microservice,
}


impl Settings {
    /// Creates settings from base.toml, which are overwritten by <env>.toml, where
    /// env is one of development, test, production. After that it could be overwritten
    /// by env variables like STQ_GATEWAY_URL (this will override `url` field in settings)
    pub fn new() -> Result<Self, ConfigError> {
        let env = Env::new();
        let mut s = Config::new();

        s.merge(File::with_name("config/base"))?;
        // Optional file specific for environment
        s.merge(File::with_name(&format!("config/{}", env.to_string())).required(false))?;

        // Add in settings from the environment (with a prefix of STQ_GATEWAY)
        s.merge(Environment::with_prefix("STQ_GATEWAY"))?;

        s.try_into()
    }
}
