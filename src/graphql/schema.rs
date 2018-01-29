use std::str::FromStr;
use std::cmp;

use juniper;
use juniper::FieldResult;
use super::context::Context;
use super::model::{ID, Service, Model, Provider, User, Node, JWT, Connection, Edge, PageInfo, Viewer, ProviderOauth,Product, Store};
use hyper::{Method, StatusCode};
use futures::Future;
use juniper::ID as GraphqlID;
use serde_json;

use ::http::client::{Error, ErrorMessage};



pub struct Query;
pub struct Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

const MIN_ID: i32 = 0; 

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
            Node::Store(Store { ref id, .. })  => ID::new(Service::Stores, Model::Store, *id).to_string().into(),
            Node::Product(Product { ref id, .. })  => ID::new(Service::Stores, Model::Product, *id).to_string().into(),
        }
    }

    instance_resolvers: |_| {
        &User => match *self { Node::User(ref h) => Some(h), _ => None },
        &Store => match *self { Node::Store(ref h) => Some(h), _ => None },
        &Product => match *self { Node::Product(ref h) => Some(h), _ => None },
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

graphql_object!(Store: () as "Store" |&self| {
    description: "Store's profile."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Store, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field full_name() -> String as "Full Name" {
        self.full_name.clone()
    }

    field isActive() -> bool as "If the store was disabled (deleted), isActive is false" {
        self.is_active
    }

});

graphql_object!(Product: () as "Product" |&self| {
    description: "Product's profile."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Product, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field full_name() -> String as "Full Name" {
        self.full_name.clone()
    }

    field isActive() -> bool as "If the product was disabled (deleted), isActive is false" {
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

graphql_object!(Connection<Store>: () as "StoresConnection" |&self| {
    description:"Stores Connection"

    field edges() -> Vec<Edge<Store>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<Store>: () as "StoresEdge" |&self| {
    description:"Stores Edge"
    
    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> Store {
        self.node.clone()
    }
});


graphql_object!(Connection<Product>: () as "ProductsConnection" |&self| {
    description:"Products Connection"

    field edges() -> Vec<Edge<Product>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<Product>: () as "ProductsEdge" |&self| {
    description:"Products Edge"
    
    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> Product {
        self.node.clone()
    }
});

graphql_object!(Viewer: Context as "Viewer" |&self| {
    description: "Viewer for users, stores, products.
    To access users data one must receive viewer object, 
    by passing jwt in bearer authentification header of http request.
    All requests without it or with wrong jwt will recieve null."

    field user(&executor, id: GraphqlID as "Id of a user.") -> FieldResult<User> as "Fetches user by id." {
        let context = executor.context();
        let user = context.user.clone().unwrap();

        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<User>(Method::Get, url, None, user)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field users(&executor, first = None : Option<i32> as "First edges", after = None : Option<GraphqlID>  as "Id of a user") -> FieldResult<Connection<User>> as "Fetches users using relay connection." {
        let context = executor.context();
        let user = context.user.clone().unwrap();
        
        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };
        
        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/?from={}&count={}",
            Service::Users.to_url(&context.config), 
            Model::User.to_url(),
            raw_id,
            first + 1);

        context.http_client.request_with_auth_header::<Vec<User>>(Method::Get, url, None, user)
            .or_else(|err| Err(err.to_graphql()))
            .map (|users| {
                let mut user_edges: Vec<Edge<User>> = users
                    .into_iter()
                    .map(|user| Edge::new(
                                juniper::ID::from(ID::new(Service::Users, Model::User, user.id.clone()).to_string()),
                                user.clone()
                            ))
                    .collect();
                let has_next_page = user_edges.len() as i32 == first + 1;
                if has_next_page { 
                    user_edges.pop(); 
                };
                let has_previous_page = true;
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(user_edges, page_info)
            })
            .wait()
    }

    field store(&executor, id: GraphqlID as "Id of a store.") -> FieldResult<Store> as "Fetches store by id." {
        let context = executor.context();
        let user = context.user.clone().unwrap();

        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<Store>(Method::Get, url, None, user)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field stores(&executor, first = None : Option<i32> as "First edges", after = None : Option<GraphqlID>  as "Id of a store") -> FieldResult<Connection<Store>> as "Fetches stores using relay connection." {
        let context = executor.context();
        let user = context.user.clone().unwrap();
        
        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };
        
        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/?from={}&count={}",
            Service::Stores.to_url(&context.config), 
            Model::Store.to_url(),
            raw_id,
            first + 1);

        context.http_client.request_with_auth_header::<Vec<Store>>(Method::Get, url, None, user)
            .or_else(|err| Err(err.to_graphql()))
            .map (|stores| {
                let mut store_edges: Vec<Edge<Store>> = stores
                    .into_iter()
                    .map(|store| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::Store, store.id.clone()).to_string()),
                                store.clone()
                            ))
                    .collect();
                let has_next_page = store_edges.len() as i32 == first + 1;
                if has_next_page { 
                    store_edges.pop(); 
                };
                let has_previous_page = true;
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(store_edges, page_info)
            })
            .wait()
    }

    field product(&executor, id: GraphqlID as "Id of a product.") -> FieldResult<Product> as "Fetches product by id." {
        let context = executor.context();
        let user = context.user.clone().unwrap();

        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<Product>(Method::Get, url, None, user)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field products(&executor, first = None : Option<i32> as "First edges", after = None : Option<GraphqlID>  as "Id of a product") -> FieldResult<Connection<Product>> as "Fetches products using relay connection." {
        let context = executor.context();
        let user = context.user.clone().unwrap();
        
        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };
        
        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/?from={}&count={}",
            Service::Stores.to_url(&context.config), 
            Model::Product.to_url(),
            raw_id,
            first + 1);

        context.http_client.request_with_auth_header::<Vec<Product>>(Method::Get, url, None, user)
            .or_else(|err| Err(err.to_graphql()))
            .map (|products| {
                let mut product_edges: Vec<Edge<Product>> = products
                    .into_iter()
                    .map(|product| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::Product, product.id.clone()).to_string()),
                                product.clone()
                            ))
                    .collect();
                let has_next_page = product_edges.len() as i32 == first + 1;
                if has_next_page { 
                    product_edges.pop(); 
                };
                let has_previous_page = true;
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(product_edges, page_info)
            })
            .wait()
    }

    field current_user(&executor) -> FieldResult<User> as "Fetches current user information." {
        let context = executor.context();
        let user = context.user.clone().unwrap();
        let url = format!("{}/{}/current",
            Service::Users.to_url(&context.config), 
            Model::User.to_url());
        context.http_client.request_with_auth_header::<User>(Method::Get, url, None, user)
                    .or_else(|err| Err(err.to_graphql()))
                    .wait()
                            
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

    field node(&executor, id: GraphqlID as "Id of a node.") -> FieldResult<Node> as "Fetches graphql interface node by id."  {
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        match (&identifier.service, &identifier.model) {
            (&Service::Users, _) => {
                            context.http_client.request::<User>(Method::Get, identifier.url(&context.config), None, None)
                                .map(|res| Node::User(res))
                                .or_else(|err| Err(err.to_graphql()))
                                .wait()
            },
            (&Service::Stores, &Model::Store) => {
                            context.http_client.request::<Store>(Method::Get, identifier.url(&context.config), None, None)
                                .map(|res| Node::Store(res))
                                .or_else(|err| Err(err.to_graphql()))
                                .wait()
            },
            (&Service::Stores, &Model::Product) => {
                            context.http_client.request::<Product>(Method::Get, identifier.url(&context.config), None, None)
                                .map(|res| Node::Product(res))
                                .or_else(|err| Err(err.to_graphql()))
                                .wait()
            },
            (&Service::Stores, _) => {
                            context.http_client.request::<Store>(Method::Get, identifier.url(&context.config), None, None)
                                .map(|res| Node::Store(res))
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

        context.http_client.request::<User>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field updateUser(&executor, id: GraphqlID as "Id of a user." , email: String as "New email of a user.") -> FieldResult<User>  as "Updates existing user."{
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);
        let user = json!({"email": email});
        let body: String = user.to_string();

        context.http_client.request::<User>(Method::Put, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field deactivateUser(&executor, id: GraphqlID as "Id of a user.") -> FieldResult<User>  as "Deactivates existing user." {
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request::<User>(Method::Delete, url, None, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field createStore(&executor, full_name: String as "Full name of a store.") -> FieldResult<Store> as "Creates new store." {
        let context = executor.context();
        let url = format!("{}/{}", 
            Service::Stores.to_url(&context.config),
            Model::Store.to_url());
        let store = json!({"full_name": full_name});
        let body: String = store.to_string();

        context.http_client.request::<Store>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field updateStore(&executor, id: GraphqlID as "Id of a store." , full_name: String as "New name of a store.") -> FieldResult<Store>  as "Updates existing store."{
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);
        let store = json!({"full_name": full_name});
        let body: String = store.to_string();

        context.http_client.request::<Store>(Method::Put, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field deactivateStore(&executor, id: GraphqlID as "Id of a store.") -> FieldResult<Store>  as "Deactivates existing store." {
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request::<Store>(Method::Delete, url, None, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field createProduct(&executor, full_name: String as "Full name of a product.") -> FieldResult<Product> as "Creates new product." {
        let context = executor.context();
        let url = format!("{}/{}", 
            Service::Stores.to_url(&context.config),
            Model::Product.to_url());
        let product = json!({"full_name": full_name});
        let body: String = product.to_string();

        context.http_client.request::<Product>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field updateProduct(&executor, id: GraphqlID as "Id of a product." , full_name: String as "New name of a product.") -> FieldResult<Product>  as "Updates existing product."{
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);
        let product = json!({"full_name": full_name});
        let body: String = product.to_string();

        context.http_client.request::<Product>(Method::Put, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field deactivateProduct(&executor, id: GraphqlID as "Id of a product.") -> FieldResult<Product>  as "Deactivates existing product." {
        let context = executor.context();
        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request::<Product>(Method::Delete, url, None, None)
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

        context.http_client.request::<JWT>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field getJWTByProvider(&executor, provider: Provider as "Token provider", token: String as "Token.") -> FieldResult<JWT> as "Get JWT Token from token provider." {
        let context = executor.context();
        let url = format!("{}/{}/{}", 
            Service::Users.to_url(&context.config), 
            Model::JWT.to_url(),
            provider);
        let oauth = ProviderOauth { token: token };
        let body: String = serde_json::to_string(&oauth).unwrap();

        context.http_client.request::<JWT>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

});
