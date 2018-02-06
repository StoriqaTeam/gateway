use juniper;

#[derive(Clone)]
pub struct Edge<T> {
    pub cursor: juniper::ID,
    pub node: T,
}

impl<T> Edge<T> {
    pub fn new (cursor: juniper::ID, node: T) -> Self {
        Self {
            cursor: cursor,
            node:node
        }
    }
}


#[derive(GraphQLObject, Clone)]
#[graphql(name = "PageInfo", description = "Page Info from relay spec: https://facebook.github.io/relay/graphql/connections.htm")]
pub struct PageInfo {
    #[graphql(description = "has next page")] 
    pub has_next_page: bool,

    #[graphql(description = "has previous page")] 
    pub has_previous_page: bool,
}


pub struct Connection<T> {
    pub edges: Vec<Edge<T>>,
    pub page_info: PageInfo,
}

impl<T> Connection<T> {
    pub fn new (edges: Vec<Edge<T>>, page_info: PageInfo) -> Self {
        Self {
            edges: edges,
            page_info: page_info
        }
    }
}