use juniper;
use juniper::FieldResult;
use hyper::Method;

use super::context::Context;
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
    
    field id() -> &i32 {
        match *self {
            Node::User(User { ref id, .. })  => id,
        }
    }

    instance_resolvers: |_| {
        &User => match *self { Node::User(ref h) => Some(h), _ => None },
    }
});


graphql_object!(User: () as "User" |&self| {
    description: "User's profile."

    interfaces: [&Node]

    field id() -> &i32 as "Unique id"{
        &self.id
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
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field users(&executor, from: i32 as "Starting id", count: i32 as "Count of users") -> FieldResult<Vec<User>> as "Fetches users using from and count." {
        let context = executor.context();
        let url = format!("{}/users/?from={}&count={}" ,context.config.users_microservice.url.clone(),
        from, count);

        context.http_client.request::<Vec<User>>(Method::Get, url, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field node(&executor, id: i32 as "Id of a user.") -> FieldResult<Option<Node>> as "Fetches graphql interface node by id. Remote."  {
        let context = executor.context();
        let url = format!("{}/users/{}", context.config.users_microservice.url.clone(), id);
        context.http_client.request::<User>(Method::Get, url, None)
            .map(|res| Some(Node::User(res)))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
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
        let url = format!("{}/users", context.config.users_microservice.url.clone());
        let user = json!({"email": email, "password": password});
        let body: String = user.to_string();

        context.http_client.request::<User>(Method::Post, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field updateUser(&executor, id: i32 as "Id of a user." , email: String as "New email of a user.") -> FieldResult<User>  as "Updates existing user."{
        let context = executor.context();
        let url = format!("{}/users/{}", context.config.users_microservice.url.clone(), id);
        let user = json!({"email": email});
        let body: String = user.to_string();

        context.http_client.request::<User>(Method::Put, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field deactivateUser(&executor, id: i32 as "Id of a user.") -> FieldResult<User>  as "Deactivates existing user." {
        let context = executor.context();
        let url = format!("{}/users/{}", context.config.users_microservice.url.clone(), id);

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
        let provider = match provider {
            Provider::Facebook => "facebook",
            Provider::Google => "google"
        };
        let url = format!("{}/jwt/{}", context.config.users_microservice.url.clone(), provider);
        let oauth = json!({"token": token});
        let body: String = oauth.to_string();

        context.http_client.request::<JWT>(Method::Post, url, Some(body))
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

});
