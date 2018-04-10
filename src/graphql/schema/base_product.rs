//! File containing product object of graphql schema
use std::cmp;

use juniper;
use juniper::ID as GraphqlID;
use juniper::FieldResult;
use hyper::Method;
use futures::Future;
use base64::encode;

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

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field is_active() -> bool as "If the product was disabled (deleted), isActive is false" {
        self.is_active
    }

    field short_description() -> &[Translation] as "Short description" {
        &self.short_description
    }

    field long_description() -> &Option<Vec<Translation>> as "Long Description" {
        &self.long_description
    }
    
    field seo_title() -> &Option<Vec<Translation>> as "SEO title" {
        &self.seo_title
    }
    
    field seo_description() -> &Option<Vec<Translation>> as "SEO Description" {
        &self.seo_description
    }

    field currency_id() -> &i32 as "Currency Id" {
        &self.currency_id
    }
    
    field store_id() -> &i32 as "Store Id" {
        &self.store_id
    }

    field category(&executor) -> FieldResult<Option<Category>> as "Category" {
       let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url(),
            self.category_id);

        context.request::<Category>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }

    field views() -> &i32 as "Views" {
        &self.views
    }
});

graphql_object!(Connection<BaseProduct, PageInfo>: Context as "BaseProductsConnection" |&self| {
    description:"Base Products Connection"

    field edges() -> &[Edge<BaseProduct>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<BaseProduct>: Context as "BaseProductsEdge" |&self| {
    description:"Base Products Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &BaseProduct {
        &self.node
    }
});

graphql_object!(BaseProductWithVariants: Context as "BaseProductWithVariants" |&self| {
    description: "Base product with variantsinfo."

    field id() -> GraphqlID as "Base64 Unique id"{
        encode(&format!("{}|{}|{}", Service::Stores, "baseproductwithvariants",  self.base_product.id)).into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.base_product.id
    }

    field base_product() -> &BaseProduct as "Base product info." {
        &self.base_product
    }

    field variants() -> &[VariantsWithAttributes] as "Variants info." {
        &self.variants
    }

    field base_products_same_store(&executor, 
        first = None : Option<i32> as "First edges", 
        after = None : Option<i32>  as "Offset from begining") 
            -> FieldResult<Option<Connection<BaseProductWithVariants, PageInfo>>> as "Fetches base product with same store id." {
        let context = executor.context();
        
        let offset = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!(
            "{}/{}/{}/products?skip_base_product_id={}&offset={}&count={}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.base_product.store_id,
            self.base_product.id,
            offset,
            count + 1
        );

        context.request::<Vec<BaseProductWithVariants>>(Method::Get, url, None)
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
                let start_cursor =  base_product_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page, 
                    has_previous_page, 
                    start_cursor,
                    end_cursor
                    };
                Connection::new(base_product_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }
});

graphql_object!(VariantsWithAttributes: Context as "VariantsWithAttributes" |&self| {
    description: "Variants with attributes info."

    field id() -> GraphqlID as "Base64 Unique id"{
        encode(&format!("{}|{}|{}", Service::Stores, "variantswithattributes",  self.product.id)).into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.product.id
    }

    field product() -> &Product as "Base product info." {
        &self.product
    }

    field attributes() -> &[AttrValue] as "Variants info." {
        &self.attrs
    }

});

graphql_object!(Connection<BaseProductWithVariants, PageInfo>: Context as "BaseProductWithVariantsConnection" |&self| {
    description:"Base Products Connection"

    field edges() -> &[Edge<BaseProductWithVariants>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Connection<BaseProductWithVariants, PageInfoWithSearchFilters<SearchFiltersWithoutCategory>>: Context as "BaseProductWithVariantsSearchFilterWithoutCategoryConnection" |&self| {
    description:"Base Products Connection"

    field edges() -> &[Edge<BaseProductWithVariants>] {
        &self.edges
    }

    field page_info() -> &PageInfoWithSearchFilters<SearchFiltersWithoutCategory> {
        &self.page_info
    }
});

graphql_object!(Connection<BaseProductWithVariants, PageInfoWithSearchFilters<SearchFiltersInCategory>>: Context as "BaseProductWithVariantsSearchFilterInCategoryConnection" |&self| {
    description:"Base Products Connection"

    field edges() -> &[Edge<BaseProductWithVariants>] {
        &self.edges
    }

    field page_info() -> &PageInfoWithSearchFilters<SearchFiltersInCategory> {
        &self.page_info
    }
});

graphql_object!(Edge<BaseProductWithVariants>: Context as "BaseProductWithVariantsEdge" |&self| {
    description:"Base Products Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &BaseProductWithVariants {
        &self.node
    }
});
