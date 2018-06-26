//! File containing warehouse object of graphql schema
use std::cmp;
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;
use serde_json;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Warehouse: Context as "Warehouse" |&self| {
    description: "Warehouse info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Warehouses, Model::Warehouse, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field name() -> &Option<String> as "Name"{
        &self.name
    }

    field location() -> &Option<GeoPoint> as "Location"{
        &self.location
    }

    field kind() -> &WarehouseKind as "Warehouse Kind"{
        &self.kind
    }

    field address_full() -> Address as "Address full"{
        self.clone().into()
    }

    field store_id() -> &i32 as "Store_id"{
        &self.store_id
    }

    field store(&executor) -> FieldResult<Option<Store>> as "Fetches store." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.store_id.to_string()
        );

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field products(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset form begining", 
        search_term : Option<SearchProductInput> as "Search pattern") 
            -> FieldResult<Option<Connection<WarehouseProduct, PageInfo>>> as "Find products of the warehouse using relay connection." {

        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/search?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1
            );

        let search_term = if let Some(search_term) = search_term {
            let options = if let Some(mut options) = search_term.options {
                options.store_id = Some(self.store_id);
                options
            } else {
                ProductsSearchOptionsInput{
                    store_id : Some(self.store_id),
                    ..Default::default()
                }
            };
            SearchProductInput {
                name: search_term.name,
                options: Some(options)
            }
        } else {
            SearchProductInput {
                name: "".to_string(),
                options: Some(ProductsSearchOptionsInput{
                    store_id : Some(self.store_id),
                    ..Default::default()
                })
            }
        };

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .map(|products| products.into_iter().map(|p| p.id).collect())
            .wait()
            .and_then (|products: Vec<i32>| {
                let url = format!("{}/{}/{}/products",
                    context.config.service_url(Service::Warehouses),
                    Model::Warehouse.to_url(),
                    self.id.to_string(),
                    );

                let body = serde_json::to_string(&products)?;

                context.request::<Vec<WarehouseProduct>>(Method::Post, url, Some(body))
                    .map (|products| {
                        let mut product_edges: Vec<Edge<WarehouseProduct>> =  vec![];
                        for i in 0..products.len() {
                            let edge = Edge::new(
                                    juniper::ID::from( (i as i32 + offset).to_string()),
                                    products[i].clone()
                                );
                            product_edges.push(edge);
                        }
                        let has_next_page = product_edges.len() as i32 == count + 1;
                        if has_next_page {
                            product_edges.pop();
                        };
                        let has_previous_page = true;
                        let start_cursor =  product_edges.iter().nth(0).map(|e| e.cursor.clone());
                        let end_cursor = product_edges.iter().last().map(|e| e.cursor.clone());
                        let page_info = PageInfo {
                            has_next_page,
                            has_previous_page,
                            start_cursor,
                            end_cursor};
                        Connection::new(product_edges, page_info)
                    })
                    .wait()
            })
            .map(|u| Some(u))
    }

    field auto_complete_product_name(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset form begining", 
        name : String as "Name part") 
            -> FieldResult<Option<Connection<String, PageInfo>>> as "Finds products full name by part of the name." {

        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/auto_complete?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1,
            );

        let search_term = AutoCompleteProductNameInput {
            name,
            store_id : Some(self.store_id),
        };

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<String>>(Method::Post, url, Some(body))
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
                let start_cursor =  full_name_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = full_name_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(full_name_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

});

graphql_object!(Connection<WarehouseProduct, PageInfo>: Context as "WarehouseProductsConnection" |&self| {
    description:"Warehouse Products Connection"

    field edges() -> &[Edge<WarehouseProduct>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<WarehouseProduct>: Context as "WarehouseProductsEdge" |&self| {
    description:"Warehouse Product Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &WarehouseProduct {
        &self.node
    }
});
