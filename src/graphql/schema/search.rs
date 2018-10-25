//! File containing search object of graphql schema
use std::cmp;
use std::str::FromStr;

use futures::future;
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;
use serde_json;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::ModerationStatus;
use stq_types::CategoryId;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(Search: Context as "Search" |&self| {
    description: "Searching endpoint."

    field find_product(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset form beginning",
        search_term : SearchProductInput as "Search pattern",
        visibility: Option<Visibility> as "Specifies allowed visibility of the base product"
    ) -> FieldResult<Option<Connection<BaseProduct, PageInfoProductsSearch>>> as "Find products by name using relay connection." {

        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();


        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/search?offset={}&count={}&visibility={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1,
            visibility
        );

        let options = if let Some(mut options) = search_term.options.clone() {
            options.status = Some(ModerationStatus::Published);
            options
        } else {
            ProductsSearchOptionsInput{
                status : Some(ModerationStatus::Published),
                ..ProductsSearchOptionsInput::default()
            }
        };

        let mut search_term = search_term;
        search_term.options = Some(options);

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .map (|products| {
                let mut product_edges = Edge::create_vec(products, offset);

                let search_filters = ProductsSearchFilters::new(search_term);
                let has_next_page = product_edges.len() as i32 == count + 1;
                if has_next_page {
                    product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  product_edges.get(0).map(|e| e.cursor.clone());
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
            .map(Some)
    }

    field auto_complete_product_name(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset form beginning",
        name : String as "Name part")
            -> FieldResult<Option<Connection<String, PageInfo>>> as "Finds products full name by part of the name." {

        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/auto_complete?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1,
            );

        let search_term = AutoCompleteProductNameInput {
            name,
            store_id : None,
            status: Some(ModerationStatus::Published),
        };

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<String>>(Method::Post, url, Some(body))
            .map (|full_names| {
                let mut full_name_edges = Edge::create_vec(full_names, offset);
                let has_next_page = full_name_edges.len() as i32 == count + 1;
                if has_next_page {
                    full_name_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  full_name_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = full_name_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(full_name_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field find_store(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset form beginning",
        search_term : SearchStoreInput as "Search store input",
        visibility: Option<Visibility> as "Specifies allowed visibility of the store"
    ) -> FieldResult<Option<Connection<Store, PageInfoStoresSearch>>> as "Finds stores by name using relay connection." {

        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let body = serde_json::to_string(&search_term)?;

        println!("{}", body);

        let url = format!("{}/{}/search?offset={}&count={}&visibility={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            offset,
            count + 1,
            visibility
        );

        context.request::<Vec<Store>>(Method::Post, url, Some(body))
            .and_then (|stores| {
                let mut store_edges = Edge::create_vec(stores, offset);
                let has_next_page = store_edges.len() as i32 == count + 1;
                if has_next_page {
                    store_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  store_edges.get(0).map(|e| e.cursor.clone());
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
            .map(Some)
    }

    field auto_complete_store_name(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset form beginning",
        name : String as "Name part")
            -> FieldResult<Option<Connection<String, PageInfo>>> as "Finds stores full name by part of the name." {
        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
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
                let mut full_name_edges = Edge::create_vec(full_names, offset);
                let has_next_page = full_name_edges.len() as i32 == count + 1;
                if has_next_page {
                    full_name_edges.pop();
                };
                let start_cursor =  full_name_edges.get(0).map(|e| e.cursor.clone());
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
            .map(Some)
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
            .map(Some)
    }

    field categories(&executor) -> FieldResult<Option<SearchCategory>> as "Category."{
        let context = executor.context();

        let body = serde_json::to_string(&self.search_term)?;

        let url = format!("{}/{}/search/filters/category",
                        context.config.service_url(Service::Stores),
                        Model::BaseProduct.to_url(),
                    );
        context.request::<SearchCategory>(Method::Post, url, Some(body))
            .wait()
            .map(Some)
    }

    field attr_filters(&executor) -> FieldResult<Option<Vec<AttributeFilter>>> as "Attribute filters for whole category."{
        let context = executor.context();

        let mut options = ProductsSearchOptionsInput::default();
        options.category_id = self.search_term.options
            .clone()
            .map(|o| o.category_id)
            .and_then(|x| x);
        options.status = Some(ModerationStatus::Published);
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

    field category(&executor) -> FieldResult<Option<Category>> as "Category."{
        let context = executor.context();

        let body = serde_json::to_string(&self.search_term)?;

        let url = format!("{}/{}/search/filters/category",
                    context.config.service_url(Service::Stores),
                    Model::Store.to_url(),
                    );

        context.request::<Category>(Method::Post, url, Some(body))
            .wait()
            .map(|mut cat|{
                cat.id = CategoryId(-1); //for Relay: root category and searched category must not have equal id
                Some(cat)
            })
    }

    field country(&executor) -> FieldResult<Option<Vec<String>>> as "Countries"{
        let context = executor.context();

        let body = serde_json::to_string(&self.search_term)?;

        let url = format!("{}/{}/search/filters/country",
                    context.config.service_url(Service::Stores),
                    Model::Store.to_url(),
                    );

        context.request::<Vec<String>>(Method::Post, url, Some(body))
            .wait()
            .map(Some)
    }

});
