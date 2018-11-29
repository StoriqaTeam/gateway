//! File containing Category object of graphql schema
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(ShippingOutput: Context as "ShippingOutput" |&self| {
    description: "Shipping Output info."

    field local() -> &[LocalShippingProducts] as "local shipping" {
        &self.local
    }

    field international() -> &[InternationalShippingProducts] as "international shipping" {
        &self.international
    }

    field pickup() -> &Option<PickupsOutput> as "pickups"{
        &self.pickup
    }

});

graphql_object!(LocalShippingProducts: Context as "LocalShippingProducts" |&self| {
    description: "Local Shipping Products info."

    field company_package_id() -> GraphqlID as "company package id"{
        ID::new(Service::Delivery, Model::CompanyPackage, self.company_package_id.0).to_string().into()
    }

    field price() -> &Option<f64> as "price" {
        &self.price
    }

    field is_fixed_price() -> bool as "is fixed price" {
        self.price.is_some()
    }


    field deliveries_to() -> &[Country] as "deliveries to" {
        &self.deliveries_to
    }
});

graphql_object!(InternationalShippingProducts: Context as "InternationalShippingProducts" |&self| {
    description: "International Shipping Products info."

    field company_package_id() -> GraphqlID as "company package id"{
        ID::new(Service::Delivery, Model::CompanyPackage, self.company_package_id.0).to_string().into()
    }

    field price() -> &Option<f64> as "price"{
        &self.price
    }

    field deliveries_to() -> &[Country] as "deliveries to" {
        &self.deliveries_to
    }

});

graphql_object!(PickupsOutput: Context as "PickupsOutput" |&self| {
    description: "Pickups info."

    field price() -> &Option<f64> as "price"{
        &self.price
    }

    field pickup() -> &bool as "pickup" {
        &self.pickup
    }

});
