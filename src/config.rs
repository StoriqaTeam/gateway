use std::env;


#[derive(Debug)]
pub struct Config {
    pub users_url: String,
    pub store_url: String,
    pub orders_url: String,
    pub billing_url: String,
    pub environment_name: String,
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
            environment_name: String::from("unconfigured"),
        }
    }
}

impl Config {
    pub fn from(config_name: &str) -> Result<Config, String> {
        // We'll start with a base config that sets some defaults and then apply the chosen app config.
        match config_name {
            "production" => Ok(Config::production_config()),
            "staging" => Ok(Config::staging_config()),
            "develop" => Ok(Config::develop_config()),
            "testing" => Ok(Config::testing_config()),
            "local" => Ok(Config::local_config()),
            _ => Err(format!("No valid config chosen: {}", config_name)),
        }
    }

    fn production_config() -> Config {
        Config {
            environment_name: String::from("production"),
            ..Default::default()
        }
    }

    fn staging_config() -> Config {
        Config {
            environment_name: String::from("staging"),
            ..Default::default()
        }
    }

    fn develop_config() -> Config {
        Config {
            environment_name: String::from("develop"),
            ..Default::default()
        }
    }

    fn testing_config() -> Config {
        Config {
            environment_name: String::from("testing"),
            ..Default::default()
        }
    }

    fn local_config() -> Config {
        Config {
            environment_name: String::from("local"),
            ..Default::default()
        }
    }
}
