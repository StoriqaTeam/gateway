use std::fmt;
use base64::encode;
use base64::decode;
use juniper::FieldError;
use std::str::FromStr;
use config::Config;
use juniper;

#[derive(GraphQLObject, Deserialize, Debug)]
#[graphql(description = "JWT Token")]
pub struct JWT {
    #[graphql(description = "Token")] 
    pub token: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub is_active: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub store_id: i32,
    pub name: String,
    pub is_active: bool,
}

pub enum Node {
    User(User),
    Store(Store),
    Product(Product)
}

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

pub enum Model {
    User,
    JWT,
    Store,
    Product
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::User => write!(f, "user"),
            Model::JWT => write!(f, "jwt"),
            Model::Store => write!(f, "store"),
            Model::Product => write!(f, "product"),
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
            Model::Store => "store".to_string(),
            Model::Product => "product".to_string(),
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
        format!(
            "{}/{}/{}",
            self.service.to_url(config),
            self.model.to_url(),
            self.raw_id
        )
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

#[derive(Clone)]
pub struct Edge<T> {
    pub cursor: juniper::ID,
    pub node: T,
}

impl<T> Edge<T> {
    pub fn new (cursor: juniper::ID, node: T) -> Self {
        Self {
            cursor: cursor,
            node:node
        }
    }
}


#[derive(GraphQLObject, Clone)]
#[graphql(name = "PageInfo", description = "Page Info from relay spec: https://facebook.github.io/relay/graphql/connections.htm")]
pub struct PageInfo {
    #[graphql(description = "has next page")] 
    pub has_next_page: bool,

    #[graphql(description = "has previous page")] 
    pub has_previous_page: bool,
}


pub struct Connection<T> {
    pub edges: Vec<Edge<T>>,
    pub page_info: PageInfo,
}

impl<T> Connection<T> {
    pub fn new (edges: Vec<Edge<T>>, page_info: PageInfo) -> Self {
        Self {
            edges: edges,
            page_info: page_info
        }
    }
}

pub struct Viewer;


/// Payload for creating JWT token by provider
#[derive(Serialize, Deserialize)]
pub struct ProviderOauth {
    pub token: String,
}