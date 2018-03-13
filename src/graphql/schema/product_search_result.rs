//! File containing product object of graphql schema
use juniper;
use juniper::ID as GraphqlID;
use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(SearchResultProduct: Context as "SearchResultProduct" |&self| {
    description: "Search Result Product's info."

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Product, self.base_product.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.base_product.id.to_string().into()
    }

    field base_product() -> BaseProduct as "Base product info." {
        self.base_product.clone()
    }

    field variant() -> Product as "Variant info." {
        self.variant.clone()
    }

    field attrs() -> Vec<AttrValue> as "Attributes of product variant." {
        self.attrs.clone()
    }

});

graphql_object!(Connection<SearchResultProduct>: Context as "SearchResultProductsConnection" |&self| {
    description:"Search Result Product Connection"

    field edges() -> Vec<Edge<SearchResultProduct>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<SearchResultProduct>: Context as "SearchResultProductsEdge" |&self| {
    description:"Search Result Product Edge"

    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> SearchResultProduct {
        self.node.clone()
    }
});
