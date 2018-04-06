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
    pub total_count: Option<i32>,
    pub search_filters: Option<SearchOptions>,
    pub start_cursor: Option<juniper::ID>,
    pub end_cursor: Option<juniper::ID>,
}

#[derive(Debug, Clone)]
pub struct Connection<T> {
    pub edges: Vec<Edge<T>>,
    pub page_info: PageInfo,
}

impl<T> Connection<T> {
    pub fn new(edges: Vec<Edge<T>>, page_info: PageInfo) -> Self {
        Self {
            edges: edges,
            page_info: page_info,
        }
    }
}
