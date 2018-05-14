//! File containing wizard store object of graphql schema
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(ModeratorProductComments: Context as "ModeratorProductComments" |&self| {
    description: "Moderator Product Comments info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::ModeratorProductComment, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field moderator_id() -> &i32 as "Moderator raw id" {
        &self.moderator_id
    }

    field base_product_id() -> &i32 as "Base product raw id" {
        &self.base_product_id
    }

    field comments() -> &str as "Comments" {
        &self.comments
    }

});

graphql_object!(ModeratorStoreComments: Context as "ModeratorStoreComments" |&self| {
    description: "Moderator Store Comments info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::ModeratorStoreComment, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field moderator_id() -> &i32 as "Moderator raw id" {
        &self.moderator_id
    }

    field store_id() -> &i32 as "Store raw id" {
        &self.store_id
    }

    field comments() -> &str as "Comments" {
        &self.comments
    }

});
