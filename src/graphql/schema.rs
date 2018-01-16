use juniper;
use juniper::FieldResult;
use hyper::Method;
use std::cmp;

use super::context::Context;
use super::model::{ID, Service, Model, Provider, User, Node, JWT, Connection, Edge, PageInfo};
use futures::Future;
use juniper::ID as GraphqlID;
use std::str::FromStr;


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


graphql_object!(Connection<User>: () as "UsersConnection" |&self| {
    description:"Users Connection"

    field edges() -> Vec<Edge<User>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<User>: () as "UsersEdge" |&self| {
    description:"Users Edge"
    
    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> User {
        self.node.clone()
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

    field user(&executor, id: GraphqlID as "Id of a user.") -> FieldResult<User> as "Fetches user by id." {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            Service::Users.to_url(&context.config), 
            Model::User.to_url(),
            id.to_string());

        context.http_client.request::<User>(Method::Get, url, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field users(&executor, first = None : Option<i32> as "First edges", after = None : Option<GraphqlID>  as "Id of a user") -> FieldResult<Connection<User>> as "Fetches users using relay connection." {
        let context = executor.context();
        
        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => 1
        };
        
        let records_limit = context.config.gateway.records_limit;
        let first = match first {
            Some(f) => cmp::min(records_limit as i32, f),
            None => records_limit as i32
        };

        let url = format!("{}/{}/?from={}&count={}",
            Service::Users.to_url(&context.config), 
            Model::User.to_url(),
            raw_id,
            first + 1);

        context.http_client.request::<Vec<User>>(Method::Get, url, None)
            .or_else(|err| Err(err.to_graphql()))
            .map (|users| {
                let user_edges: Vec<Edge<User>> = users
                    .into_iter()
                    .map(|user| Edge::new(
                                juniper::ID::from(ID::new(Service::Users, Model::User, user.id.clone()).to_string()),
                                user.clone()
                            ))
                    .collect();
                let has_next_page = user_edges.len() as i32 == first + 1;
                let has_previous_page = true;
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(user_edges, page_info)
            })
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
