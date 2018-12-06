use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(ShippingRates: Context as "ShippingRates" |&self| {
    description: "Static shipping rates."

    field id() -> GraphqlID as "Base64 Unique id" {
        ID::new(Service::Delivery, Model::ShippingRates, self.id).to_string().into()
    }

    field raw_id() -> i32 as "Int id" {
        self.id
    }

    field company_package_id() -> i32 as "Company package int id" {
        self.company_package_id
    }

    field from() -> &str as "Country from" {
        &self.from_alpha3
    }

    field to() -> &str as "Country to" {
        &self.to_alpha3
    }

    field rates() -> &[ShippingRate] as "Shipping rates for this from-to country pair" {
        self.rates.as_slice()
    }
});

graphql_object!(ShippingRate: Context as "ShippingRate" |&self| {
    description: "Single shipping rate."

    field weight_g() -> i32 as "Weight (g)" {
        self.weight_g
    }

    field price() -> f64 as "Price" {
        self.price
    }
});
