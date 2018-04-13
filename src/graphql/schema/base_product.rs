//! File containing product object of graphql schema
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
    
    field store(&executor) -> FieldResult<Option<Store>> as "Fetches store by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.id.to_string()
        );

        context.request::<Store>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
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
    
    field variants(&executor) -> FieldResult<Option<Variants>> as "Variants" {
       let context = executor.context();
        let url = format!("{}/{}/by_base_product/{}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.id);

        context.request::<Vec<Product>>(Method::Get, url, None)
            .wait()
            .or_else(|_| Ok(vec![]))
            .map(|u| 
            Some(Variants::new(u)))
    }

    field views() -> &i32 as "Views" {
        &self.views
    }
});

graphql_object!(Variants: Context as "BaseProductVariants" |&self| {
    description:"Base Product Variants"

    field all() -> &[Product] {
        &self.products
    }
    
    field most_discount() -> Option<&Product> {
        self.get_most_discount()
    }
    
    field first() -> Option<&Product> {
        self.get_first()
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


graphql_object!(Connection<BaseProduct, PageInfoProductsSearch>: Context as "BaseProductsSearchConnection" |&self| {
    description:"Base Products Connection"

    field edges() -> &[Edge<BaseProduct>] {
        &self.edges
    }

    field page_info() -> &PageInfoProductsSearch {
        &self.page_info
    }
});
