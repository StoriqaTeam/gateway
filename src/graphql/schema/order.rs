//! File containing PageInfo object of graphql schema
use std::cmp;
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::OrderState;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Order: Context as "Order" |&self| {
    description: "Order info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        self.id.clone().into()
    }

    field state() -> &OrderState as "Order State"{
        &self.state
    }

    field customer_id() -> &i32 as "Customer int id"{
        &self.customer_id.0
    }

    field customer(&executor) -> FieldResult<Option<User>> as "Customer" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            self.customer_id);

        context.request::<Option<User>>(Method::Get, url, None)
            .wait()
    }

    field product_id() -> &i32 as "Product int id"{
        &self.product_id
    }

    field product(&executor) -> FieldResult<Option<Product>> as "Product" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.product_id);

        context.request::<Option<Product>>(Method::Get, url, None)
            .wait()
    }

    field store_id() -> &i32 as "Store int id"{
        &self.store_id.0
    }

    field store(&executor) -> FieldResult<Option<Store>> as "Store" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.store_id);

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field quantity() -> &i32 as "Quantity" {
        &self.quantity
    }

    field price() -> &f64 as "Price" {
        &self.price
    }

    field subtotal() -> f64 as "Subtotal" {
        self.price * f64::from(self.quantity)
    }

    field slug() -> &i32 as "Slug" {
        &self.slug
    }

    field payment_status() -> &bool as "Payment status" {
        &self.payment_status
    }

    field delivery_company() -> &Option<String> as "Delivery Company" {
        &self.delivery_company
    }

    field track_id() -> &Option<String> as "Delivery Company" {
        &self.track_id
    }

    field created_at() -> String as "Creation time" {
        self.created_at.to_rfc3339()
    }

    field receiver_name() -> &str as "Receiver name" {
        &self.receiver_name
    }

    field address_full() -> &Address as "Full address" {
        &self.address
    }

    field history(&executor,
        first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID>  as "Offset form begining") 
            -> FieldResult<Option<Connection<OrderHistoryItem, PageInfo>>> as "History" {

        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/order_diff/by-slug/{}",
            context.config.service_url(Service::Orders),
            self.slug
            );

        context.request::<Vec<OrderHistoryItem>>(Method::Get, url, None)
            .map (|items| {
                let mut item_edges: Vec<Edge<OrderHistoryItem>> = items
                    .into_iter()
                    .skip(offset as usize)
                    .take(count as usize)
                    .enumerate()
                    .map(|(i, item)| Edge::new(juniper::ID::from((i as i32 + offset).to_string()), item))
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
            .wait()
            .map(Some)
    }

    field allowed_statuses(&executor) -> FieldResult<Vec<OrderState>> as "Allowed statuses" {
        let context = executor.context();
        let url = format!("{}/{}/{}/allowed_statuses",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            self.id);

        context.request::<Vec<OrderState>>(Method::Get, url, None)
            .wait()
    }

    field invoice(&executor) -> FieldResult<Invoice> as "Invoice" {
        let context = executor.context();
        let url = format!("{}/invoices/by-order-id/{}",
            context.config.service_url(Service::Billing),
            self.id);

        context.request::<Invoice>(Method::Get, url, None)
            .wait()
    }
});

graphql_object!(CreateOrders: Context as "CreateOrders" |&self| {
    description:"Create orders object"

    field invoice() -> &Invoice {
        &self.invoice
    }

    field cart() -> &Cart {
        &self.cart
    }
});

graphql_object!(Connection<Order, PageInfo>: Context as "OrdersConnection" |&self| {
    description:"Order Connection"

    field edges() -> &[Edge<Order>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<Order>: Context as "OrdersEdge" |&self| {
    description:"Order Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &Order {
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
