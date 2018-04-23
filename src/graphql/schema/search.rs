//! File containing search object of graphql schema
use std::cmp;
use std::str::FromStr;

use juniper;
use serde_json;
use juniper::FieldResult;
use juniper::ID as GraphqlID;
use hyper::Method;
use futures::future;
use futures::Future;
use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(Search: Context as "Search" |&self| {
    description: "Searching endpoint."

    field find_product(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset form begining", 
        search_term : SearchProductInput as "Search pattern") 
            -> FieldResult<Option<Connection<BaseProduct, PageInfoProductsSearch>>> as "Find products by name using relay connection." {

        let context = executor.context();

        let offset = after
            .and_then(|id| i32::from_str(&id).ok())
            .unwrap_or_default();


        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/search?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1
            );

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body.clone()))
            .map (|products| {
                let mut product_edges: Vec<Edge<BaseProduct>> =  vec![];
                for i in 0..products.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            products[i].clone()
                        );
                    product_edges.push(edge);
                }
                let search_filters = ProductsSearchFilters::new(search_term);
                let has_next_page = product_edges.len() as i32 == count + 1;
                if has_next_page {
                    product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  product_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfoProductsSearch {
                    has_next_page,
                    has_previous_page,
                    search_filters: Some(search_filters),
                    start_cursor,
                    end_cursor};
                Connection::new(product_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

    field auto_complete_product_name(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset form begining", 
        name : String as "Name part") 
            -> FieldResult<Option<Connection<String, PageInfo>>> as "Finds products full name by part of the name." {

        let context = executor.context();

        let offset = after
            .and_then(|id| i32::from_str(&id).ok())
            .unwrap_or_default();


        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/auto_complete?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1,
            );

        context.request::<Vec<String>>(Method::Post, url, Some(name))
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

    field find_store(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset form begining", 
        search_term : SearchStoreInput as "Search store input") 
            -> FieldResult<Option<Connection<Store, PageInfoStoresSearch>>> as "Finds stores by name using relay connection." {

        let context = executor.context();

        let offset = after
            .and_then(|id| i32::from_str(&id).ok())
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let body = serde_json::to_string(&search_term)?;

        println!("{}", body);

        let url = format!("{}/{}/search?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            offset,
            count + 1
            );

        context.request::<Vec<Store>>(Method::Post, url, Some(body))
            .and_then (|stores| {
                let mut store_edges: Vec<Edge<Store>> =  vec![];
                for i in 0..stores.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            stores[i].clone()
                        );
                    store_edges.push(edge);
                }
                let has_next_page = store_edges.len() as i32 == count + 1;
                if has_next_page {
                    store_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  store_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = store_edges.iter().last().map(|e| e.cursor.clone());

                let search_filters = StoresSearchFilters::new(search_term);

                let page_info = PageInfoStoresSearch {
                        has_next_page,
                        has_previous_page,
                        search_filters,
                        start_cursor,
                        end_cursor
                    };

                future::ok(Connection::new(store_edges, page_info))
            })
            .wait()
            .map(|u| Some(u))
    }

    field auto_complete_store_name(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset form begining", 
        name : String as "Name part") 
            -> FieldResult<Option<Connection<String, PageInfo>>> as "Finds stores full name by part of the name." {
        let context = executor.context();

        let offset = after
            .and_then(|id| i32::from_str(&id).ok())
            .unwrap_or_default();


        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/auto_complete?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            offset,
            count + 1
            );

        context.request::<Vec<String>>(Method::Post, url, Some(name))
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
                let start_cursor =  full_name_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = full_name_edges.iter().last().map(|e| e.cursor.clone());

                let has_previous_page = true;
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

graphql_object!(ProductsSearchFilters: Context as "ProductsSearchFilters" |&self| {
    description: "Products Search Filters options endpoint."

    field price_range(&executor) -> FieldResult<Option<RangeFilter>> as "Price filter."{
        let context = executor.context();

        let body = serde_json::to_string(&self.search_term)?;

        let url = format!("{}/{}/search/filters/price",
                        context.config.service_url(Service::Stores),
                        Model::BaseProduct.to_url(),
                    );
        context.request::<RangeFilter>(Method::Post, url, Some(body))
            .wait()
            .map(|u| Some(u))
    }

    field categories(&executor) -> FieldResult<Option<Category>> as "Category."{
        let context = executor.context();

        let body = serde_json::to_string(&self.search_term)?;

        let url = format!("{}/{}/search/filters/category",
                        context.config.service_url(Service::Stores),
                        Model::BaseProduct.to_url(),
                    );
        context.request::<Category>(Method::Post, url, Some(body))
            .wait()
            .map(|u| Some(u))
    }

    field attr_filters(&executor) -> FieldResult<Option<Vec<AttributeFilter>>> as "Attribute filters."{
       let context = executor.context();

        let body = serde_json::to_string(&self.search_term)?;

        let url = format!("{}/{}/search/filters/attributes",
                        context.config.service_url(Service::Stores),
                        Model::BaseProduct.to_url(),
                    );
        context.request::<Option<Vec<AttributeFilter>>>(Method::Post, url, Some(body))
            .wait()
    }

    field attr_filters_category(&executor) -> FieldResult<Option<Vec<AttributeFilter>>> as "Attribute filters for whole category."{
        let context = executor.context();

        let mut options = ProductsSearchOptionsInput::default();
        options.category_id = self.search_term.options
            .clone()
            .map(|o| o.category_id)
            .and_then(|x| x);
        let mut search_term_only_category = SearchProductInput::default();
        search_term_only_category.options = Some(options);

        let body = serde_json::to_string(&search_term_only_category)?;
        
        let url = format!("{}/{}/search/filters/attributes",
                        context.config.service_url(Service::Stores),
                        Model::BaseProduct.to_url(),
                    );
        context.request::<Option<Vec<AttributeFilter>>>(Method::Post, url, Some(body))
            .wait()
    }

});

graphql_object!(StoresSearchFilters: Context as "StoresSearchFilters" |&self| {
    description: "Stores Search Filters options endpoint."

    field total_count(&executor) -> FieldResult<i32> as "Total count."{
        let context = executor.context();

        let body = serde_json::to_string(&self.search_term)?;

        let url = format!("{}/{}/search/filters/count",
                    context.config.service_url(Service::Stores),
                    Model::Store.to_url(),
                    );

        context.request::<i32>(Method::Post, url, Some(body))
            .wait()
    }

    field category(&executor) -> FieldResult<Category> as "Category."{
        let context = executor.context();

        let body = serde_json::to_string(&self.search_term)?;

        let url = format!("{}/{}/search/filters/category",
                    context.config.service_url(Service::Stores),
                    Model::Store.to_url(),
                    );

        context.request::<Category>(Method::Post, url, Some(body))
            .wait()
    }

    field country(&executor) -> FieldResult<Vec<String>> as "Countries"{
        let context = executor.context();

        let body = serde_json::to_string(&self.search_term)?;

        let url = format!("{}/{}/search/filters/country",
                    context.config.service_url(Service::Stores),
                    Model::Store.to_url(),
                    );

        context.request::<Vec<String>>(Method::Post, url, Some(body))
            .wait()
    }

});
