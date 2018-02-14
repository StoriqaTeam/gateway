use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

pub enum Model {
    User,
    JWT,
    Store,
    Product,
    UserRoles,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::User => write!(f, "user"),
            Model::JWT => write!(f, "jwt"),
            Model::Store => write!(f, "store"),
            Model::Product => write!(f, "product"),
            Model::UserRoles => write!(f, "user_roles"),
        }
    }
}

impl FromStr for Model {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Model::User),
            "jwt" => Ok(Model::JWT),
            "store" => Ok(Model::Store),
            "product" => Ok(Model::Product),
            "user_roles" => Ok(Model::UserRoles),
            _ => {
                return Err(FieldError::new(
                    "Unknown model",
                    graphql_value!({ "code": 300, "details": { 
                        format!("Can not resolve model name. Unknown model: '{}'", s) 
                        }}),
                ))
            }
        }
    }
}

impl Model {
    pub fn to_url(&self) -> String {
        match *self {
            Model::User => "users".to_string(),
            Model::JWT => "jwt".to_string(),
            Model::Store => "stores".to_string(),
            Model::Product => "products".to_string(),
            Model::UserRoles => "user_roles".to_string(),
        }
    }
}
