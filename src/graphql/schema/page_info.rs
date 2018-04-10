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

graphql_object!(PageInfoWithTotalCount: Context as "PageInfoWithTotalCount" |&self| {
    description: "Page Info from relay spec: https://facebook.github.io/relay/graphql/connections.htm."

    field has_next_page() -> &bool as "has next page"{
        &self.has_next_page
    }

    field has_previous_page() -> &bool as "has previous page" {
        &self.has_previous_page
    }

    field total_count() -> &Option<i32> as "total elements count" {
        &self.total_count
    }
    
    field end_cursor() -> &Option<juniper::ID> as "end cursor" {
        &self.end_cursor
    }
    
    field start_cursor() -> &Option<juniper::ID> as "start cursor" {
        &self.start_cursor
    }
    
});

graphql_object!(PageInfoWithSearchFilters<SearchFiltersInCategory>: Context as "PageInfoWithSearchFiltersInCategory" |&self| {
    description: "Page Info from relay spec: https://facebook.github.io/relay/graphql/connections.htm."

    field has_next_page() -> &bool as "has next page"{
        &self.has_next_page
    }

    field has_previous_page() -> &bool as "has previous page" {
        &self.has_previous_page
    }

    field search_filters() -> &Option<SearchFiltersInCategory> as "search options" {
        &self.search_filters
    }
    
    field end_cursor() -> &Option<juniper::ID> as "end cursor" {
        &self.end_cursor
    }
    
    field start_cursor() -> &Option<juniper::ID> as "start cursor" {
        &self.start_cursor
    }
    
});

graphql_object!(PageInfoWithSearchFilters<SearchFiltersWithoutCategory>: Context as "PageInfoWithSearchFiltersWithoutCategory" |&self| {
    description: "Page Info from relay spec: https://facebook.github.io/relay/graphql/connections.htm."

    field has_next_page() -> &bool as "has next page"{
        &self.has_next_page
    }

    field has_previous_page() -> &bool as "has previous page" {
        &self.has_previous_page
    }

    field search_filters() -> &Option<SearchFiltersWithoutCategory> as "search options" {
        &self.search_filters
    }
    
    field end_cursor() -> &Option<juniper::ID> as "end cursor" {
        &self.end_cursor
    }
    
    field start_cursor() -> &Option<juniper::ID> as "start cursor" {
        &self.start_cursor
    }
    
});
