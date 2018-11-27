//! File containing Category object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
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

    field raw_id() -> &i32 as "Int id"{
        &self.id.0
    }

    field name() -> &str as "Name"{
        &self.name
    }

    field max_size() -> &i32 as "max volume (cm^3)"{
        &self.max_size
    }

    field min_size() -> &i32 as "min volume (cm^3)"{
        &self.min_size
    }

    field max_weight() -> &i32 as "max weight (g)"{
        &self.max_weight
    }

    field min_weight() -> &i32 as "min weight (g)"{
        &self.min_weight
    }

    field deliveries_to() -> &[Country] as "deliveries_to" {
        &self.deliveries_to
    }

    field companies(&executor) -> FieldResult<Vec<Company>> as "Fetches companies by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}/{}",
            &context.config.service_url(Service::Delivery),
            Model::Package.to_url(),
            self.id,
            Model::Company.to_url(),
        );

        context.request::<Vec<Company>>(Method::Get, url, None)
            .wait()
    }

});
