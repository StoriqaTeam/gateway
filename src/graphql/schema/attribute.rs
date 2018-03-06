//! File containing product object of graphql schema
use juniper::ID as GraphqlID;
use stq_static_resources::Translation;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(Attribute: Context as "Attribute" |&self| {
    description: "Attribute's info."

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field name() -> Vec<Translation> as "Full Name" {
        self.name.clone()
    }

    field meta_field() -> Option<String> as "Meta field of product" {
        self.meta_field.clone()
    }
});
