use std::fmt;
use base64::encode;
use base64::decode;
use juniper::FieldError;
use std::str::FromStr;
use ::config::Config;


#[derive(GraphQLObject, Deserialize, Debug)]
#[graphql(description = "JWT Token")]
pub struct JWT {
    #[graphql(description = "Token")] 
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub is_active: bool,
}

pub enum Node {
    User(User),
}

pub enum Service {
    Users,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Service::Users => write!(f, "users"),
        }
    }
}

impl FromStr for Service {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "users" => Ok(Service::Users),
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
            }
    }
}

pub enum Model {
    User,
    JWT
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::User => write!(f, "user"),
            Model::JWT => write!(f, "jwt"),
        }
    }
}

impl FromStr for Model {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Model::User),
            "jwt" => Ok(Model::JWT),
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
                Model::User => "users".to_owned(),
                Model::JWT => "jwt".to_owned(),
            }
    }
}

pub struct ID {
    pub service: Service,
    pub model: Model,
    pub raw_id: i32,
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            encode(&*format!("{}_{}_{}", self.service, self.model, self.raw_id))
        )
    }
}

impl FromStr for ID {
    type Err = FieldError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let base64 = decode(&*id).map_err(|err| {
            FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { err.to_string() }}),
            )
        })?;

        let id = String::from_utf8(base64).map_err(|err| {
            FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { err.to_string() }}),
            )
        })?;

        let v: Vec<&str> = id.split('_').collect();
        if v.len() != 3 {
            return Err(FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { "can not resolve service, model or id" }}),
            ));
        }

        let service = Service::from_str(v[0])?;
        let model = Model::from_str(v[1])?;
        let raw_id = v[2].parse::<i32>().map_err(|err| {
            FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { err.to_string() }}),
            )
        })?;
        Ok(ID::new(service, model, raw_id))
    }
}

impl ID {
    pub fn new(service: Service, model: Model, id: i32) -> ID {
        ID {
            service: service,
            model: model,
            raw_id: id,
        }
    }

    pub fn url(&self, config: &Config) -> String {
        format!("{}/{}/{}", 
            self.service.to_url(config), 
            self.model.to_url(), 
            self.raw_id) 
    }
}

#[derive(GraphQLEnum)]
#[graphql(name = "Provider", description = "Token providers")]
pub enum Provider {
    #[graphql(description = "Google")] 
    Google,
    #[graphql(description = "Facebook")] 
    Facebook,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Provider::Facebook => write!(f, "facebook"),
            Provider::Google => write!(f, "google"),
        }
    }
}
