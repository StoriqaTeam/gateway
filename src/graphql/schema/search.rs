//! File containing search object of graphql schema
use std::cmp;

use juniper;
use serde_json;
use juniper::FieldResult;
use hyper::Method;
use futures::Future;
use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(Search: Context as "Search" |&self| {
    description: "Searching endpoint."

    field find_product(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", search_term : SearchProductsByNameInput as "Search pattern") -> FieldResult<Connection<BaseProductWithVariants>> as "Find products by name using relay connection." {
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

        let body = serde_json::to_string(&search_term)?;

        context.http_client.request_with_auth_header::<Vec<BaseProductWithVariants>>(Method::Get, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .map (|stores| {
                let mut store_edges: Vec<Edge<BaseProductWithVariants>> =  vec![];
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

    field auto_complete_product_name(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", name : String as "Name part") -> FieldResult<Connection<String>> as "Finds products full name by part of the name." {
        let context = executor.context();

        let offset = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/auto_complete?count={}&offset={}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            count + 1,
            offset
            );

        context.http_client.request_with_auth_header::<Vec<String>>(Method::Get, url, Some(name), context.user.as_ref().map(|t| t.to_string()))
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

    field find_store(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", search_term : SearchStoreInput as "Search store input") -> FieldResult<Connection<Store>> as "Finds stores by name using relay connection." {
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

    field auto_complete_store_name(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", name : String as "Name part") -> FieldResult<Connection<String>> as "Finds stores full name by part of the name." {
        let context = executor.context();

        let offset = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/auto_complete?count={}&offset={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            count + 1,
            offset
            );

        context.http_client.request_with_auth_header::<Vec<String>>(Method::Get, url, Some(name), context.user.as_ref().map(|t| t.to_string()))
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

});
