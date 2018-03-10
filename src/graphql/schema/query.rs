//! File containing query object of graphql schema
use std::str::FromStr;
use std::cmp;

use serde_json;
use juniper::ID as GraphqlID;
use juniper::FieldResult;
use hyper::Method;
use futures::Future;
use stq_static_resources::currency::{Currency, CurrencyGraphQl};
use stq_static_resources::{Language, LanguageGraphQl};
use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;
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
            context.config.service_url(Service::Users),
            Model::User.to_url());
        context.http_client.request_with_auth_header::<User>(Method::Get, url, None, context.user.as_ref().map(|t| t.to_string()))
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
                                context.http_client.request_with_auth_header::<User>(Method::Get, identifier.url(&context.config), None, context.user.as_ref().map(|t| t.to_string()))
                                    .map(|res| Node::User(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                },
                (&Service::Stores, &Model::Store) => {
                                context.http_client.request_with_auth_header::<Store>(Method::Get, identifier.url(&context.config), None, context.user.as_ref().map(|t| t.to_string()))
                                    .map(|res| Node::Store(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                },
                (&Service::Stores, &Model::Product) => {
                                context.http_client.request_with_auth_header::<Product>(Method::Get, identifier.url(&context.config), None, context.user.as_ref().map(|t| t.to_string()))
                                    .map(|res| Node::Product(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                },
                (&Service::Stores, _) => {
                                context.http_client.request_with_auth_header::<Store>(Method::Get, identifier.url(&context.config), None, context.user.as_ref().map(|t| t.to_string()))
                                    .map(|res| Node::Store(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                }
            }
        }
    }


    field stores_find_by_name(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", search_term : SearchStoreInput as "Search store input") -> FieldResult<Connection<Store>> as "Finds stores by name using relay connection." {
        let context = executor.context();

        let offset = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let body = serde_json::to_string(&search_term)?;

        let url = format!("{}/{}/search?count={}&offset={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            count + 1,
            offset
            );

        context.http_client.request_with_auth_header::<Vec<Store>>(Method::Get, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .map (|stores| {
                let mut store_edges: Vec<Edge<Store>> =  vec![];
                for i in 0..stores.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            stores[i].clone()
                        );
                    store_edges.push(edge);
                }
                let has_next_page = store_edges.len() as i32 == count + 1;
                if has_next_page {
                    store_edges.pop();
                };
                let has_previous_page = true;
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(store_edges, page_info)
            })
            .wait()
    }

    field stores_name_auto_complete(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", search_term : SearchStoreInput as "Search store input") -> FieldResult<Connection<String>> as "Finds stores full name by part of the name." {
        let context = executor.context();

        let offset = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let body = serde_json::to_string(&search_term)?;

        let url = format!("{}/{}/auto_complete?count={}&offset={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            count + 1,
            offset
            );

        context.http_client.request_with_auth_header::<Vec<String>>(Method::Get, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .map (|full_names| {
                let mut full_name_edges: Vec<Edge<String>> =  vec![];
                for i in 0..full_names.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            full_names[i].clone()
                        );
                    full_name_edges.push(edge);
                }
                let has_next_page = full_name_edges.len() as i32 == count + 1;
                if has_next_page {
                    full_name_edges.pop();
                };
                let has_previous_page = true;
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(full_name_edges, page_info)
            })
            .wait()
    }

    field products_find(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", search_term : SearchProductInput as "Search pattern") -> FieldResult<Connection<Product>> as "Finds stores by name using relay connection." {
        let context = executor.context();

        let offset = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/search?count={}&offset={}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            count + 1,
            offset
            );

        let search_product = SearchProduct::from_input(search_term)?;
        let body = serde_json::to_string(&search_product)?;

        context.http_client.request_with_auth_header::<Vec<Product>>(Method::Get, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .map (|stores| {
                let mut store_edges: Vec<Edge<Product>> =  vec![];
                for i in 0..stores.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            stores[i].clone()
                        );
                    store_edges.push(edge);
                }
                let has_next_page = store_edges.len() as i32 == count + 1;
                if has_next_page {
                    store_edges.pop();
                };
                let has_previous_page = true;
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(store_edges, page_info)
            })
            .wait()
    }

    field products_name_auto_complete(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", search_term : SearchProductInput as "Search product input") -> FieldResult<Connection<String>> as "Finds products full name by part of the name." {
        let context = executor.context();

        let offset = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let body = serde_json::to_string(&search_term)?;

        let url = format!("{}/{}/auto_complete?count={}&offset={}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            count + 1,
            offset
            );

        context.http_client.request_with_auth_header::<Vec<String>>(Method::Get, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .map (|full_names| {
                let mut full_name_edges: Vec<Edge<String>> =  vec![];
                for i in 0..full_names.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            full_names[i].clone()
                        );
                    full_name_edges.push(edge);
                }
                let has_next_page = full_name_edges.len() as i32 == count + 1;
                if has_next_page {
                    full_name_edges.pop();
                };
                let has_previous_page = true;
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(full_name_edges, page_info)
            })
            .wait()
    }

    field languages(&executor) -> FieldResult<Vec<LanguageGraphQl>> as "Fetches languages." {
        Ok(Language::as_vec())
    }


    field currencies(&executor) -> FieldResult<Vec<CurrencyGraphQl>> as "Fetches currencies." {
        Ok(Currency::as_vec())
    }

    field categories_tree(&executor) -> FieldResult<Vec<CategoryTree>> as "Fetches categories tree." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url());

        context.http_client.request_with_auth_header::<Vec<CategoryTree>>(Method::Get, url, None, context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

});
