//! File containing Category object of graphql schema
use juniper::ID as GraphqlID;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(Category: Context as "Category" |&self| {
    description: "Category info."

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Category, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field name() -> Vec<Translation> as "Full Name" {
        self.name.clone()
    }

    field meta_field() -> Option<String> as "Meta field" {
        self.meta_field.clone()
    }

    field children() -> Vec<Category> as "Children categories" {
        self.children.clone()
    }
});