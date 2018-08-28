use std::env;

use stq_http;
use stq_logging::GrayLogConfig;
use stq_routes::service::Service as StqService;

use config_crate::{Config as RawConfig, ConfigError, Environment, File};

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
    pub http_timeout_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub gateway: Gateway,
    pub users_microservice: Microservice,
    pub stores_microservice: Microservice,
    pub saga_microservice: Microservice,
    pub orders_microservice: Microservice,
    pub billing_microservice: Microservice,
    pub warehouses_microservice: Microservice,
    pub notifications_microservice: Microservice,
    pub delivery_microservice: Microservice,
    pub jwt: JWT,
    pub cors: CORS,
    pub graylog: Option<GrayLogConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JWT {
    pub public_key_path: String,
    pub leeway: i64,
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
        let mut s = RawConfig::new();

        s.merge(File::with_name("config/base"))?;
        // Optional file specific for environment
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env.to_string())).required(false))?;

        // Add in settings from the environment (with a prefix of STQ_GATEWAY)
        s.merge(Environment::with_prefix("STQ_GATEWAY"))?;

        s.try_into()
    }

    pub fn to_http_config(&self) -> stq_http::client::Config {
        stq_http::client::Config {
            http_client_buffer_size: self.gateway.http_client_buffer_size,
            http_client_retries: self.gateway.http_client_retries,
            timeout_duration_ms: self.gateway.http_timeout_ms,
        }
    }

    pub fn service_url(&self, service: StqService) -> String {
        match service {
            StqService::Users => self.users_microservice.url.clone(),
            StqService::Stores => self.stores_microservice.url.clone(),
            StqService::Orders => self.orders_microservice.url.clone(),
            StqService::Warehouses => self.warehouses_microservice.url.clone(),
            StqService::Notifications => self.notifications_microservice.url.clone(),
            StqService::Billing => self.billing_microservice.url.clone(),
            StqService::Delivery => self.delivery_microservice.url.clone(),
        }
    }
}
