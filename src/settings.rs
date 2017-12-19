use std::env;
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct Microservice {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Gateway {
    pub url: String,
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
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/base"))?;

        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in settings from the environment (with a prefix of STQ_GATEWAY)
        s.merge(Environment::with_prefix("STQ_GATEWAY"))?;

        s.try_into()
    }
}
