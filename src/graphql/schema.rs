use juniper;
use juniper::FieldResult;
use hyper::{Method, StatusCode};
use serde_json;

use super::context::Context;
use http;

pub struct Query;
pub struct Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    let query = Query {};
    let mutation = Mutation {};
    Schema::new(query, mutation)
}

#[derive(GraphQLObject, Deserialize, Debug)]
#[graphql(description = "User's profile")]
pub struct User {
    #[graphql(description = "Unique id")] 
    pub id: i32,
    #[graphql(description = "Email")] 
    pub email: String,
    #[graphql(name = "isActive", description = "If the user was disabled (deleted), isActive is false")]
    pub is_active: bool,
}

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field user(&executor, id: i32) -> FieldResult<Option<User>> {
        let context = executor.context();
        let url = format!("{}/users/{}", context.config.users_microservice.url.clone(), id);

        let res = match send(Method::Get, url, None, &context) {
            Ok(resp) => resp,
            Err(SchemaError::NotFound) => return Ok(None),
            Err(SchemaError::GraphQlError(e)) => return Err(e)
        };
        
        match serde_json::from_str::<User>(&res) {
            Ok(user) => Ok(Some(user)),
            Err(err) => {
                error!("Error deserializing user: {}", err);
                Err(
                    juniper::FieldError::new(
                        "Error deserializing response from users microservice",
                        graphql_value!("Probably mismatching client / server api versions? See logs for details."),
                    )
                )
            }
        }
    }
    
    field users(&executor, from: i32, to: i32) -> FieldResult<Vec<User>> {
        let context = executor.context();
        let user1 = User {
            id: 1,
            email: String::from("example@mail.com"),
            is_active: false,
        };

        let user2 = User {
            id: 2,
            email: String::from("elpmaxe@mail.com"),
            is_active: false,
        };
        let users = vec![user1, user2];
        Ok(users)
    }
});


//mutation {
//  createUser(name: "andy", email: "hope is a good thing") {
//    id
//  }
//}

graphql_object!(Mutation: Context |&self| {

    //POST /users - создать пользователя. + Механизм для подтверждения email, если //не через соцсети
    field createUser(&executor, name: String, email: String) -> FieldResult<User> {
        let context = executor.context();
        let user = User {
            id: 0,
            email,
            is_active: false,
        };
        Ok(user)
    }

    //PUT /users/:id - апдейт пользователя
    field updateUser(&executor,id: i32, name: String, email: String) -> FieldResult<User> {
        let context = executor.context();
        let user = User {
            id: 0,
            email,
            is_active: false,
        };
        Ok(user)
    }

    //DELETE /users/:id - удалить пользователя
    field deleteUser(&executor, id: i32) -> FieldResult<()> {
        let context = executor.context();
        Ok(())
    }
    
});


#[derive(Debug)]
pub enum SchemaError {
    GraphQlError(juniper::FieldError),
    NotFound,
}

pub type SchemaResult = Result<String, SchemaError>;


fn send(method: Method, url: String, body: Option<String>, context: &Context) -> SchemaResult {
    match context.http_client.send_sync(method, url, body) {
        Ok(resp) => Ok(resp),
        Err(http::client::Error::Api(StatusCode::NotFound, _)) => return Err(SchemaError::NotFound),
        Err(http::client::Error::Api(status, message)) => {
            let status = status.to_string();
            if let Some(http::client::ErrorMessage { code, message }) = message {
                let code = code.to_string();
                return Err(SchemaError::GraphQlError(juniper::FieldError::new(
                    "Error response from users microservice",
                    graphql_value!({ "status": status, "code": code, "message": message }),
                )));
            } else {
                return Err(SchemaError::GraphQlError(juniper::FieldError::new(
                    "Error response from users microservice",
                    graphql_value!({ "status": status }),
                )));
            }
        }
        Err(http::client::Error::Network(_)) => {
            return Err(SchemaError::GraphQlError(juniper::FieldError::new(
                "Error connecting to users microservice",
                graphql_value!("See logs for details."),
            )))
        }
        _ => {
            return Err(SchemaError::GraphQlError(juniper::FieldError::new(
                "Unknown error for request to users microservice",
                graphql_value!("See logs for details."),
            )))
        }
    }
}