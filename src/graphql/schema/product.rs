//! File containing product object of graphql schema
use std::cmp;

use juniper;
use juniper::ID as GraphqlID;
use serde_json;
use juniper::FieldResult;
use hyper::Method;
use futures::Future;
use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;
use super::*;

graphql_object!(Product: Context as "Product" |&self| {
    description: "Product's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Product, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field is_active() -> bool as "If the product was disabled (deleted), isActive is false" {
        self.is_active
    }

    field discount() -> Option<f64> as "Discount" {
        self.discount.clone()
    }

    field photo_main() -> Option<String> as "Photo main" {
        self.photo_main.clone()
    }

    field vendor_code() -> Option<String> as "Vendor code" {
        self.vendor_code.clone()
    }

    field cashback() -> Option<f64> as "Cashback" {
        self.cashback.clone()
    }

    field find(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", search_term : SearchProductInput as "Search pattern") -> FieldResult<Connection<BaseProduct>> as "Find products by name using relay connection." {
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

        context.http_client.request_with_auth_header::<Vec<BaseProduct>>(Method::Get, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .map (|stores| {
                let mut store_edges: Vec<Edge<BaseProduct>> =  vec![];
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

    field name_auto_complete(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset form begining", name : String as "Name part") -> FieldResult<Connection<String>> as "Finds products full name by part of the name." {
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

});

graphql_object!(Connection<Product>: Context as "ProductsConnection" |&self| {
    description:"Products Connection"

    field edges() -> Vec<Edge<Product>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<Product>: Context as "ProductsEdge" |&self| {
    description:"Products Edge"

    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> Product {
        self.node.clone()
    }
});
