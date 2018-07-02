//! File containing PageInfo object of graphql schema

use juniper;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(PageInfo: Context as "PageInfo" |&self| {
    description: "Page Info from relay spec: https://facebook.github.io/relay/graphql/connections.htm."

    field has_next_page() -> &bool as "has next page"{
        &self.has_next_page
    }

    field has_previous_page() -> &bool as "has previous page" {
        &self.has_previous_page
    }

    field end_cursor() -> &Option<juniper::ID> as "end cursor" {
        &self.end_cursor
    }

    field start_cursor() -> &Option<juniper::ID> as "start cursor" {
        &self.start_cursor
    }

});

graphql_object!(PageInfoStoresSearch: Context as "PageInfoStoresSearch" |&self| {
    description: "Page Info from relay spec: https://facebook.github.io/relay/graphql/connections.htm."

    field has_next_page() -> &bool as "has next page"{
        &self.has_next_page
    }

    field has_previous_page() -> &bool as "has previous page" {
        &self.has_previous_page
    }

    field deprecated "Use search_filters.total_count " total_count() -> Option<i32> as "total elements count" {
        Some(0)
    }

    field end_cursor() -> &Option<juniper::ID> as "end cursor" {
        &self.end_cursor
    }

    field start_cursor() -> &Option<juniper::ID> as "start cursor" {
        &self.start_cursor
    }

    field search_filters() -> &StoresSearchFilters as "search options" {
        &self.search_filters
    }

});

graphql_object!(PageInfoProductsSearch: Context as "PageInfoProductsSearch" |&self| {
    description: "Page Info from relay spec: https://facebook.github.io/relay/graphql/connections.htm."

    field has_next_page() -> &bool as "has next page"{
        &self.has_next_page
    }

    field has_previous_page() -> &bool as "has previous page" {
        &self.has_previous_page
    }

    field search_filters() -> &Option<ProductsSearchFilters> as "search options" {
        &self.search_filters
    }

    field end_cursor() -> &Option<juniper::ID> as "end cursor" {
        &self.end_cursor
    }

    field start_cursor() -> &Option<juniper::ID> as "start cursor" {
        &self.start_cursor
    }

});

graphql_object!(PageInfoOrdersSearch: Context as "PageInfoOrdersSearch" |&self| {
    description: "Page Info order."

    field total_pages() -> &i32 as "total_pages"{
        &self.total_pages
    }

    field current_page() -> &i32 as "current_page" {
        &self.current_page
    }

    field page_items_count() -> &i32 as "page_items_count" {
        &self.page_items_count
    }

    field search_term_options() -> &SearchOrderOption as "search options" {
        &self.search_term_options
    }

});

graphql_object!(PageInfoWarehouseProductSearch: Context as "PageInfoWarehouseProductSearch" |&self| {
    description: "Page Info Warehouse Product Search."

    field total_pages() -> &i32 as "total_pages"{
        &self.total_pages
    }

    field current_page() -> &i32 as "current_page" {
        &self.current_page
    }

    field page_items_count() -> &i32 as "page_items_count" {
        &self.page_items_count
    }

    field search_term_options() -> &Option<ProductsSearchFilters> as "search options" {
        &self.search_term_options
    }

});
