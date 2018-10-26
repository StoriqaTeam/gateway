//! File containing Cart object of graphql schema

use std::cmp;
use std::str::FromStr;

use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::UserId;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Cart: Context as "Cart" |&self| {
    description: "Users cart"

    interfaces: [&Node]

    field id(&executor) -> GraphqlID as "Base64 Unique id"{
        let context = executor.context();

        if let Some(ref user) = context.user {
            ID::new(Service::Orders, Model::Cart, user.user_id.0).to_string().into()
        } else if let Some(session_id) = context.session_id {
            session_id.0.to_string().into()
        }  else {
            ID::new(Service::Orders, Model::Cart, UserId::default().0).to_string().into()
        }

    }

    field stores(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset")
            -> Connection<CartStore, PageInfo> as "Fetches stores using relay connection." {
        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let mut cart_stores: Vec<CartStore> = self.inner.clone()
            .into_iter()
            .skip(offset as usize)
            .take(count as usize)
            .collect();
        let mut store_edges = Edge::create_vec(cart_stores, offset);
        let has_next_page = store_edges.len() as i32 > count;
        let has_previous_page = true;
        let start_cursor =  store_edges.get(0).map(|e| e.cursor.clone());
        let end_cursor = store_edges.iter().last().map(|e| e.cursor.clone());
        let page_info = PageInfo {
            has_next_page,
            has_previous_page,
            start_cursor,
            end_cursor};
        Connection::new(store_edges, page_info)
    }

    field products_cost() -> f64 as "Products cost" {
        self.inner.iter().fold(0.0, |acc, store| {
            let store_products_cost = store.products.iter().fold(0.0, |acc, product| {
                if product.selected {
                    acc + product.price.0 * f64::from(product.quantity.0)
                } else {
                    acc
                }
            });
            acc + store_products_cost
        })
    }

    field delivery_cost() -> f64 as "Delivery cost" {
        0.0
    }

    field total_cost() -> f64 as "Total cost" {
        self.inner.iter().fold(0.0, |acc, store| {
            let store_products_cost = store.products.iter().fold(0.0, |acc, product| {
                if product.selected {
                    acc + product.price.0 * f64::from(product.quantity.0)
                } else {
                    acc
                }
            });
            acc + store_products_cost
        })
    }

    field total_count() -> i32 as "Total products count" {
        self.inner.iter().fold(0, |acc, store| {
            let store_products_cost = store.products.iter().fold(0, |acc, product| {
                if product.selected {
                    acc + product.quantity.0
                } else {
                    acc
                }
            });
            acc + store_products_cost
        })
    }

});

graphql_object!(CartProductStore: Context as "CartProductStore" |&self| {
    description: "Cart product store's info."

    field product_id() -> GraphqlID as "Base64 Unique product id"{
        ID::new(Service::Stores, Model::CartProduct, self.product_id.0).to_string().into()
    }

    field store_id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::CartStore, self.store_id.0).to_string().into()
    }

});
