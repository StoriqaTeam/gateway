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
        &self.user_id.0
    }

    field address() -> Address as "address" {
        self.clone().into()
    }

    field is_priority() -> &bool as "is_priority" {
        &self.is_priority
    }
});
