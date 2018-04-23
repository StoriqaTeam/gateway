use juniper;
use super::*;

#[derive(Clone, Debug)]
pub struct Edge<T> {
    pub cursor: juniper::ID,
    pub node: T,
}

impl<T> Edge<T> {
    pub fn new(cursor: juniper::ID, node: T) -> Self {
        Self {
            cursor: cursor,
            node: node,
        }
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
        Self {
            edges: edges,
            page_info: page_info,
        }
    }
}
