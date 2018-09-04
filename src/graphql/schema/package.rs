//! File containing Category object of graphql schema
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Packages: Context as "Packages" |&self| {
    description: "Packages info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Delivery, Model::Package, self.id.0).to_string().into()
    }

    field name() -> &str as "Name"{
        &self.name
    }

    field max_size() -> &f64 as "max_size"{
        &self.max_size
    }

    field min_size() -> &f64 as "min_size"{
        &self.min_size
    }

    field max_weight() -> &f64 as "max_weight"{
        &self.max_weight
    }

    field min_weight() -> &f64 as "min_weight"{
        &self.min_weight
    }

    field deliveries_to() -> Vec<String> as "deliveries_to" {
        self.deliveries_to.clone().into_iter().map(|d| d.0).collect()
    }

});
