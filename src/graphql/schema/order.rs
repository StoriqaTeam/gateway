//! File containing PageInfo object of graphql schema
use std::cmp;
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Order: Context as "Order" |&self| {
    description: "Order info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        self.id.clone().into()
    }

    field status() -> &OrderStatus as "Order Status"{
        &self.status
    }

    field customer_id() -> &i32 as "Customer int id"{
        &self.customer_id
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
        &self.store_id
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
        self.price * (self.quantity as f64)
    }

    field slug() -> &i32 as "Slug" {
        &self.slug
    }

    field payment_status() -> &bool as "Payment status" {
        &self.payment_status
    }

    field delivery_company() -> &str as "Delivery Company" {
        &self.delivery_company
    }

    field track_id() -> &Option<String> as "Delivery Company" {
        &self.track_id
    }

    field creation_time() -> &str as "Creation time" {
        &self.creation_time
    }

    field receiver_name() -> &str as "Receiver name" {
        &self.receiver_name
    }

    field address_full() -> Address as "Full address" {
        self.clone().into()
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

        let url = format!("{}/{}/history?offset={}&count={}",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            offset,
            count + 1
            );

        context.request::<Vec<OrderHistoryItem>>(Method::Post, url, None)
            .map (|items| {
                let mut item_edges: Vec<Edge<OrderHistoryItem>> =  vec![];
                for i in 0..items.len() {
                    let edge = Edge::new(
                            juniper::ID::from( (i as i32 + offset).to_string()),
                            items[i].clone()
                        );
                    item_edges.push(edge);
                }
                let has_next_page = item_edges.len() as i32 == count + 1;
                if has_next_page {
                    item_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  item_edges.iter().nth(0).map(|e| e.cursor.clone());
                let end_cursor = item_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(item_edges, page_info)
            })
            .wait()
            .map(|u| Some(u))
    }

    field allowed_statuses(&executor) -> FieldResult<Vec<OrderStatus>> as "Allowed statuses" {
        let context = executor.context();
        let url = format!("{}/{}/{}/allowed_statuses",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            self.id);

        context.request::<Vec<OrderStatus>>(Method::Get, url, None)
            .wait()
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
