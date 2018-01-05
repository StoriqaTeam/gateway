use juniper;
use juniper::FieldResult;
use hyper::Method;

use super::context::Context;
use futures::Future;
use std::fmt;
use base64::encode as encode_to_base64;
use base64::decode as decode_from_base64;
use juniper::FieldError;
use std::str::FromStr;

pub struct Query;
pub struct Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    let query = Query {};
    let mutation = Mutation {};
    Schema::new(query, mutation)
}

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

enum Node {
    User(User),
}

graphql_interface!(Node: () as "Node" |&self| {
    description: "The Node interface contains a single field, 
        id, which is a ID!. The node root field takes a single argument, 
        a ID!, and returns a Node. These two work in concert to allow refetching."
    
    field id() -> String {
        match *self {
            Node::User(User { ref id, .. })  => ID::new(Services::Users, Models::User, *id).to_string(),
        }
    }

    instance_resolvers: |_| {
        &User => match *self { Node::User(ref h) => Some(h), _ => None },
    }
});

graphql_object!(User: () as "User" |&self| {
    description: "User's profile."

    interfaces: [&Node]

    field id() -> String as "Unique id"{
        ID::new(Services::Users, Models::User, self.id).to_string()
    }

    field raw_id() -> String as "Unique id"{
        self.id.to_string()
    }

    field email() -> String as "Email" {
        self.email.clone()
    }

    field isActive() -> bool as "If the user was disabled (deleted), isActive is false" {
        self.is_active
    }

});

#[derive(GraphQLEnum)]
#[graphql(name = "Provider", description = "Token providers")]
enum Provider {
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

enum Services {
    Users,
}

impl fmt::Display for Services {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Services::Users => write!(f, "users"),
        }
    }
}

impl FromStr for Services {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "users" => Ok(Services::Users),
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

enum Models {
    User,
}

impl fmt::Display for Models {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Models::User => write!(f, "user"),
        }
    }
}

impl FromStr for Models {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Models::User),
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

struct ID {
    service: Services,
    model: Models,
    raw_id: i32,
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            encode_to_base64(&*format!("{}_{}_{}", self.service, self.model, self.raw_id))
        )
    }
}

impl FromStr for ID {
    type Err = FieldError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let base64 = decode_from_base64(&*id).map_err(|err| {
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

        let service = Services::from_str(v[0])?;
        let model = Models::from_str(v[1])?;
        let raw_id = v[2].parse::<i32>().map_err(|err| {
            FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { err.to_string() }}),
            )
        })?;
        Ok(ID {
            service: service,
            model: model,
            raw_id: raw_id,
        })
    }
}

impl ID {
    fn new(service: Services, model: Models, id: i32) -> ID {
        ID {
            service: service,
            model: model,
            raw_id: id,
        }
    }
}

graphql_object!(Query: Context |&self| {

    description: "Top level query.

    Remote mark

    Some fields are marked as `Remote`. That means that they are
    part of microservices and their fetching can fail.
    In this case null will be returned (even if o/w
    type signature declares not-null type) and corresponding errors
    will be returned in errors section. Each error is guaranteed
    to have a `code` field and `details field`.

    Codes:
    - 100 - microservice responded,
    but with error http status. In this case `details` is guaranteed
    to have `status` field with http status and
    probably some additional details.

    - 200 - there was a network error while connecting to microservice.

    - 300 - there was a parse error - that usually means that
    graphql couldn't parse api json response
    (probably because of mismatching types on graphql and microservice)
    or api url parse failed.

    - 400 - Unknown error."

    field apiVersion() -> &str as "Current api version." {
        "1.0"
    }

    field user(&executor, id: String as "Id of a user.") -> FieldResult<User> as "Fetches user by id." {
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = format!("{}/{}/{}", 
            context.config.users_microservice.url.clone(), 
            Services::Users, 
            identifier.raw_id);

        context.http_client.request::<User>(Method::Get, url, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field users(&executor, from: String as "Starting id", count: i32 as "Count of users") -> FieldResult<Vec<User>> as "Fetches users using from and count." {
        let context = executor.context();
        let identifier = ID::from_str(&*from)?;
        let url = format!("{}/{}/?from={}&count={}",
        context.config.users_microservice.url.clone(),
        Services::Users, 
        identifier.raw_id, 
        count);

        context.http_client.request::<Vec<User>>(Method::Get, url, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field node(&executor, id: String as "Id of a user.") -> FieldResult<Node> as "Fetches graphql interface node by id."  {
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = format!("{}/{}/{}", 
            context.config.users_microservice.url.clone(),
            Services::Users, 
            identifier.raw_id);
        match identifier.service {
            Services::Users => {
                        context.http_client.request::<User>(Method::Get, url, None)
                            .map(|res| Node::User(res))
                            .or_else(|err| Err(err.to_graphql()))
                            .wait()
            }
        }
        
    }

});

graphql_object!(Mutation: Context |&self| {
     
    description: "Top level mutation.

    Codes:
    - 100 - microservice responded,
    but with error http status. In this case `details` is guaranteed
    to have `status` field with http status and
    probably some additional details.

    - 200 - there was a network error while connecting to microservice.

    - 300 - there was a parse error - that usually means that
    graphql couldn't parse api json response
    (probably because of mismatching types on graphql and microservice)
    or api url parse failed.

    - 400 - Unknown error."

    field createUser(&executor, email: String as "Email of a user.", password: String as "Password of a user.") -> FieldResult<User> as "Creates new user." {
        let context = executor.context();
        let url = format!("{}/{}", 
        context.config.users_microservice.url.clone(),
        Services::Users);
        let user = json!({"email": email, "password": password});
        let body: String = user.to_string();

        context.http_client.request::<User>(Method::Post, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field updateUser(&executor, id: String as "Id of a user." , email: String as "New email of a user.") -> FieldResult<User>  as "Updates existing user."{
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = format!("{}/{}/{}", 
            context.config.users_microservice.url.clone(), 
            Services::Users, 
            identifier.raw_id);
        let user = json!({"email": email});
        let body: String = user.to_string();

        context.http_client.request::<User>(Method::Put, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field deactivateUser(&executor, id: String as "Id of a user.") -> FieldResult<User>  as "Deactivates existing user." {
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = format!("{}/{}/{}", 
            context.config.users_microservice.url.clone(), 
            Services::Users, 
            identifier.raw_id);

        context.http_client.request::<User>(Method::Delete, url, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }


    field getJWTByEmail(&executor, email: String as "Email of a user.", password: String as "Password of a user") -> FieldResult<JWT> as "Get JWT Token by email." {
        let context = executor.context();
        let url = format!("{}/jwt/email", context.config.users_microservice.url.clone());
        let account = json!({"email": email, "password": password});
        let body: String = account.to_string();

        context.http_client.request::<JWT>(Method::Post, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field getJWTByProvider(&executor, provider: Provider as "Token provider", token: String as "Token.") -> FieldResult<JWT> as "Get JWT Token from token provider." {
        let context = executor.context();
        let url = format!("{}/jwt/{}", context.config.users_microservice.url.clone(), provider.to_string());
        let oauth = json!({"token": token});
        let body: String = oauth.to_string();

        context.http_client.request::<JWT>(Method::Post, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

});
