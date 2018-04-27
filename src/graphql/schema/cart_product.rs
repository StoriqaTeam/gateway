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
        ID::new(Service::Orders, Model::CartProduct, self.id).to_string().into()
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field quantity() -> &i32 as "Quantity" {
        &self.quantity
    }

    field price() -> &f64 as "Price" {
        &self.price
    }

    field delivery_price() -> f64 as "Delivery Price" {
        0.0
    }

    field photo_main() -> &Option<String> as "Photo main" {
        &self.photo_main
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

    field delivery_return_payd_by() -> &str as "Delivery return payd by" {
        "Seller"
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
            .map(|u| Some(u))
    }
});
