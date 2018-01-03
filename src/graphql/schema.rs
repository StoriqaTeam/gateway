use juniper;
use juniper::FieldResult;
use hyper::{Method, StatusCode};

use super::context::Context;
use http;
use futures::Future;
use url::form_urlencoded;

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
    #[graphql(name = "Password", description = "Password")]
    pub password: String,
    #[graphql(name = "isActive", description = "If the user was disabled (deleted), isActive is false")]
    pub is_active: bool,
}

enum Node {
    User(User),
    
}

graphql_interface!(Node: () |&self| {
    field id() -> &i32 {
        match *self {
            Node::User (User { ref id, .. })  => id,
        }
    }

    instance_resolvers: |_| {
        &User => match *self { Node::User(ref user) => Some(user), _ => None },
    }
});


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

    field user(&executor, id: i32 as "Id of a user.") -> FieldResult<Option<User>> as "Fetches user by id. Remote." {
        let context = executor.context();
        let url = format!("{}/users/{}", context.config.users_microservice.url.clone(), id);

        context.http_client.request::<User>(Method::Get, url, None)
            .map(|res| Some(res))
            .or_else(|err| match err {
                http::client::Error::Api(StatusCode::NotFound, _) => Ok(None),
                err => Err(err.to_graphql())
            })
            .wait()
    }

    field users(&executor, from: i32 as "Starting id", count: i32 as "Count of users") -> FieldResult<Option<Vec<User>>> as "Fetches users using from and count." {
        let context = executor.context();
        let url = format!("{}/users" ,context.config.users_microservice.url.clone());

        let body: String = form_urlencoded::Serializer::new(String::new())
         .append_pair("from", &*from.to_string())
         .append_pair("count", &*count.to_string())
         .finish();
        context.http_client.request::<Vec<User>>(Method::Get, url, Some(body))
            .map(|res| Some(res))
            .or_else(|err| match err {
                http::client::Error::Api(StatusCode::NotFound, _) => Ok(None),
                err => Err(err.to_graphql())
            })
            .wait()
    }
});


graphql_object!(Mutation: Context |&self| {

    field createUser(&executor, email: String as "Email of a user.", password: String as "Password of a user.") -> FieldResult<Option<User>> as "Creates new user." {
        let context = executor.context();
        let url = format!("{}/users/", context.config.users_microservice.url.clone());
        let user = json!({"email": email, "password": password});
        let body: String = user.to_string();

        context.http_client.request::<User>(Method::Post, url, Some(body))
            .map(|res| Some(res))
            .or_else(|err| match err {
                http::client::Error::Api(StatusCode::NotFound, _) => Ok(None),
                err => Err(err.to_graphql())
            })
            .wait()
    }

    field updateUser(&executor, id: i32 as "Id of a user." , email: String as "New email of a user.") -> FieldResult<Option<User>>  as "Updates existing user."{
        let context = executor.context();
        let url = format!("{}/users/{}", context.config.users_microservice.url.clone(), id);
        let user = json!({"email": email});
        let body: String = user.to_string();

        context.http_client.request::<User>(Method::Put, url, Some(body))
            .map(|res| Some(res))
            .or_else(|err| match err {
                http::client::Error::Api(StatusCode::NotFound, _) => Ok(None),
                err => Err(err.to_graphql())
            })
            .wait()
    }

    field deactivateUser(&executor, id: i32 as "Id of a user.") -> FieldResult<Option<User>>  as "Deactivates existing user." {
        let context = executor.context();
        let url = format!("{}/users/{}", context.config.users_microservice.url.clone(), id);

        context.http_client.request::<User>(Method::Delete, url, None)
            .map(|res| Some(res))
            .or_else(|err| match err {
                http::client::Error::Api(StatusCode::NotFound, _) => Ok(None),
                err => Err(err.to_graphql())
            })
            .wait()
    }

});

