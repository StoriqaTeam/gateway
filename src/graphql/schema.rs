use juniper;
use juniper::FieldResult;
use hyper::{Method, StatusCode};
use serde_json;

use super::context::Context;
use http;
use futures::Future;

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

        let result = context.http_client.send(Method::Get, url, None)
            .map(|res| Some(res))
            .or_else(|err| match err {
                http::client::Error::Api(StatusCode::NotFound, _) => Ok(None),
                err => Err(err)
            }).wait();

        match result {
            Ok(Some(response)) => {
                serde_json::from_str::<User>(&response)
                    .map(|user| Some(user))
                    .map_err(|err| http::client::Error::Parse(format!("{}", err)).to_graphql())
            },
            Ok(None) => Ok(None),
            Err(err) => Err(err.to_graphql()),
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

