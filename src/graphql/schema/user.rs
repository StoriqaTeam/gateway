use std::str::FromStr;
use std::cmp;

use juniper;
use juniper::FieldResult;
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use futures::Future;
use juniper::ID as GraphqlID;
use super::*;

const MIN_ID: i32 = 0;

graphql_object!(User: Context as "User" |&self| {
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

    field phone() -> Option<String> as "Phone" {
        self.phone.clone()
    }

    field first_name() -> Option<String> as "First name" {
        self.first_name.clone()
    }

    field last_name() -> Option<String> as "Last name" {
        self.last_name.clone()
    }

    field middle_name() -> Option<String> as "Middle name" {
        self.middle_name.clone()
    }

    field gender() -> Gender as "Gender" {
        self.gender.clone()
    }

    field birthdate() -> Option<String> as "Birthdate" {
        self.birthdate.clone()
    }

    field isActive() -> bool as "If the user was disabled (deleted), isActive is false" {
        self.is_active
    }


    field user(&executor, id: GraphqlID as "Id of a user.") -> FieldResult<User> as "Fetches user by id." {
        let context = executor.context();

        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<User>(Method::Get, url, None, context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field users(&executor, first = None : Option<i32> as "First edges", after = None : Option<GraphqlID>  as "Id of a user") -> FieldResult<Connection<User>> as "Fetches users using relay connection." {
        let context = executor.context();
        
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

        context.http_client.request_with_auth_header::<Vec<User>>(Method::Get, url, None, context.user.clone())
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

        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<Store>(Method::Get, url, None, context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field stores(&executor, first = None : Option<i32> as "First edges", after = None : Option<GraphqlID>  as "Id of a store") -> FieldResult<Connection<Store>> as "Fetches stores using relay connection." {
        let context = executor.context();
        
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

        context.http_client.request_with_auth_header::<Vec<Store>>(Method::Get, url, None, context.user.clone())
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

        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<Product>(Method::Get, url, None, context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field products(&executor, first = None : Option<i32> as "First edges", after = None : Option<GraphqlID>  as "Id of a product") -> FieldResult<Connection<Product>> as "Fetches products using relay connection." {
        let context = executor.context();
        
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

        context.http_client.request_with_auth_header::<Vec<Product>>(Method::Get, url, None, context.user.clone())
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

});

graphql_object!(Connection<User>: Context as "UsersConnection" |&self| {
    description:"Users Connection"

    field edges() -> Vec<Edge<User>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<User>: Context as "UsersEdge" |&self| {
    description:"Users Edge"
    
    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> User {
        self.node.clone()
    }
});
