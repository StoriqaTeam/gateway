//! File containing user object of graphql schema
use std::cmp;
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper;
use juniper::FieldResult;
use juniper::ID as GraphqlID;
use serde_json;

use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

const MIN_ID: i32 = 0;

graphql_object!(Admin: Context as "Admin" |&self| {
    description: "Admin's profile."

    field user(&executor, id: GraphqlID as "Base64 Id of a user.") -> FieldResult<Option<User>> as "Fetches user by id." {
        let context = executor.context();

        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.request::<Option<User>>(Method::Get, url, None)
            .wait()
    }

    field deprecated "use usersSearchPages" users_search(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Base64 Id of a user",
        search_term : SearchUserInput as "Search pattern"
        )
            -> FieldResult<Option<Connection<User, PageInfo>>> as "Searching for users using relay connection." {
        let context = executor.context();

        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id + 1,
            None => MIN_ID
        };

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/search?offset={}&count={}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            raw_id,
            first + 1);

        let body = serde_json::to_string(&search_term)?;

        context.request::<UserSearchResults>(Method::Post, url, Some(body))
            .map (|UserSearchResults { users, .. }| {
                let mut user_edges: Vec<Edge<User>> = users
                    .into_iter()
                    .map(|user| Edge::new(
                        juniper::ID::from(ID::new(Service::Users, Model::User, user.id.0).to_string()),
                        user.clone()
                    )).collect();
                let has_next_page = user_edges.len() as i32 == first + 1;
                if has_next_page {
                    user_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  user_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = user_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(user_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field users_search_pages(&executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
        search_term : SearchUserInput as "Search pattern"
        )
            -> FieldResult<Option<Connection<User, PageInfoSegments>>> as "Searching for users using relay connection." {
        let context = executor.context();

        let current_page = cmp::max(current_page, 1);

        let records_limit = context.config.gateway.records_limit;
        let items_count = cmp::max(1, cmp::min(items_count, records_limit as i32));

        let skip = items_count * (current_page - 1);

        let url = format!(
            "{}/{}/search?skip={}&count={}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            skip, items_count,
        );

        let body = serde_json::to_string(&search_term)?;

        context.request::<UserSearchResults>(Method::Post, url, Some(body))
            .map(|UserSearchResults { total_count, users }| {
                let total_pages = cmp::max(0, total_count as i32 - 1) / items_count + 1;
                let mut user_edges: Vec<Edge<User>> = users
                    .into_iter()
                    .map(|user| Edge::new(
                        juniper::ID::from(ID::new(Service::Users, Model::User, user.id.0).to_string()), user.clone()
                    )).collect();
                let page_info = PageInfoSegments {
                    current_page,
                    page_items_count: items_count,
                    total_pages,
                };
                Connection::new(user_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field deprecated "use storesSearchPages" stores_search(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Base64 Id of a store",
        search_term : SearchModeratorStoreInput as "Search pattern"
        )
            -> FieldResult<Option<Connection<Store, PageInfo>>> as "Searching stores by moderator using relay connection." {
        let context = executor.context();

        let store_manager_ids = if let Some(ref store_manager_email) = search_term.store_manager_email {
            let url = format!("{}/{}/search/by_email?email={}",
                context.config.service_url(Service::Users),
                Model::User.to_url(),
                store_manager_email);

            let users_ids = context.request::<Vec<User>>(Method::Get, url, None)
                .wait()?
                .into_iter()
                .map(|user| user.id).collect();
            Some(users_ids)
        } else {
            None
        };

        let term: SearchModeratorStore = SearchModeratorStore::new(search_term, store_manager_ids);

        let body = serde_json::to_string(&term)?;

        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/moderator_search?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            raw_id,
            first);

        context.request::<StoreSearchResults>(Method::Post, url, Some(body))
            .map(|StoreSearchResults { stores, .. }| {
                let mut store_edges: Vec<Edge<Store>> = stores
                    .into_iter()
                    .map(|store| Edge::new(
                        juniper::ID::from(ID::new(Service::Stores, Model::Store, store.id.0).to_string()),
                        store.clone()
                    )).collect();
                let has_next_page = store_edges.len() as i32 == first + 1;
                if has_next_page {
                    store_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  store_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = store_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(store_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field stores_search_pages(&executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
        search_term : SearchModeratorStoreInput as "Search pattern"
        )
            -> FieldResult<Option<Connection<Store, PageInfoSegments>>> as "Searching stores by moderator using relay connection." {
        let context = executor.context();

        let store_manager_ids = if let Some(ref store_manager_email) = search_term.store_manager_email {
            let url = format!("{}/{}/search/by_email?email={}",
                context.config.service_url(Service::Users),
                Model::User.to_url(),
                store_manager_email);

            let users_ids = context.request::<Vec<User>>(Method::Get, url, None)
                .wait()?
                .into_iter()
                .map(|user| user.id).collect();
            Some(users_ids)
        } else {
            None
        };

        let term: SearchModeratorStore = SearchModeratorStore::new(search_term, store_manager_ids);

        let current_page = cmp::max(current_page, 1);

        let records_limit = context.config.gateway.records_limit;
        let items_count = cmp::max(1, cmp::min(items_count, records_limit as i32));

        let skip = items_count * (current_page - 1);

        let body = serde_json::to_string(&term)?;

        let url = format!("{}/{}/moderator_search?skip={}&count={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            skip, items_count,
        );

        context.request::<StoreSearchResults>(Method::Post, url, Some(body))
            .map(|StoreSearchResults { stores, total_count }| {
                let total_pages = cmp::max(0, total_count as i32 - 1) / items_count + 1;
                let mut store_edges: Vec<Edge<Store>> = stores
                    .into_iter()
                    .map(|store| Edge::new(
                        juniper::ID::from(ID::new(Service::Stores, Model::Store, store.id.0).to_string()),
                        store.clone()
                    )).collect();
                let page_info = PageInfoSegments {
                    current_page,
                    page_items_count: items_count,
                    total_pages,
                };
                Connection::new(store_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field deprecated "use baseProductsSearchPages" base_products_search(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Base64 Id of a base_product",
        search_term : SearchModeratorBaseProductInput as "Search pattern"
    ) -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Searching base_products by moderator using relay connection." {
        base_products_search(executor.context(), first, after, search_term)
    }

    field base_products_search_pages(&executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
        search_term : SearchModeratorBaseProductInput as "Search pattern"
    ) -> FieldResult<Option<Connection<BaseProduct, PageInfoSegments>>> as "Searching base_products by moderator using relay connection." {
        base_products_search_pages(executor.context(), current_page, items_count, search_term)
    }
});

pub fn base_products_search(
    context: &Context,
    first: Option<i32>,
    after: Option<GraphqlID>,
    search_term: SearchModeratorBaseProductInput,
) -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> {
    let body = serde_json::to_string(&search_term)?;

    let raw_id = match after {
        Some(val) => ID::from_str(&*val)?.raw_id,
        None => MIN_ID,
    };

    let records_limit = context.config.gateway.records_limit;
    let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

    let url = format!(
        "{}/{}/moderator_search?offset={}&count={}",
        context.config.service_url(Service::Stores),
        Model::BaseProduct.to_url(),
        raw_id,
        first
    );

    context
        .request::<BaseProductSearchResults>(Method::Post, url, Some(body))
        .map(|BaseProductSearchResults { base_products, .. }| {
            let mut base_product_edges: Vec<Edge<BaseProduct>> = base_products
                .into_iter()
                .map(|base_product| {
                    Edge::new(
                        juniper::ID::from(ID::new(Service::Stores, Model::BaseProduct, base_product.id.0).to_string()),
                        base_product.clone(),
                    )
                })
                .collect();
            let has_next_page = base_product_edges.len() as i32 == first + 1;
            if has_next_page {
                base_product_edges.pop();
            };
            let has_previous_page = true;
            let start_cursor = base_product_edges.get(0).map(|e| e.cursor.clone());
            let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
            let page_info = PageInfo {
                has_next_page,
                has_previous_page,
                start_cursor,
                end_cursor,
            };
            Connection::new(base_product_edges, page_info)
        })
        .wait()
        .map(Some)
}

pub fn base_products_search_pages(
    context: &Context,
    current_page: i32,
    items_count: i32,
    search_term: SearchModeratorBaseProductInput,
) -> FieldResult<Option<Connection<BaseProduct, PageInfoSegments>>> {
    let current_page = cmp::max(current_page, 1);

    let records_limit = context.config.gateway.records_limit;
    let items_count = cmp::max(1, cmp::min(items_count, records_limit as i32));

    let skip = items_count * (current_page - 1);

    let url = format!(
        "{}/{}/moderator_search?skip={}&count={}",
        context.config.service_url(Service::Stores),
        Model::BaseProduct.to_url(),
        skip,
        items_count,
    );

    let body = serde_json::to_string(&search_term)?;

    context
        .request::<BaseProductSearchResults>(Method::Post, url, Some(body))
        .map(
            |BaseProductSearchResults {
                 base_products,
                 total_count,
             }| {
                let total_pages = cmp::max(0, total_count as i32 - 1) / items_count + 1;
                let base_product_edges: Vec<Edge<BaseProduct>> = base_products
                    .into_iter()
                    .map(|base_product| {
                        Edge::new(
                            juniper::ID::from(ID::new(Service::Stores, Model::BaseProduct, base_product.id.0).to_string()),
                            base_product.clone(),
                        )
                    })
                    .collect();
                let page_info = PageInfoSegments {
                    current_page,
                    page_items_count: items_count,
                    total_pages,
                };
                Connection::new(base_product_edges, page_info)
            },
        )
        .wait()
        .map(Some)
}
