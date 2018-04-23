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
