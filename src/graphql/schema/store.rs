//! File containing node object of graphql schema
//! File containing store object of graphql schema
use std::cmp;

use juniper;
use juniper::ID as GraphqlID;
use juniper::FieldResult;
use hyper::Method;
use futures::Future;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::{Language, Translation};

use graphql::context::Context;
use graphql::models::*;
use super::*;

graphql_object!(Store: Context as "Store" |&self| {
    description: "Store's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Store, self.id).to_string().into()
    }

    field raw_id() -> i32 as "Unique int id"{
        self.id
    }

    field name() -> Vec<Translation> as "Full Name" {
        self.name.clone()
    }

    field isActive() -> bool as "If the store was disabled (deleted), isActive is false" {
        self.is_active
    }

    field short_description() -> Vec<Translation> as "Short description" {
        self.short_description.clone()
    }

    field long_description() -> Option<Vec<Translation>> as "Long description" {
        self.long_description.clone()
    }

    field slug() -> String as "Slug" {
        self.slug.clone()
    }

    field cover() -> Option<String> as "Cover" {
        self.cover.clone()
    }

    field logo() -> Option<String> as "Logo" {
        self.logo.clone()
    }

    field phone() -> Option<String> as "Phone" {
        self.phone.clone()
    }

    field email() -> Option<String> as "Email" {
        self.email.clone()
    }

    field address() -> Option<String> as "Address" {
        self.address.clone()
    }

    field facebook_url() -> Option<String> as "Facebook url" {
        self.facebook_url.clone()
    }

    field twitter_url() -> Option<String> as "Twitter url" {
        self.twitter_url.clone()
    }

    field instagram_url() -> Option<String> as "Instagram url" {
        self.instagram_url.clone()
    }

    field default_language() -> Language as "Default language" {
        self.default_language.clone()
    }

    field slogan() -> Option<String> as "Slogan" {
        self.slogan.clone()
    }

    field base_products_with_variants(&executor, first = None : Option<i32> as "First edges", after = None : Option<i32>  as "Offset from begining") -> FieldResult<Connection<BaseProductWithVariants>> as "Fetches base products of the store." {
        let context = executor.context();
        
        let offset = after.unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!(
            "{}/{}/{}/products?offset={}&count={}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.id,
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
                let page_info = PageInfo {has_next_page: has_next_page, has_previous_page: has_previous_page};
                Connection::new(base_product_edges, page_info)
            })
            .wait()
    }

    field products_count(&executor) -> FieldResult<i32> as "Fetches products count of the store." {
        let context = executor.context();
        
        let url = format!(
            "{}/{}/{}/products/count",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.id,
        );

        context.request::<i32>(Method::Get, url, None)
            .wait()
    }


});

graphql_object!(Connection<Store>: Context as "StoresConnection" |&self| {
    description:"Stores Connection"

    field edges() -> Vec<Edge<Store>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<Store>: Context as "StoresEdge" |&self| {
    description:"Stores Edge"

    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> Store {
        self.node.clone()
    }
});

graphql_object!(Connection<String>: Context as "FullNameConnection" |&self| {
    description:"Name Connection"

    field edges() -> Vec<Edge<String>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<String>: Context as "FullNameEdge" |&self| {
    description:"Name Edge"

    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> String {
        self.node.clone()
    }
});
