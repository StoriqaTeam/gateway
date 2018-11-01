//! File containing Category object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Currency;

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

    field currency() -> Currency as "currency" {
        self.currency
    }

    field logo() -> &str as "logo" {
        &self.logo
    }

    field deliveries_from() -> &[Country] as "deliveries_from" {
        &self.deliveries_from
    }

    field packages(&executor) -> FieldResult<Vec<Packages>> as "Fetches packages by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}/{}",
            &context.config.service_url(Service::Delivery),
            Model::Company.to_url(),
            self.id,
            Model::Package.to_url(),
        );

        context.request::<Vec<Packages>>(Method::Get, url, None)
            .wait()
    }
});
