use super::*;
use juniper;

#[derive(Clone, Debug)]
pub struct Edge<T> {
    pub cursor: juniper::ID,
    pub node: T,
}

impl<T> Edge<T> {
    pub fn new(cursor: juniper::ID, node: T) -> Self {
        Self { cursor, node }
    }

    pub fn create_vec(vec: Vec<T>, offset: i32) -> Vec<Edge<T>> {
        vec.into_iter()
            .enumerate()
            .map(|(i, item)| Edge::new(juniper::ID::from((i as i32 + offset).to_string()), item))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<juniper::ID>,
    pub end_cursor: Option<juniper::ID>,
}

#[derive(Clone, Debug)]
pub struct PageInfoSegments {
    pub current_page: i32,
    pub page_items_count: i32,
    pub total_pages: i32,
}

#[derive(Clone, Debug)]
pub struct PageInfoStoresSearch {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<juniper::ID>,
    pub end_cursor: Option<juniper::ID>,
    pub search_filters: StoresSearchFilters,
}

#[derive(Clone, Debug)]
pub struct PageInfoProductsSearch {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub search_filters: Option<ProductsSearchFilters>,
    pub start_cursor: Option<juniper::ID>,
    pub end_cursor: Option<juniper::ID>,
}

#[derive(Debug, Clone)]
pub struct Connection<T, P> {
    pub edges: Vec<Edge<T>>,
    pub page_info: P,
}

impl<T, P> Connection<T, P> {
    pub fn new(edges: Vec<Edge<T>>, page_info: P) -> Self {
        Self { edges, page_info }
    }
}
