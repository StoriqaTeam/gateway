//! File containing PageInfo object of graphql schema
use std::cmp;
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};
use serde_json;

use stq_api::orders::{CartClient, OrderClient};
use stq_api::types::ApiFutureExt;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::{Currency, OrderState};

use super::*;
use errors::into_graphql;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::coupon::try_get_coupon;

graphql_object!(GraphQLOrder: Context as "Order" |&self| {
    description: "Order info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        self.0.id.to_string().into()
    }

    field state() -> &OrderState as "Order State"{
        &self.0.state
    }

    field customer_id() -> &i32 as "Customer int id"{
        &self.0.customer.0
    }

    field customer(&executor) -> FieldResult<Option<User>> as "Customer" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            self.0.customer);

        context.request::<Option<User>>(Method::Get, url, None)
            .wait()
    }

    field product_id() -> &i32 as "Product int id"{
        &self.0.product.0
    }

    field product(&executor) -> FieldResult<Option<Product>> as "Product" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.0.product);

        context.request::<Option<Product>>(Method::Get, url, None)
            .wait()
    }

    field store_id() -> &i32 as "Store int id"{
        &self.0.store.0
    }

    field store(&executor) -> FieldResult<Option<Store>> as "Store" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.0.store);

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field quantity() -> &i32 as "Quantity" {
        &self.0.quantity.0
    }

    field price() -> &f64 as "Price" {
        &self.0.price.0
    }

    field currency() -> &Currency as "Currency" {
        &self.0.currency
    }

    field subtotal() -> f64 as "Subtotal" {
        self.0.price.0 * f64::from(self.0.quantity.0)
    }

    field coupon(&executor) -> FieldResult<Option<Coupon>> as "Coupon added user" {
        let context = executor.context();

        if let Some(coupon_id) = self.0.coupon_id {
            try_get_coupon(context, coupon_id)
        } else {
            Ok(None)
        }
    }

    field coupon_percent() -> &Option<i32> as "Coupon percent" {
        &self.0.coupon_percent
    }

    field coupon_discount() -> Option<f64> as "Coupon discount" {
        self.0.coupon_discount.map(|c| c.0)
    }

    field product_discount() -> Option<f64> as "Product discount" {
        self.0.product_discount.map(|c| c.0)
    }

    field total_amount() -> f64 as "Total amount" {
        self.0.total_amount.0
    }

    field slug() -> &i32 as "Slug" {
        &self.0.slug.0
    }

    field payment_status() -> &bool as "Payment status" {
        &self.0.payment_status
    }

    field delivery_company() -> &Option<String> as "Delivery Company" {
        &self.0.delivery_company
    }

    field track_id() -> &Option<String> as "Delivery Company" {
        &self.0.track_id
    }

    field created_at() -> String as "Creation time" {
        self.0.created_at.to_rfc3339()
    }

    field receiver_name() -> &str as "Receiver name" {
        &self.0.receiver_name
    }

    field receiver_phone() -> &str as "Receiver phone" {
        &self.0.receiver_phone
    }

    field reveiver_email() -> &str as "Receiver email" {
        &self.0.receiver_email
    }

    field address_full() -> Address as "Full address" {
        self.0.address.clone().into()
    }

    field pre_order() -> &bool as "Pre order" {
        &self.0.pre_order
    }

    field pre_order_days() -> &i32 as "Pre order days" {
        &self.0.pre_order_days
    }

    field history(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset form beginning")
            -> FieldResult<Option<Connection<OrderHistoryItem, PageInfo>>> as "History" {

        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let rpc_client = context.get_rest_api_client(Service::Orders);
        rpc_client.get_order_diff(self.0.slug.into())
            .sync()
            .map_err(into_graphql)
            .map (|items| {
                let mut item_edges: Vec<Edge<OrderHistoryItem>> = items
                    .into_iter()
                    .skip(offset as usize)
                    .take(count as usize)
                    .enumerate()
                    .map(|(i, item)| Edge::new(juniper::ID::from((i as i32 + offset).to_string()), OrderHistoryItem(item)))
                    .collect();
                let has_next_page = item_edges.len() as i32 == count + 1;
                if has_next_page {
                    item_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  item_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = item_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(item_edges, page_info)
            })
            .map(Some)
    }

    field allowed_statuses(&executor) -> FieldResult<Vec<OrderState>> as "Allowed statuses" {
        let context = executor.context();
        let url = format!("{}/{}/{}/allowed_statuses",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            self.0.id);

        context.request::<Vec<OrderState>>(Method::Get, url, None)
            .wait()
    }

    field invoice(&executor) -> FieldResult<Option<Invoice>> as "Invoice" {
        let context = executor.context();
        let url = format!("{}/invoices/by-order-id/{}",
            context.config.service_url(Service::Billing),
            self.0.id);

        context.request::<Option<Invoice>>(Method::Get, url, None)
            .wait()
    }
});

graphql_object!(CreateOrdersOutput: Context as "CreateOrdersOutput" |&self| {
    description:"Create orders object"

    field invoice() -> &Invoice {
        &self.0
    }

    field cart(&executor) -> FieldResult<Option<Cart>> as "Fetches cart products." {
        let context = executor.context();

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let fut = if let Some(session_id) = context.session_id {
            if let Some(ref user) = context.user {
                rpc_client.merge(session_id.into(), user.user_id.into())
            } else {
                rpc_client.get_cart(session_id.into())
            }
        } else if let Some(ref user) = context.user {
            rpc_client.get_cart(user.user_id.into())
        }  else {
            return Err(FieldError::new(
                "Could not get users cart.",
                graphql_value!({ "code": 100, "details": { "No user id or session id in request header." }}),
            ));
        };

        let products: Vec<_> = fut
            .sync()
            .map_err(into_graphql)?.into_iter().collect();

        let url = format!("{}/{}/cart",
            context.config.service_url(Service::Stores),
            Model::Store.to_url());

        let body = serde_json::to_string(&products)?;

        context.request::<Vec<Store>>(Method::Post, url, Some(body))
            .map(|stores| convert_to_cart(stores, &products))
            .map(Some)
            .wait()
    }

});

graphql_object!(Connection<GraphQLOrder, PageInfo>: Context as "OrdersConnection" |&self| {
    description:"Order Connection"

    field edges() -> &[Edge<GraphQLOrder>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<GraphQLOrder>: Context as "OrdersEdge" |&self| {
    description:"Order Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &GraphQLOrder {
        &self.node
    }
});

graphql_object!(Connection<OrderHistoryItem, PageInfo>: Context as "OrderHistoryItemsConnection" |&self| {
    description:"Order History Item Connection"

    field edges() -> &[Edge<OrderHistoryItem>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<OrderHistoryItem>: Context as "OrderHistoryItemsEdge" |&self| {
    description:"Order History Item Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &OrderHistoryItem {
        &self.node
    }
});
