use std::env;

use stq_routes::service::Service as StqService;
use stq_http;

use config_crate::{Config as RawConfig, ConfigError, Environment, File};

enum Env {
    Development,
    Test,
    Production,
}

impl Env {
    fn new() -> Self {
        match env::var("RUN_MODE") {
            Ok(ref s) if s == "test" => Env::Test,
            Ok(ref s) if s == "production" => Env::Production,
            _ => Env::Development,
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            &Env::Development => "development",
            &Env::Production => "production",
            &Env::Test => "test",
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
    pub graphql_thread_pool_size: usize,
    pub http_client_buffer_size: usize,
    pub http_client_retries: usize,
    pub records_limit: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub gateway: Gateway,
    pub users_microservice: Microservice,
    pub stores_microservice: Microservice,
    pub orders_microservice: Microservice,
    pub billing_microservice: Microservice,
    pub jwt: JWT,
    pub cors: CORS,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JWT {
    pub secret_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CORS {
    pub domain: String,
    pub max_age: u32,
}

impl Config {
    /// Creates config from base.toml, which are overwritten by <env>.toml, where
    /// env is one of development, test, production. After that it could be overwritten
    /// by env variables like STQ_GATEWAY_URL (this will override `url` field in config)
    pub fn new() -> Result<Self, ConfigError> {
        let env = Env::new();
        let mut s = RawConfig::new();

        s.merge(File::with_name("config/base"))?;
        // Optional file specific for environment
        s.merge(File::with_name(&format!("config/{}", env.to_string())).required(false))?;

        // Add in settings from the environment (with a prefix of STQ_GATEWAY)
        s.merge(Environment::with_prefix("STQ_GATEWAY"))?;

        s.try_into()
    }

    pub fn to_http_config(&self) -> stq_http::client::Config {
        stq_http::client::Config {
            http_client_buffer_size: self.gateway.http_client_buffer_size,
            http_client_retries: self.gateway.http_client_retries,
        }
    }

    pub fn service_url(&self, service: StqService) -> String {
        match service {
            StqService::Users => self.users_microservice.url.clone(),
            StqService::Stores => self.stores_microservice.url.clone(),
        }
    }
}
