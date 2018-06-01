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

use graphql::context::Context;
use graphql::models::*;

graphql_object!(Cart: Context as "Cart" |&self| {
    description: "Users cart"

    field stores(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset") 
            -> FieldResult<Option<Connection<CartStore, PageInfo>>> as "Fetches stores using relay connection." {
        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/cart",
            context.config.service_url(Service::Stores),
            Model::Store.to_url());

        let body = serde_json::to_string(&self.inner)?;

        context.request::<Vec<Store>>(Method::Post, url, Some(body))
            .map (|stores| {
                let mut cart_stores: Vec<CartStore> = stores
                    .into_iter()
                    .skip(offset as usize)
                    .take(count as usize)
                    .map(|store| {
                        let products = store.base_products
                            .clone()
                            .unwrap_or_default()
                            .into_iter()
                            .flat_map(|base_product| {
                                base_product.variants.clone()
                                .and_then(|mut v|{
                                    Some(v.iter_mut().map(|variant| {
                                        let (quantity, selected) = self.inner
                                            .iter()
                                            .find(|v|v.product_id == variant.id)
                                            .map(|v| (v.quantity, v.selected))
                                            .unwrap_or_default();

                                        let price = if let Some(discount) = variant.discount.clone() {
                                            variant.price * ( 1.0 - discount )
                                        } else {
                                            variant.price
                                        };

                                        CartProduct {
                                            id: variant.id,
                                            name: base_product.name.clone(),
                                            photo_main: variant.photo_main.clone(),
                                            selected,
                                            price,
                                            quantity
                                        }
                                    }).collect::<Vec<CartProduct>>())
                                }).unwrap_or_default()
                            }).collect();
                        CartStore::new(store, products)
                    })
                    .collect();
                let mut store_edges: Vec<Edge<CartStore>> =  vec![];
                for i in 0..cart_stores.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            cart_stores[i].clone()
                        );
                    store_edges.push(edge);
                }
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

});

graphql_object!(CartProductStore: Context as "CartProductStore" |&self| {
    description: "Cart product store's info."

    field product_id() -> GraphqlID as "Base64 Unique product id"{
        ID::new(Service::Stores, Model::CartProduct, self.product_id).to_string().into()
    }

    field store_id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::CartStore, self.store_id).to_string().into()
    }

});
