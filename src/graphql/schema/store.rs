//! File containing node object of graphql schema
//! File containing store object of graphql schema
use std::cmp;
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::{Language, Translation};

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Store: Context as "Store" |&self| {
    description: "Store's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Store, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field isActive() -> &bool as "If the store was disabled (deleted), isActive is false" {
        &self.is_active
    }

    field short_description() -> &[Translation] as "Short description" {
        &self.short_description
    }

    field long_description() -> &Option<Vec<Translation>> as "Long description" {
        &self.long_description
    }

    field slug() -> &str as "Slug" {
        &self.slug
    }

    field cover() -> &Option<String> as "Cover" {
        &self.cover
    }

    field logo() -> &Option<String> as "Logo" {
        &self.logo
    }

    field phone() -> &Option<String> as "Phone" {
        &self.phone
    }

    field email() -> &Option<String> as "Email" {
        &self.email
    }

    field address() -> &Option<String> as "Address" {
        &self.address
    }

    field country() -> &Option<String> as "Country" {
        &self.country
    }

    field facebook_url() -> &Option<String> as "Facebook url" {
        &self.facebook_url
    }

    field twitter_url() -> &Option<String> as "Twitter url" {
        &self.twitter_url
    }

    field instagram_url() -> &Option<String> as "Instagram url" {
        &self.instagram_url
    }

    field default_language() -> &Language as "Default language" {
        &self.default_language
    }

    field slogan() -> &Option<String> as "Slogan" {
        &self.slogan
    }

    field rating() -> &f64 as "Rating" {
        &self.rating
    }

    field base_products(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID> as "Offset from begining",
        skip_base_prod_id = None : Option<i32> as "Skip base prod id" ) 
            -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Fetches base products of the store." {
        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        if let Some(ref base_products) = self.base_products {
            let mut base_product_edges: Vec<Edge<BaseProduct>> = base_products.clone()
                .into_iter()
                .skip(offset as usize)
                .take(count as usize)
                .map(|base_product| Edge::new(
                            juniper::ID::from(ID::new(Service::Stores, Model::BaseProduct, base_product.id.clone()).to_string()),
                            base_product.clone()
                        ))
                .collect();
            let has_next_page = base_product_edges.len() as i32 > count;
            let has_previous_page = true;
            let start_cursor =  base_product_edges.iter().nth(0).map(|e| e.cursor.clone());
            let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
            let page_info = PageInfo {
                has_next_page,
                has_previous_page,
                start_cursor,
                end_cursor};
            Ok(Some(Connection::new(base_product_edges, page_info)))
        } else {

            let url = match skip_base_prod_id {
                None => format!(
                        "{}/{}/{}/products?offset={}&count={}",
                        &context.config.service_url(Service::Stores),
                        Model::Store.to_url(),
                        self.id,
                        offset,
                        count + 1
                    ),
                Some(id) => format!(
                        "{}/{}/{}/products?skip_base_product_id={}&offset={}&count={}",
                        &context.config.service_url(Service::Stores),
                        Model::Store.to_url(),
                        self.id,
                        id,
                        offset,
                        count + 1
                    )
            };

            context.request::<Vec<BaseProduct>>(Method::Get, url, None)
                .map (|base_products| {
                    let mut base_product_edges: Vec<Edge<BaseProduct>> =  vec![];
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
                        end_cursor};
                    Connection::new(base_product_edges, page_info)
                })
                .wait()
                .map(|u| Some(u))
        }
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

graphql_object!(Connection<Store, PageInfo>: Context as "StoresConnection" |&self| {
    description:"Stores Connection"

    field edges() -> &[Edge<Store>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<Store>: Context as "StoresEdge" |&self| {
    description:"Stores Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &Store {
        &self.node
    }
});

graphql_object!(Connection<String, PageInfo>: Context as "FullNameConnection" |&self| {
    description:"Name Connection"

    field edges() -> &[Edge<String>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Connection<Store, PageInfoStoresSearch>: Context as "StoresWithTotalCountConnection" |&self| {
    description:"Stores Connection"

    field edges() -> &[Edge<Store>] {
        &self.edges
    }

    field page_info() -> &PageInfoStoresSearch {
        &self.page_info
    }
});

graphql_object!(Edge<String>: Context as "FullNameEdge" |&self| {
    description:"Name Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &str {
        &self.node
    }
});
