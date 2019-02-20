//! File containing node object of graphql schema
//! File containing store object of graphql schema
use std::cmp;
use std::str::FromStr;

use chrono::prelude::*;
use futures::Future;
use hyper::Method;
use juniper;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};
use serde_json;

use stq_api::orders::{OrderClient, OrderSearchTerms};
use stq_api::types::ApiFutureExt;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::{Language, ModerationStatus, Translation};
use stq_types::{OrderIdentifier, OrderSlug, ProductId, StoreId};

use super::*;
use errors::into_graphql;
use graphql::context::Context;
use graphql::microservice::CalculatePayoutPayload;
use graphql::models::*;
use graphql::schema::warehouse as warehouse_module;
use schema::admin::{base_products_search, base_products_search_pages};
use schema::order as order_module;

graphql_object!(Store: Context as "Store" |&self| {
    description: "Store's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Store, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field user_id() -> &i32 as "Store manager id"{
        &self.user_id.0
    }

    field store_manager(&executor) -> FieldResult<Option<User>> as "Fetches store manager by user_id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Users),
            Model::User.to_url(),
            self.user_id.to_string()
        );

        context.request::<Option<User>>(Method::Get, url, None)
            .wait()
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field isActive() -> &bool as "If the store was disabled (deleted), isActive is false" {
        &self.is_active
    }

    field short_description() -> &[Translation] as "Short description" {
        &self.short_description
    }

    field long_description() -> &Option<Vec<Translation>> as "Long description" {
        &self.long_description
    }

    field slug() -> &str as "Slug" {
        &self.slug
    }

    field created_at() -> String as "Created at" {
        let datetime: DateTime<Utc> = self.created_at.into();
        datetime.to_rfc3339()
    }

    field updated_at() -> String as "Updated at" {
        let datetime: DateTime<Utc> = self.updated_at.into();
        datetime.to_rfc3339()
    }

    field cover() -> &Option<String> as "Cover" {
        &self.cover
    }

    field logo() -> &Option<String> as "Logo" {
        &self.logo
    }

    field phone() -> &Option<String> as "Phone" {
        &self.phone
    }

    field email() -> &Option<String> as "Email" {
        &self.email
    }

    field facebook_url() -> &Option<String> as "Facebook url" {
        &self.facebook_url
    }

    field twitter_url() -> &Option<String> as "Twitter url" {
        &self.twitter_url
    }

    field instagram_url() -> &Option<String> as "Instagram url" {
        &self.instagram_url
    }

    field default_language() -> &Language as "Default language" {
        &self.default_language
    }

    field slogan() -> &Option<String> as "Slogan" {
        &self.slogan
    }

    field rating() -> &f64 as "Rating" {
        &self.rating
    }

    field status() -> &ModerationStatus as "Moderation Status" {
        &self.status
    }

    field deprecated "Use address_full -> value" address() -> &Option<String> as "address" {
        &self.address
    }

    field deprecated "Use address_full -> country" country() -> &Option<String> as "country" {
        &self.country
    }

    field address_full() -> Address as "full address" {
        self.clone().into()
    }

    field base_products(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID> as "After base_product GraphQL id",
        skip_base_prod_id = None : Option<i32> as "Skip base prod id",
        visibility: Option<Visibility> as "Specifies visibility of the base products")
            -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Fetches base products of the store." {
        let context = executor.context();

        let offset = after
            .and_then(|val| ID::from_str(&*val).map(|id| id.raw_id + 1).ok())
            .unwrap_or_default();
        let visibility = visibility.unwrap_or(Visibility::Active);

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

            let url = match skip_base_prod_id {
                None => format!(
                        "{}/{}/{}/products?offset={}&count={}&visibility={}",
                        &context.config.service_url(Service::Stores),
                        Model::Store.to_url(),
                        self.id,
                        offset,
                        count + 1,
                        visibility,
                    ),
                Some(id) => format!(
                        "{}/{}/{}/products?skip_base_product_id={}&offset={}&count={}&visibility={}",
                        &context.config.service_url(Service::Stores),
                        Model::Store.to_url(),
                        self.id,
                        id,
                        offset,
                        count + 1,
                        visibility,
                    )
            };

            context.request::<Vec<BaseProduct>>(Method::Get, url, None)
                .map (|base_products| {
                    let mut base_product_edges: Vec<Edge<BaseProduct>> =  vec![];
                    for base_product in base_products {
                        let edge = Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::BaseProduct, base_product.id.0).to_string()),
                                base_product.clone()
                            );
                        base_product_edges.push(edge);
                    }
                    let has_next_page = base_product_edges.len() as i32 > count;
                    if has_next_page {
                        base_product_edges.pop();
                    };
                    let has_previous_page = true;
                    let start_cursor =  base_product_edges.get(0).map(|e| e.cursor.clone());
                    let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
                    let page_info = PageInfo {
                        has_next_page,
                        has_previous_page,
                        start_cursor,
                        end_cursor};
                    Connection::new(base_product_edges, page_info)
                })
                .wait()
                .map(Some)
        }

    field products_count(&executor) -> FieldResult<i32> as "Fetches products count of the store." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}/products/count",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.id,
        );

        context.request::<i32>(Method::Get, url, None)
            .wait()
    }

    field moderator_comment(&executor) -> FieldResult<Option<ModeratorStoreComments>> as "Fetches moderator comment by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::ModeratorStoreComment.to_url(),
            self.id.to_string()
        );

        context.request::<Option<ModeratorStoreComments>>(Method::Get, url, None)
            .wait()
    }

    field warehouses(&executor) -> FieldResult<Vec<GraphQLWarehouse>> as "Fetches store warehouses." {
        let context = executor.context();

        warehouse_module::get_warehouses_for_store(context, self.id).map(|res| res.into_iter().map(GraphQLWarehouse).collect())
    }

    field orders(&executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
        search_term_options : SearchOrderOptionInput as "Search options pattern")
            -> FieldResult<Option<Connection<GraphQLOrder, PageInfoOrdersSearch>>> as "Fetches orders using relay connection." {
        let context = executor.context();

        let offset = items_count * (current_page - 1);

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(items_count, records_limit as i32);

        let created_from = match search_term_options.created_from.clone() {
            Some(value) => {
                match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => return Err(FieldError::new(
                        "Parsing created_from error",
                        graphql_value!({ "code": 300, "details": { "created_from has wrong format." }}),
                    )),
                }
            },
            None => None
        };

        let created_to = match search_term_options.created_to.clone() {
            Some(value) => {
                match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => return Err(FieldError::new(
                        "Parsing created_to error",
                        graphql_value!({ "code": 300, "details": { "created_to has wrong format." }}),
                    )),
                }
            },
            None => None
        };

        let customer = search_term_options.email.clone().and_then(|email| {
            let url = format!("{}/{}/by_email?email={}",
                context.config.service_url(Service::Users),
                Model::User.to_url(),
                email);

            context.request::<Option<User>>(Method::Get, url, None)
                .wait()
                .ok()
                .and_then (|user| user.map(|u|u.id))
        });

        let search_term = OrderSearchTerms {
                slug: search_term_options.slug.map(OrderSlug),
                customer,
                store: Some(self.id),
                created_from,
                created_to,
                payment_status: search_term_options.payment_status,
                state: search_term_options.order_status,
                ..OrderSearchTerms::default()
            };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        rpc_client.search(search_term)
            .sync()
            .map_err(into_graphql)
            .map(|res| res.into_iter().map(GraphQLOrder).collect())
            .map (move |orders: Vec<GraphQLOrder>| {
                let total_pages = (orders.iter().count() as f32 / items_count as f32).ceil() as i32;

                let mut orders_edges: Vec<Edge<GraphQLOrder>> = orders
                    .into_iter()
                    .skip(offset as usize)
                    .take(count as usize)
                    .map(|order| Edge::new(
                                juniper::ID::from(order.0.id.to_string()),
                                order.clone()
                            ))
                    .collect();

                let page_info = PageInfoOrdersSearch {
                    total_pages,
                    current_page,
                    page_items_count: items_count,
                    search_term_options: search_term_options.into()
                };
                Connection::new(orders_edges, page_info)
            })
            .map(Some)
    }

    field order(&executor, slug: i32 as "Order slug" ) -> FieldResult<Option<GraphQLOrder>> as "Fetches order." {
        let context = executor.context();

        order_module::try_get_order(context, OrderIdentifier::Slug(OrderSlug(slug)))
    }

    field find_most_viewed_products(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset from beginning",
        search_term : MostViewedProductsInput as "Most viewed search pattern")
            -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Find most viewed base products each one contains one variant." {
        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/most_viewed?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1
            );

        let mut options = if let Some(mut options) = search_term.options.clone() {
            options.store_id = Some(self.id.0);
            options
        } else {
            ProductsSearchOptionsInput{
                store_id : Some(self.id.0),
                ..ProductsSearchOptionsInput::default()
            }
        };

        options.status = Some(ModerationStatus::Published);

        let mut search_term = search_term;
        search_term.options = Some(options);

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .map (|base_products| {
                let mut base_product_edges = Edge::create_vec(base_products, offset);
                let has_next_page = base_product_edges.len() as i32 == count + 1;
                if has_next_page {
                    base_product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  base_product_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(base_product_edges, page_info)
            })
            .wait()
            .map(Some)
    }


    field find_most_discount_products(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset from beginning",
        search_term : MostDiscountProductsInput as "Most discount search pattern")
            -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Find base products each one with most discount variant." {
        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/most_discount?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1
            );

        let mut options = if let Some(mut options) = search_term.options.clone() {
            options.store_id = Some(self.id.0);
            options
        } else {
            ProductsSearchOptionsInput{
                store_id : Some(self.id.0),
                ..ProductsSearchOptionsInput::default()
            }
        };

        options.status = Some(ModerationStatus::Published);

        let mut search_term = search_term;
        search_term.options = Some(options);

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .map (|base_products| {
                let mut base_product_edges = Edge::create_vec(base_products, offset);
                let has_next_page = base_product_edges.len() as i32 == count + 1;
                if has_next_page {
                    base_product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  base_product_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(base_product_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field find_product(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset form beginning",
        search_term : SearchProductInput as "Search pattern",
        visibility: Option<Visibility> as "Specifies allowed visibility of the base product"
        )
            -> FieldResult<Option<Connection<BaseProduct, PageInfoProductsSearch>>> as "Find products by name using relay connection." {

        let context = executor.context();

        let visibility = visibility.unwrap_or_default();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();


        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/search?offset={}&count={}&visibility={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1,
            visibility
            );

        let mut options = search_term.options.clone().unwrap_or_default();

        options.store_id = Some(self.id.0);

        if visibility == Visibility::Published {
            options.status = Some(ModerationStatus::Published);
        };

        let mut search_term = search_term;
        search_term.options = Some(options);

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .map (|products| {
                let mut product_edges = Edge::create_vec(products, offset);
                let search_filters = ProductsSearchFilters::new(search_term);
                let has_next_page = product_edges.len() as i32 == count + 1;
                if has_next_page {
                    product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  product_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfoProductsSearch {
                    has_next_page,
                    has_previous_page,
                    search_filters: Some(search_filters),
                    start_cursor,
                    end_cursor};
                Connection::new(product_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field auto_complete_product_name(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset form beginning",
        name : String as "Name part")
            -> FieldResult<Option<Connection<String, PageInfo>>> as "Finds products full name by part of the name." {

        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}/auto_complete?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            offset,
            count + 1,
            );

        let search_term = AutoCompleteProductNameInput {
            name,
            store_id : Some(self.id.0),
            status: Some(ModerationStatus::Published),
        };

        let body = serde_json::to_string(&search_term)?;

        context.request::<Vec<String>>(Method::Post, url, Some(body))
            .map (|full_names| {
                let mut full_name_edges = Edge::create_vec(full_names, offset);
                let has_next_page = full_name_edges.len() as i32 == count + 1;
                if has_next_page {
                    full_name_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  full_name_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = full_name_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(full_name_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field deprecated "use findProductsAdminPages" find_products_admin(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Base64 Id of a base_product",
        search_term : SearchModeratorBaseProductInput as "Search pattern"
    ) -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Searching base_products by moderator using relay connection." {

        let mut search_term = search_term;
        if search_term.store_id.is_none() {
            search_term.store_id = Some(self.id.0);
        }

        base_products_search(executor.context(), first, after, search_term)
    }

    field find_products_admin_pages(&executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
        search_term : SearchModeratorBaseProductInput as "Search pattern"
    ) -> FieldResult<Option<Connection<BaseProduct, PageInfoSegments>>> as "Searching base_products by moderator using relay connection." {

        let mut search_term = search_term;
        if search_term.store_id.is_none() {
            search_term.store_id = Some(self.id.0);
        }

        base_products_search_pages(executor.context(), current_page, items_count, search_term)
    }

    field coupons(&executor) -> FieldResult<Vec<Coupon>> {
        let context = executor.context();
        let url = format!("{}/{}/stores/{}",
            context.config.service_url(Service::Stores),
            Model::Coupon.to_url(),
            self.id);
        context.request::<Vec<Coupon>>(Method::Get, url, None).wait()
    }

    field paid_to_seller_orders
    (
        &executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
    )
    -> FieldResult<Connection<OrderBilling, PageInfoSegments>> as "find orders with PaidToSeller state."
    {
        let context = executor.context();
        let search_params = OrderBillingSearchInput {
            payment_state: Some(PaymentState::PaidToSeller),
            store_id: Some(self.id.0),
            ..Default::default()
        };
        orders_billing(context, current_page, items_count, search_params)
    }

    field payment_to_seller_needed_orders(
        &executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
    )
    -> FieldResult<Connection<OrderBilling, PageInfoSegments>> as "find orders with PaymentToSellerNeeded state."
    {
        let context = executor.context();
        let search_params = OrderBillingSearchInput {
            payment_state: Some(PaymentState::PaymentToSellerNeeded),
            store_id: Some(self.id.0),
            ..Default::default()
        };
        orders_billing(context, current_page, items_count, search_params)
    }

    field billing_type(&executor) -> FieldResult<Option<BillingType>> as "Store billing type." {
        let context = executor.context();
        context.get_billing_microservice().billing_type(self.id)
    }

    field international_billing_info(&executor) -> FieldResult<Option<InternationalBillingInfo>> as "International billing info." {
        let context = executor.context();
        context.get_billing_microservice().international_billing_info(self.id)
    }

    field russia_billing_info(&executor) -> FieldResult<Option<RussiaBillingInfo>> as "International billing info." {
        let context = executor.context();
        context.get_billing_microservice().russia_billing_info(self.id)
    }

    field calculate_payout(&executor, input: CalculatePayoutInput)
        -> FieldResult<PayoutCalculation> as "Calculate payout for orders in a particular currency." {
        let context = executor.context();

        let CalculatePayoutInput {
            currency,
            wallet_address,
        } = input;

        let payload = CalculatePayoutPayload {
            store_id: self.id,
            currency: currency.to_string().to_ascii_lowercase(),
            wallet_address,
        };

        context.get_billing_microservice().calculate_payout(payload)
    }

    field get_payouts(&executor) -> FieldResult<PayoutsByStoreId> as "Get payouts for this store." {
        let context = executor.context();
        context.get_billing_microservice().get_payouts_by_store_id(self.id)
    }
});

graphql_object!(Connection<Store, PageInfo>: Context as "StoresConnection" |&self| {
    description:"Stores Connection"

    field edges() -> &[Edge<Store>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Connection<Store, PageInfoSegments>: Context as "StoresConnectionPages" |&self| {
    description: "Stores Connection"

    field edges() -> &[Edge<Store>] {
        &self.edges
    }

    field page_info() -> &PageInfoSegments {
        &self.page_info
    }
});

graphql_object!(Edge<Store>: Context as "StoresEdge" |&self| {
    description:"Stores Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &Store {
        &self.node
    }
});

graphql_object!(Connection<String, PageInfo>: Context as "FullNameConnection" |&self| {
    description:"Name Connection"

    field edges() -> &[Edge<String>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Connection<Store, PageInfoStoresSearch>: Context as "StoresWithTotalCountConnection" |&self| {
    description:"Stores Connection"

    field edges() -> &[Edge<Store>] {
        &self.edges
    }

    field page_info() -> &PageInfoStoresSearch {
        &self.page_info
    }
});

graphql_object!(Edge<String>: Context as "FullNameEdge" |&self| {
    description:"Name Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &str {
        &self.node
    }
});

graphql_object!(Connection<GraphQLOrder, PageInfoOrdersSearch>: Context as "OrderSearchConnection" |&self| {
    description:"Order Search Connection"

    field edges() -> &[Edge<GraphQLOrder>] {
        &self.edges
    }

    field page_info() -> &PageInfoOrdersSearch {
        &self.page_info
    }
});

fn orders_billing(
    context: &Context,
    current_page: i32,
    items_count: i32,
    search_params: OrderBillingSearchInput,
) -> FieldResult<Connection<OrderBilling, PageInfoSegments>> {
    let current_page = std::cmp::max(current_page, 1);
    let records_limit = context.config.gateway.records_limit;
    let items_count = std::cmp::max(1, std::cmp::min(items_count, records_limit as i32));
    let skip = items_count * (current_page - 1);
    let search_params = convert_search_term(context, search_params)?;
    let orders = context.get_billing_microservice().orders(skip, items_count, search_params)?;
    let total_pages = std::cmp::max(0, orders.total_count as i32 - 1) / items_count + 1;
    let orders_edges: Vec<Edge<OrderBilling>> = orders
        .orders
        .into_iter()
        .map(|order| Edge::new(GraphqlID::from(order.id.0.to_string()), order))
        .collect();
    let page_info = PageInfoSegments {
        current_page,
        page_items_count: items_count,
        total_pages,
    };
    Ok(Connection::new(orders_edges, page_info))
}

pub fn get_store_id_by_product(context: &Context, product_id: ProductId) -> FieldResult<StoreId> {
    let url_store_id = format!(
        "{}/{}/store_id?product_id={}",
        context.config.service_url(Service::Stores),
        Model::Product.to_url(),
        product_id
    );

    context
        .request::<Option<StoreId>>(Method::Get, url_store_id, None)
        .wait()
        .and_then(|id| {
            if let Some(id) = id {
                Ok(id)
            } else {
                Err(FieldError::new(
                    "Could not find store_id from product id.",
                    graphql_value!({ "code": 100, "details": { "Product with such id does not exist in stores microservice." }}),
                ))
            }
        })
}

pub fn try_get_store(context: &Context, store_id: StoreId, visibility: Visibility) -> FieldResult<Option<Store>> {
    let url_store = format!(
        "{}/{}/{}?visibility={}",
        context.config.service_url(Service::Stores),
        Model::Store.to_url(),
        store_id,
        visibility
    );

    context.request::<Option<Store>>(Method::Get, url_store, None).wait()
}

pub fn get_store(context: &Context, store_id: StoreId, visibility: Visibility) -> FieldResult<Store> {
    try_get_store(context, store_id, visibility).and_then(|store| {
        if let Some(store) = store {
            Ok(store)
        } else {
            Err(FieldError::new(
                "Could not find store from store id.",
                graphql_value!({ "code": 100, "details": { "Store with such id does not exist in stores microservice." }}),
            ))
        }
    })
}

pub fn run_send_to_moderation_store(context: &Context, store_id: StoreId) -> FieldResult<Store> {
    let payload = StoreModerate {
        store_id,
        status: ModerationStatus::Moderation,
    };

    if validate_change_moderation_status(context, &payload)? {
        send_to_moderation(context, store_id)
    } else {
        Err(FieldError::new(
            "Could not change store status.",
            graphql_value!({ "code": 100, "details": { "Store cannot be sent to moderation." }}),
        ))
    }
}

fn send_to_moderation(context: &Context, store_id: StoreId) -> FieldResult<Store> {
    let url = format!(
        "{}/{}/{}/moderation",
        context.config.saga_microservice.url.clone(),
        Model::Store.to_url(),
        store_id
    );

    context.request::<Store>(Method::Post, url, None).wait()
}

pub fn run_moderation_status_store(context: &Context, input: StoreModerateInput) -> FieldResult<Store> {
    let identifier = ID::from_str(&*input.id)?;
    let store_id = StoreId(identifier.raw_id);

    let payload = StoreModerate {
        store_id,
        status: input.status,
    };

    if validate_change_moderation_status(context, &payload)? {
        send_to_moderate(context, payload)
    } else {
        Err(FieldError::new(
            "Could not change store status.",
            graphql_value!({ "code": 100, "details": { "Store status cannot be changed." }}),
        ))
    }
}

fn validate_change_moderation_status(context: &Context, payload: &StoreModerate) -> FieldResult<bool> {
    let url = format!(
        "{}/{}/validate_change_moderation_status",
        context.config.service_url(Service::Stores),
        Model::Store.to_url()
    );

    let body: String = serde_json::to_string(&payload)?.to_string();

    context.request::<bool>(Method::Post, url, Some(body)).wait()
}

fn send_to_moderate(context: &Context, payload: StoreModerate) -> FieldResult<Store> {
    let url = format!(
        "{}/{}/moderate",
        context.config.saga_microservice.url.clone(),
        Model::Store.to_url()
    );

    let body: String = serde_json::to_string(&payload)?.to_string();

    context.request::<Store>(Method::Post, url, Some(body)).wait()
}

pub fn run_send_to_draft_store_mutation(context: &Context, store_id: StoreId) -> FieldResult<Store> {
    let url = format!(
        "{}/{}/{}/draft",
        context.config.service_url(Service::Stores),
        Model::Store.to_url(),
        store_id
    );

    context.request::<Store>(Method::Post, url, None).wait()
}

pub fn run_update_store_mutation(context: &Context, input: UpdateStoreInput) -> FieldResult<Store> {
    let identifier = ID::from_str(&*input.id)?;
    let store_id = StoreId(identifier.raw_id);

    let url = identifier.url(&context.config);

    if input.is_none() {
        return Err(FieldError::new(
            "Nothing to update",
            graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
        ));
    }

    if validate_update_store(context, store_id)? {
        let body: String = serde_json::to_string(&input)?.to_string();
        context.request::<Store>(Method::Put, url, Some(body)).wait()
    } else {
        let current_store = get_store(context, store_id, Visibility::Active)?;

        Err(FieldError::new(
            "Could not update store.",
            graphql_value!({ "code": 100, "details": { format!("Store with id: {} in status: {} cannot be changed.", current_store.id, current_store.status) }}),
        ))
    }
}

pub fn validate_update_store(context: &Context, store_id: StoreId) -> FieldResult<bool> {
    let url = format!(
        "{}/{}/{}/validate_update",
        context.config.service_url(Service::Stores),
        Model::Store.to_url(),
        store_id
    );

    context.request::<bool>(Method::Get, url, None).wait()
}
