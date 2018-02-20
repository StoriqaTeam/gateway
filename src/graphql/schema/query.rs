//! File containing query object of graphql schema
use std::str::FromStr;
use std::cmp;

use juniper::FieldResult;
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use futures::Future;
use juniper::ID as GraphqlID;

use super::*;

pub const QUERY_NODE_ID: i32 = 1;

pub struct Query;

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

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        QUERY_NODE_ID.to_string().into()
    }

    field apiVersion() -> &str as "Current api version." {
        "1.0"
    }

    field static_node_id() -> FieldResult<StaticNodeIds> as "Static node id dictionary." {
        Ok(StaticNodeIds{})
    }

    field me(&executor) -> FieldResult<Option<User>> as "Fetches viewer for users." {
        let context = executor.context();
        let url = format!("{}/{}/current",
            Service::Users.to_url(&context.config),
            Model::User.to_url());
        context.http_client.request_with_auth_header::<User>(Method::Get, url, None, context.user.clone())
                    .or_else(|err| Err(err.into_graphql()))
                    .wait()
                    .map(|u| Some(u))
    }

    field node(&executor, id: GraphqlID as "Id of a node.") -> FieldResult<Node> as "Fetches graphql interface node by id."  {
        let context = executor.context();
        if id.to_string() == QUERY_NODE_ID.to_string() {
             Ok(Node::Query(Query{}))
        } else {
            let identifier = ID::from_str(&*id)?;
            match (&identifier.service, &identifier.model) {
                (&Service::Users, _) => {
                                context.http_client.request_with_auth_header::<User>(Method::Get, identifier.url(&context.config), None, context.user.clone())
                                    .map(|res| Node::User(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                },
                (&Service::Stores, &Model::Store) => {
                                context.http_client.request_with_auth_header::<Store>(Method::Get, identifier.url(&context.config), None, context.user.clone())
                                    .map(|res| Node::Store(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                },
                (&Service::Stores, &Model::Product) => {
                                context.http_client.request_with_auth_header::<Product>(Method::Get, identifier.url(&context.config), None, context.user.clone())
                                    .map(|res| Node::Product(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                },
                (&Service::Stores, _) => {
                                context.http_client.request_with_auth_header::<Store>(Method::Get, identifier.url(&context.config), None, context.user.clone())
                                    .map(|res| Node::Store(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                }
            }
        }
    }


    field stores_find_by_name(&executor, first = None : Option<i32> as "First edges", after = None : Option<String>  as "Store name") -> FieldResult<Connection<Store>> as "Finds stores by name using relay connection." {
        let context = executor.context();

        let name = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/search?name={}&count={}",
            Service::Stores.to_url(&context.config),
            Model::Store.to_url(),
            name,
            first + 1);

        context.http_client.request_with_auth_header::<Vec<Store>>(Method::Get, url, None, context.user.clone())
            .or_else(|err| Err(err.into_graphql()))
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

    field stores_name_auto_complete(&executor, first = None : Option<i32> as "First edges", after = None : Option<String>  as "Part of store name") -> FieldResult<Connection<String>> as "Finds stores full name by part of the name." {
        let context = executor.context();

        let name_part = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/auto_complete?name_part={}&count={}",
            Service::Stores.to_url(&context.config),
            Model::Store.to_url(),
            name_part,
            first + 1);

        context.http_client.request_with_auth_header::<Vec<String>>(Method::Get, url, None, context.user.clone())
            .or_else(|err| Err(err.into_graphql()))
            .map (|full_names| {
                let mut store_edges: Vec<Edge<String>> = full_names
                    .into_iter()
                    .map(|full_name| Edge::new(
                                juniper::ID::from(full_name.clone()),
                                full_name
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

});
