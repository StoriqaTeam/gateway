//! File containing Category object of graphql schema
use graphql::context::Context;
use graphql::models::*;

graphql_object!(PageInfo: Context as "PageInfo" |&self| {
    description: "Page Info from relay spec: https://facebook.github.io/relay/graphql/connections.htm."

    field has_next_page() -> bool as "has next page"{
        self.has_next_page
    }

    field has_previous_page() -> bool as "has previous page" {
        self.has_previous_page
    }

    field total_count() -> Option<i32> as "total elements count" {
        self.total_count.clone()
    }
    
    field search_filters() -> Option<SearchOptions> as "search options" {
        self.search_filters.clone()
    }
    
});
