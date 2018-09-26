//! File containing PageInfo object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(CartProduct: Context as "CartProduct" |&self| {
    description: "Cart Product info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Orders, Model::CartProduct, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field quantity() -> &i32 as "Quantity" {
        &self.quantity.0
    }

    field price() -> &f64 as "Price" {
        &self.price.0
    }

    field subtotal() -> f64 as "Subtotal" {
        self.price.0 * f64::from(self.quantity.0)
    }

    field delivery_cost() -> f64 as "Delivery cost" {
        0.0
    }

    field photo_main() -> &Option<String> as "Photo main" {
        &self.photo_main
    }

    field comment() -> &str as "Comment" {
        &self.comment
    }

    field selected() -> &bool as "Selected" {
        &self.selected
    }

    field delivery_operator() -> &str as "Delivery Operator" {
        "Operator"
    }

    field delivery_period() -> &str as "Delivery Period" {
        "14 days"
    }

    field delivery_return_type() -> &str as "Delivery return type" {
        "funds return"
    }

    field delivery_return_paid_by() -> &str as "Delivery return paid by" {
        "Seller"
    }

    field pre_order() -> &bool as "Pre order" {
        &self.pre_order
    }

    field pre_order_days() -> &i32 as "Pre order days" {
        &self.pre_order_days
    }

    field attributes(&executor) -> FieldResult<Option<Vec<AttrValue>>> as "Variants" {
       let context = executor.context();
        let url = format!("{}/{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.id);

        context.request::<Vec<AttrValue>>(Method::Get, url, None)
            .wait()
            .or_else(|_| Ok(vec![]))
            .map(Some)
    }
});
