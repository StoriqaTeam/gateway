//! File containing product object of graphql schema
use std::cmp;

use juniper;
use juniper::ID as GraphqlID;
use juniper::FieldResult;
use hyper::Method;
use futures::Future;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;

use graphql::context::Context;
use graphql::models::*;
use super::*;

graphql_object!(BaseProduct: Context as "BaseProduct" |&self| {
    description: "Base Product's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::BaseProduct, self.id).to_string().into()
    }

    field raw_id() -> i32 as "Unique int id"{
        self.id
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
        self.currency_id
    }
    
    field store_id() -> i32 as "Store Id" {
        self.store_id
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

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::BaseProduct, self.base_product.id).to_string().into()
    }

    field raw_id() -> i32 as "Unique int id"{
        self.base_product.id
    }

    field base_product() -> BaseProduct as "Base product info." {
        self.base_product.clone()
    }

    field variants() -> Vec<VariantsWithAttributes> as "Variants info." {
        self.variants.clone()
    }

    field base_products_same_store(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset from begining") -> FieldResult<Connection<BaseProductWithVariants>> as "Fetches base product with same store id." {
        let context = executor.context();
        
        let offset = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!(
            "{}/{}/with_variants?store_id={}&skip_base_product_id={}&count={}&offset={}",
            &context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            self.base_product.store_id,
            self.base_product.id,
            count + 1,
            offset
        );

        context.http_client.request_with_auth_header::<Vec<BaseProductWithVariants>>(Method::Get, url, None, context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .map (|base_products| {
                let mut base_product_edges: Vec<Edge<BaseProductWithVariants>> =  vec![];
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
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(base_product_edges, page_info)
            })
            .wait()
    }
});

graphql_object!(VariantsWithAttributes: Context as "VariantsWithAttributes" |&self| {
    description: "Variants with attributes info."

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Product, self.product.id).to_string().into()
    }

    field raw_id() -> i32 as "Unique int id"{
        self.product.id
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
