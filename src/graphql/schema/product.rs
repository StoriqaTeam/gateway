//! File containing product object of graphql schema
use juniper;
use juniper::ID as GraphqlID;
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

    field additional_photos() -> Option<Vec<String>> as "Additional photos of the product." {
        self.additional_photos.clone()
    }

    field vendor_code() -> Option<String> as "Vendor code" {
        self.vendor_code.clone()
    }

    field cashback() -> Option<f64> as "Cashback" {
        self.cashback.clone()
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
