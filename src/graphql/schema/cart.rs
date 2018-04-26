//! File containing PageInfo object of graphql schema

use std::cmp;
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper;
use juniper::FieldResult;
use juniper::ID as GraphqlID;
use serde_json;
use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Cart: Context as "Cart" |&self| {
    description: "Users cart"

    field stores(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Id of a store") 
            -> FieldResult<Option<Connection<Store, PageInfo>>> as "Fetches stores using relay connection." {
        let context = executor.context();

        let offset = after
            .and_then(|id| i32::from_str(&id).ok())
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/cart",
            context.config.service_url(Service::Stores),
            Model::Store.to_url());

        let body = serde_json::to_string(&self)?;

        context.request::<Vec<Store>>(Method::Post, url, Some(body))

            .map (|stores| {
                let mut store_edges: Vec<Edge<Store>> = stores
                    .into_iter()
                    .skip(offset as usize)
                    .take(count as usize)
                    .map(|store| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::Store, store.id.clone()).to_string()),
                                store.clone()
                            ))
                    .collect();
                let has_next_page = store_edges.len() as i32 > count;
                let has_previous_page = true;
                let start_cursor =  store_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = store_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(store_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

    field cart_products(&executor,
        first = None : Option<i32> as "First edges",  
        after = None : Option<GraphqlID>  as "Base64 Id of product") 
            -> Connection<CartProduct, PageInfo> as "Fetches cart products using relay connection." { 
        let context = executor.context();

        let offset = after
            .and_then(|id| i32::from_str(&id).ok())
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let mut carts_edges: Vec<Edge<CartProduct>> = self.inner
            .clone()
            .into_iter()
            .skip(offset as usize)
            .take(count as usize)
            .map(|cart_product| Edge::new(
                        juniper::ID::from(ID::new(Service::Orders, Model::CartProduct, cart_product.product_id.clone()).to_string()),
                        cart_product.clone()
                    ))
            .collect();
        let has_next_page = carts_edges.len() as i32 > count;
        let has_previous_page = true;
        let start_cursor =  carts_edges.iter().nth(0).map(|e| e.cursor.clone());
        let end_cursor = carts_edges.iter().last().map(|e| e.cursor.clone());
        let page_info = PageInfo {
            has_next_page,
            has_previous_page,
            start_cursor,
            end_cursor};
        Connection::new(carts_edges, page_info)
    }

});

graphql_object!(CartProduct: Context as "CartProduct" |&self| {
    description: "Cart Product info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Orders, Model::CartProduct, self.product_id).to_string().into()
    }

    field quantity() -> &i32 as "Quantity" {
        &self.quantity
    }

    field product(&executor) -> FieldResult<Option<Product>> as "Fetches product from cart." {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.product_id);

        context.request::<Product>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }
});
