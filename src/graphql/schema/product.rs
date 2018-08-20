//! File containing product object of graphql schema

use futures::Future;
use hyper::Method;
use juniper;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_api::warehouses::{Stock, Warehouse, WarehouseClient};
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::{Quantity, StockId};

use errors::into_graphql;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Product: Context as "Product" |&self| {
    description: "Product's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Product, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field is_active() -> &bool as "If the product was disabled (deleted), isActive is false" {
        &self.is_active
    }

    field discount() -> &Option<f64> as "Discount" {
        &self.discount
    }

    field currency_id() -> Option<i32> as "Currency id" {
        self.currency_id.map(|c| c.0)
    }

    field photo_main() -> &Option<String> as "Photo main" {
        &self.photo_main
    }

    field additional_photos() -> &Option<Vec<String>> as "Additional photos of the product." {
        &self.additional_photos
    }

    field vendor_code() -> &String as "Vendor code" {
        &self.vendor_code
    }

    field cashback() -> &Option<f64> as "Cashback" {
        &self.cashback
    }

    field price() -> &f64 as "Price" {
        &self.price.0
    }

    field base_product(&executor) -> FieldResult<Option<BaseProduct>> as "Fetches base product by product." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            self.base_product_id.to_string()
        );

        context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()
    }

    field attributes(&executor) -> FieldResult<Option<Vec<AttrValue>>> as "Variants" {
       let context = executor.context();
        let url = format!("{}/{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.id);

        context.request::<Vec<AttrValue>>(Method::Get, url, None)
            .wait()
            .or_else(|_| Ok(vec![]))
            .map(Some)
    }

    field quantity(&executor) -> FieldResult<Option<i32>> as "Fetches product quantity from warehouses." {
        let context = executor.context();

        let rpc_client = context.get_rest_api_client(Service::Warehouses);
        rpc_client.find_by_product_id(self.id)
            .wait()
            .map_err(into_graphql)
            .map(|products| {
                products.iter().fold(0, |acc, p| {
                    acc + p.quantity.0
                })
            })
            .map(Some)
    }

    field stocks(&executor) -> FieldResult<Vec<GraphQLStock>> as "Find product on warehouses." {

        let context = executor.context();

       let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            self.base_product_id.to_string()
        );

        let store_id = context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()?
            .ok_or_else(|| FieldError::new(
                        "Base product not found",
                        graphql_value!({ "code": 400, "details": { "base product for this product not found" }}),
                    ))
            .map(|base_product| base_product.store_id)?;

        let rpc_client = context.get_rest_api_client(Service::Warehouses);
        rpc_client.get_warehouses_for_store(store_id)
            .wait()
            .map_err(into_graphql)
            .and_then (|warehouses: Vec<Warehouse>| {
                warehouses.into_iter().map(|warehouse| {
                    let rpc_client = context.get_rest_api_client(Service::Warehouses);
                    rpc_client.get_product_in_warehouse(warehouse.id, self.id)
                        .wait()
                        .map_err(into_graphql)
                        .map (|stock| {
                            if let Some(stock) = stock {
                                stock
                            } else {
                                Stock {
                                    id: StockId::new(),
                                    product_id: self.id,
                                    warehouse_id: warehouse.id,
                                    quantity: Quantity::default(),
                                }
                            }
                        })
                        .map(GraphQLStock)
                }).collect::<FieldResult<Vec<GraphQLStock>>>()
            })
    }

});

graphql_object!(Connection<Product, PageInfo>: Context as "ProductsConnection" |&self| {
    description:"Products Connection"

    field edges() -> &[Edge<Product>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<Product>: Context as "ProductsEdge" |&self| {
    description:"Products Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &Product {
        &self.node
    }
});
