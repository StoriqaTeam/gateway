//! File containing Category object of graphql schema
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Company: Context as "Company" |&self| {
    description: "Company info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Delivery, Model::Company, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Int id"{
        &self.id.0
    }

    field name() -> &str as "Name"{
        &self.name
    }

    field label() -> &str as "label"{
        &self.label
    }

    field description() -> Option<String> as "description"{
        self.description.clone()
    }

    field logo() -> &str as "logo" {
        &self.logo
    }

    field deliveries_from() -> Vec<String> as "deliveries_from" {
        self.deliveries_from.clone().into_iter().map(|d| d.0).collect()
    }

});
