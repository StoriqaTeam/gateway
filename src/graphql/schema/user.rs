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

use super::*;
use graphql::context::Context;
use graphql::models::*;

const MIN_ID: i32 = 0;

graphql_object!(User: Context as "User" |&self| {
    description: "User's profile."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Users, Model::User, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field email() -> &str as "Email" {
        &self.email
    }

    field phone() -> &Option<String> as "Phone" {
        &self.phone
    }

    field first_name() -> &Option<String> as "First name" {
        &self.first_name
    }

    field last_name() -> &Option<String> as "Last name" {
        &self.last_name
    }

    field middle_name() -> &Option<String> as "Middle name" {
        &self.middle_name
    }

    field gender() -> &Gender as "Gender" {
        &self.gender
    }

    field birthdate() -> &Option<String> as "Birthdate" {
        &self.birthdate
    }

    field avatar() -> &Option<String> as "Avatar" {
        &self.avatar
    }

    field isActive() -> &bool as "If the user was disabled (deleted), isActive is false" {
        &self.is_active
    }

    field provider(&executor) -> Option<Provider> as "Provider user has logged in with" {
        let context = executor.context();

        context.user.clone().map(|payload| payload.provider)
    }

    field roles(&executor) -> FieldResult<Option<Vec<Role>>> as "Fetches roles for user." {
        let context = executor.context();

        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Users),
            Model::UserRoles.to_url(),
            self.id);

        context.request::<Vec<Role>>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }

    field user(&executor, id: GraphqlID as "Base64 Id of a user.") -> FieldResult<Option<User>> as "Fetches user by id." {
        let context = executor.context();

        let identifier = ID::from_str(&*id)?;
        let url = identifier.url(&context.config);

        context.request::<Option<User>>(Method::Get, url, None)
            .wait()
    }

    field users(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Base64 Id of a user")
            -> FieldResult<Option<Connection<User, PageInfo>>> as "Fetches users using relay connection." {
        let context = executor.context();

        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}?offset={}&count={}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            raw_id,
            first + 1);

        context.request::<Vec<User>>(Method::Get, url, None)
            .map (|users| {
                let mut user_edges: Vec<Edge<User>> = users
                    .into_iter()
                    .map(|user| Edge::new(
                                juniper::ID::from(ID::new(Service::Users, Model::User, user.id.clone()).to_string()),
                                user.clone()
                            ))
                    .collect();
                let has_next_page = user_edges.len() as i32 == first + 1;
                if has_next_page {
                    user_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  user_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = user_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(user_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

    field store(&executor, id: i32 as "Int id of a store.") -> FieldResult<Option<Store>> as "Fetches store by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            id.to_string()
        );

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field stores(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Id of a store")
            -> FieldResult<Option<Connection<Store, PageInfo>>> as "Fetches stores using relay connection." {
        let context = executor.context();

        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            raw_id,
            first + 1);

        context.request::<Vec<Store>>(Method::Get, url, None)
            .map (|stores| {
                let mut store_edges: Vec<Edge<Store>> = stores
                    .into_iter()
                    .map(|store| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::Store, store.id.clone()).to_string()),
                                store.clone()
                            ))
                    .collect();
                let has_next_page = store_edges.len() as i32 == first + 1;
                if has_next_page {
                    store_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  store_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = store_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(store_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

    field product(&executor, id: i32 as "Int id of a product.") -> FieldResult<Option<Product>> as "Fetches product by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            id.to_string()
        );

        context.request::<Option<Product>>(Method::Get, url, None)
            .wait()
    }

    field products(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Base64 Id of a product")
            -> FieldResult<Option<Connection<Product, PageInfo>>> as "Fetches products using relay connection." {
        let context = executor.context();

        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            raw_id,
            first + 1);

        context.request::<Vec<Product>>(Method::Get, url, None)
            .map (|products| {
                let mut product_edges: Vec<Edge<Product>> = products
                    .into_iter()
                    .map(|product| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::Product, product.id.clone()).to_string()),
                                product.clone()
                            ))
                    .collect();
                let has_next_page = product_edges.len() as i32 == first + 1;
                if has_next_page {
                    product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  product_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(product_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

    field base_product(&executor, id: i32 as "Int Id of a base product.") -> FieldResult<Option<BaseProduct>> as "Fetches base product by id." {
        let context = executor.context();

       let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            id.to_string()
        );

        context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()
    }

    field base_products(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Base64 Id of base product")
            -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Fetches base products using relay connection." {
        let context = executor.context();

        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            raw_id,
            first + 1);

        context.request::<Vec<BaseProduct>>(Method::Get, url, None)
            .map (|products| {
                let mut product_edges: Vec<Edge<BaseProduct>> = products
                    .into_iter()
                    .map(|product| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::BaseProduct, product.id.clone()).to_string()),
                                product.clone()
                            ))
                    .collect();
                let has_next_page = product_edges.len() as i32 == first + 1;
                if has_next_page {
                    product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  product_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(product_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

    field deprecated "use query cart" cart(&executor) -> FieldResult<Option<Cart>> as "Fetches cart products." {
        let context = executor.context();

        let id = context.session_id.unwrap_or(self.id);

        let body = serde_json::to_string(&CartMergePayload {user_from: id})?;

        let url = format!("{}/cart/merge",
            &context.config.service_url(Service::Orders));

        context.request::<CartHash>(Method::Post, url, Some(body))
            .map (|hash| hash.into_iter()
                .map(|(product_id, info)| OrdersCartProduct {
                    product_id,
                    quantity: info.quantity,
                    store_id: info.store_id,
                    selected: info.selected,
                    comment: info.comment,
            }).collect::<Vec<OrdersCartProduct>>())
            .map(|u| Some(Cart::new(u)))
            .wait()
    }

    field wizard_store(&executor) -> FieldResult<Option<WizardStore>> as "Fetches wizard store." {
        let context = executor.context();

        let url = format!("{}/{}",
            &context.config.service_url(Service::Stores),
            Model::WizardStore.to_url(),
            );

        context.request::<Option<WizardStore>>(Method::Get, url, None)
            .wait()
    }

    field delivery_addresses(&executor) -> FieldResult<Option<Vec<UserDeliveryAddress>>> as "Fetches delivery addresses for user." {
        let context = executor.context();

        let url = format!("{}/{}/delivery_addresses/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            self.id);

        context.request::<Vec<UserDeliveryAddress>>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }

    field orders(&executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
        search_term_options : Option<SearchOrderOptionInput> as "Search options pattern")
            -> FieldResult<Option<Connection<Order, PageInfoOrdersSearch>>> as "Fetches orders using relay connection." {
        let context = executor.context();

        let offset = items_count * (current_page - 1);

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(items_count, records_limit as i32);

        let search_term = search_term_options.clone().map(|options| {
            let created_from= options.date.clone().and_then(|date|
                date.min_value.map(|value| i64::from_str(&value).unwrap_or_default()));

            let created_to = options.date.and_then(|date|
                date.max_value.map(|value| i64::from_str(&value).unwrap_or_default()));

            SearchOrder {
                slug: options.slug,
                customer: Some(self.id),
                store: None,
                created_from,
                created_to,
                payment_status: options.payment_status,
                state: options.order_status,
            }
        });

        let body = serde_json::to_string(&search_term)?;

        let url = format!("{}/{}/search",
            context.config.service_url(Service::Orders),
            Model::Order.to_url());

        context.request::<Vec<Order>>(Method::Post, url, Some(body))
            .map (|orders| {
                let total_pages = orders.iter().count() as i32;

                let mut orders_edges: Vec<Edge<Order>> = orders
                    .into_iter()
                    .skip(offset as usize)
                    .take(count as usize)
                    .map(|order| Edge::new(
                                juniper::ID::from(ID::new(Service::Orders, Model::Order, order.id.clone()).to_string()),
                                order.clone()
                            ))
                    .collect();

                let page_info = PageInfoOrdersSearch {
                    total_pages,
                    current_page,
                    page_items_count: items_count,
                    search_term_options};
                Connection::new(orders_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

});

graphql_object!(Connection<User, PageInfo>: Context as "UsersConnection" |&self| {
    description:"Users Connection"

    field edges() -> &[Edge<User>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<User>: Context as "UsersEdge" |&self| {
    description:"Users Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &User {
        &self.node
    }
});

graphql_object!(Connection<CartProduct, PageInfo>: Context as "CartProductConnection" |&self| {
    description:"CartProduct Connection"

    field edges() -> &[Edge<CartProduct>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<CartProduct>: Context as "CartProductEdge" |&self| {
    description:"CartProduct Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &CartProduct {
        &self.node
    }
});
