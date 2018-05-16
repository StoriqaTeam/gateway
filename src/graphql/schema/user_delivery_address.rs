//! File containing product object of graphql schema
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(UserDeliveryAddress: Context as "UserDeliveryAddress" |&self| {
    description: "User delivery address."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Users, Model::UserDeliveryAddress, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field user_id() -> &i32 as "user id"{
        &self.user_id
    }

    field administrative_area_level_1() -> &Option<String> as "administrative_area_level_1" {
        &self.administrative_area_level_1
    }

    field administrative_area_level_2() -> &Option<String> as "administrative_area_level_2" {
        &self.administrative_area_level_2
    }

    field country() -> &str as "Country" {
        &self.country
    }

    field locality() -> &Option<String> as "Locality" {
        &self.locality
    }

    field political() -> &Option<String> as "political" {
        &self.political
    }

    field postal_code() -> &str as "postal_code" {
        &self.postal_code
    }

    field route() -> &Option<String> as "route" {
        &self.route
    }

    field street_number() -> &Option<String> as "street_number" {
        &self.street_number
    }

    field address() -> &Option<String> as "address" {
        &self.address
    }

    field is_priority() -> &bool as "is_priority" {
        &self.is_priority
    }
});
