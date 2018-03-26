//! File containing product object of graphql schema
use juniper;
use juniper::ID as GraphqlID;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;

use graphql::context::Context;
use graphql::models::*;
use super::*;

graphql_object!(BaseProduct: Context as "BaseProduct" |&self| {
    description: "Base Product's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::BaseProduct, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field name() -> Vec<Translation> as "Full Name" {
        self.name.clone()
    }

    field is_active() -> bool as "If the product was disabled (deleted), isActive is false" {
        self.is_active
    }

    field short_description() -> Vec<Translation> as "Short description" {
        self.short_description.clone()
    }

    field long_description() -> Option<Vec<Translation>> as "Long Description" {
        self.long_description.clone()
    }
    
    field seo_title() -> Option<Vec<Translation>> as "SEO title" {
        self.seo_title.clone()
    }
    
    field seo_description() -> Option<Vec<Translation>> as "SEO Description" {
        self.seo_description.clone()
    }

    field currency_id() -> i32 as "Currency Id" {
        self.currency_id.clone()
    }

    field category_id() -> i32 as "Category id" {
        self.category_id
    }

    field views() -> i32 as "Views" {
        self.views
    }
});

graphql_object!(Connection<BaseProduct>: Context as "BaseProductsConnection" |&self| {
    description:"Base Products Connection"

    field edges() -> Vec<Edge<BaseProduct>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<BaseProduct>: Context as "BaseProductsEdge" |&self| {
    description:"Base Products Edge"

    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> BaseProduct {
        self.node.clone()
    }
});

graphql_object!(BaseProductWithVariants: Context as "BaseProductWithVariants" |&self| {
    description: "Base product with variantsinfo."

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::BaseProduct, self.base_product.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.base_product.id.to_string().into()
    }

    field base_product() -> BaseProduct as "Base product info." {
        self.base_product.clone()
    }

    field variants() -> Vec<VariantsWithAttributes> as "Variants info." {
        self.variants.clone()
    }

});

graphql_object!(VariantsWithAttributes: Context as "VariantsWithAttributes" |&self| {
    description: "Variants with attributes info."

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Product, self.product.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.product.id.to_string().into()
    }

    field product() -> Product as "Base product info." {
        self.product.clone()
    }

    field attributes() -> Vec<AttrValue> as "Variants info." {
        self.attrs.clone()
    }

});


graphql_object!(Connection<BaseProductWithVariants>: Context as "BaseProductWithVariantsConnection" |&self| {
    description:"Base Products Connection"

    field edges() -> Vec<Edge<BaseProductWithVariants>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<BaseProductWithVariants>: Context as "BaseProductWithVariantsEdge" |&self| {
    description:"Base Products Edge"

    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> BaseProductWithVariants {
        self.node.clone()
    }
});