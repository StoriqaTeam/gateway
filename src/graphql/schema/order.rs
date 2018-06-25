//! File containing PageInfo object of graphql schema
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

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Orders, Model::Order, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
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

    field subtotal() -> &f64 as "Subtotal" {
        &self.subtotal
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

    field delivery_track_id() -> &Option<String> as "Delivery Company" {
        &self.delivery_track_id
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

    field history(&executor) -> FieldResult<Vec<OrderHistoryItem>> as "History" {
        let context = executor.context();
        let url = format!("{}/{}/{}/history",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            self.id);

        context.request::<Vec<OrderHistoryItem>>(Method::Get, url, None)
            .wait()
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
