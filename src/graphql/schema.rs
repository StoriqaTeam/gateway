use std::str::FromStr;

use juniper;
use juniper::FieldResult;
use hyper::{Method, StatusCode};
use futures::Future;
use juniper::ID as GraphqlID;

use super::context::Context;
use super::model::{ID, Service, Model, Provider, User, Node, JWT, Viewer};
use ::http::client::{Error, ErrorMessage};



pub struct Query;
pub struct Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    let query = Query {};
    let mutation = Mutation {};
    Schema::new(query, mutation)
}

graphql_interface!(Node: () as "Node" |&self| {
    description: "The Node interface contains a single field, 
        id, which is a ID!. The node root field takes a single argument, 
        a ID!, and returns a Node. These two work in concert to allow refetching."
    
    field id() -> GraphqlID {
        match *self {
            Node::User(User { ref id, .. })  => ID::new(Service::Users, Model::User, *id).to_string().into(),
        }
    }

    instance_resolvers: |_| {
        &User => match *self { Node::User(ref h) => Some(h), _ => None },
    }
});

graphql_object!(User: () as "User" |&self| {
    description: "User's profile."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Users, Model::User, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field email() -> String as "Email" {
        self.email.clone()
    }

    field isActive() -> bool as "If the user was disabled (deleted), isActive is false" {
        self.is_active
    }

});


graphql_object!(Viewer: Context as "Viewer" |&self| {
    description: "Viewer for users.
    To access users data one must receive viewer object, 
    by passing jwt in bearer authentification header of http request.
    All requests without it or with wrong jwt will recieve null."

    field user(&executor, id: GraphqlID as "Id of a user.") -> FieldResult<User> as "Fetches user by id." {
        let context = executor.context();

        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request::<User>(Method::Get, url, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field users(&executor, from: GraphqlID as "Starting id", count: i32 as "Count of users") -> FieldResult<Vec<User>> as "Fetches users using from and count." {
        let context = executor.context();

        let identifier = ID::from_str(&*from)?;
        let url = format!("{}/{}/?from={}&count={}",
            Service::Users.to_url(&context.config), 
            Model::User.to_url(),
            identifier.raw_id,
            count);

        context.http_client.request::<Vec<User>>(Method::Get, url, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field node(&executor, id: GraphqlID as "Id of a user.") -> FieldResult<Node> as "Fetches graphql interface node by id."  {
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        match (&identifier.service, &identifier.model) {
            (&Service::Users, _) => {
                            context.http_client.request::<User>(Method::Get, identifier.url(&context.config), None)
                                .map(|res| Node::User(res))
                                .or_else(|err| Err(err.to_graphql()))
                                .wait()
            }
        }
        
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

    field viewer(&executor) -> FieldResult<Viewer> as "Fetches viewer for users." {
        let context = executor.context();

        match context.user {
            Some(_) => return Ok(Viewer{}),
            None => return Err (
                Error::Api( 
                    StatusCode::Unauthorized, 
                    Some(ErrorMessage { code: 401, message: "Authentification of Json web token failure".to_string() })
                    )
                .to_graphql())
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
            Service::Users.to_url(&context.config),
            Model::User.to_url());
        let user = json!({"email": email, "password": password});
        let body: String = user.to_string();

        context.http_client.request::<User>(Method::Post, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field updateUser(&executor, id: GraphqlID as "Id of a user." , email: String as "New email of a user.") -> FieldResult<User>  as "Updates existing user."{
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);
        let user = json!({"email": email});
        let body: String = user.to_string();

        context.http_client.request::<User>(Method::Put, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field deactivateUser(&executor, id: GraphqlID as "Id of a user.") -> FieldResult<User>  as "Deactivates existing user." {
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request::<User>(Method::Delete, url, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }


    field getJWTByEmail(&executor, email: String as "Email of a user.", password: String as "Password of a user") -> FieldResult<JWT> as "Get JWT Token by email." {
        let context = executor.context();
        let url = format!("{}/{}/email", 
            Service::Users.to_url(&context.config),
            Model::JWT.to_url());
        let account = json!({"email": email, "password": password});
        let body: String = account.to_string();

        context.http_client.request::<JWT>(Method::Post, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field getJWTByProvider(&executor, provider: Provider as "Token provider", token: String as "Token.") -> FieldResult<JWT> as "Get JWT Token from token provider." {
        let context = executor.context();
        let url = format!("{}/{}/{}", 
            Service::Users.to_url(&context.config), 
            Model::JWT.to_url(),
            provider);
        let oauth = json!({"token": token});
        let body: String = oauth.to_string();

        context.http_client.request::<JWT>(Method::Post, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

});
