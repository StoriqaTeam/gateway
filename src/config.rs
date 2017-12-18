use std::str::FromStr;
use std::env;

#[derive(Debug)]
pub enum Env {
    production,
    develop,
    testing,
    unconfigured,
}

impl FromStr for Env {
    type Err = ();

    fn from_str(s: &str) -> Result<Env, ()> {
        match s {
            "production" => Ok(Env::production),
            "develop" => Ok(Env::develop),
            "testing" => Ok(Env::testing),
            "unconfigured" => Ok(Env::unconfigured),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub users_url: String,
    pub store_url: String,
    pub orders_url: String,
    pub billing_url: String,
    pub environment_name: Env,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            users_url: env::var("USERS_MICROSERVICE_URL")
                .expect("USERS_MICROSERVICE_URL must be set"),
            store_url: env::var("STORE_MICROSERVICE_URL")
                .expect("STORE_MICROSERVICE_URL must be set"),
            orders_url: env::var("ORDERS_MICROSERVICE_URL")
                .expect("ORDERS_MICROSERVICE_URL must be set"),
            billing_url: env::var("BILLING_MICROSERVICE_URL")
                .expect("BILLING_MICROSERVICE_URL must be set"),
            environment_name: Env::unconfigured,
        }
    }
}

impl Config {
    pub fn from(config_name: Env) -> Result<Config, String> {
        // We'll start with a base config that sets some defaults and then apply the chosen app config.
        match config_name {
            Env::production => Ok(Config::production_config()),
            Env::develop => Ok(Config::develop_config()),
            Env::testing => Ok(Config::testing_config()),
            _ => Err(format!("No valid config chosen: {:?}", config_name)),
        }
    }

    fn production_config() -> Config {
        Config {
            environment_name: Env::production,
            ..Default::default()
        }
    }

    fn develop_config() -> Config {
        Config {
            environment_name: Env::develop,
            ..Default::default()
        }
    }

    fn testing_config() -> Config {
        Config {
            environment_name: Env::testing,
            ..Default::default()
        }
    }
}
