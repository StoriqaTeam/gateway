//! File containing search object of graphql schema
use std::cmp;
use std::str::FromStr;

use juniper;
use juniper::FieldResult;
use juniper::ID as GraphqlID;
use serde_json;
use hyper::Method;
use futures::Future;
use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(MainPage: Context as "MainPage" |&self| {
    description: "Main Page endpoint."

    field find_most_viewed_products(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset from begining", 
        search_term : MostViewedProductsInput as "Most viewed search pattern") 
            -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Find most viewed base products each one contains one variant." {
        let context = executor.context();

        let offset = after
            .and_then(|id| i32::from_str(&id).ok())
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/most_viewed?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1
            );

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .map (|base_products| {
                let mut base_product_edges: Vec<Edge<BaseProduct>> =  vec![];
                for i in 0..base_products.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            base_products[i].clone()
                        );
                    base_product_edges.push(edge);
                }
                let has_next_page = base_product_edges.len() as i32 == count + 1;
                if has_next_page {
                    base_product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  base_product_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(base_product_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }


    field find_most_discount_products(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset from begining", 
        search_term : MostDiscountProductsInput as "Most discount search pattern") 
            -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Find base products each one with most discount variant." {
        let context = executor.context();

        let offset = after
            .and_then(|id| i32::from_str(&id).ok())
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/most_discount?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1
            );

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .map (|base_products| {
                let mut base_product_edges: Vec<Edge<BaseProduct>> =  vec![];
                for i in 0..base_products.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            base_products[i].clone()
                        );
                    base_product_edges.push(edge);
                }
                let has_next_page = base_product_edges.len() as i32 == count + 1;
                if has_next_page {
                    base_product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  base_product_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(base_product_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

});
