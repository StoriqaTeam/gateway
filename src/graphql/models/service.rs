use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

use config::Config;

pub enum Service {
    Users,
    Stores
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Service::Users => write!(f, "users"),
            Service::Stores => write!(f, "stores"),
        }
    }
}

impl FromStr for Service {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "users" => Ok(Service::Users),
            "stores" => Ok(Service::Stores),
            _ => {
                return Err(FieldError::new(
                    "Unknown service",
                    graphql_value!({ "code": 300, "details": { 
                        format!("Can not resolve service name. Unknown service: '{}'", s) 
                        }}),
                ))
            }
        }
    }
}

impl Service {
    pub fn to_url(&self, config: &Config) -> String {
        match *self {
            Service::Users => config.users_microservice.url.clone(),
            Service::Stores => config.stores_microservice.url.clone(),
        }
    }
}