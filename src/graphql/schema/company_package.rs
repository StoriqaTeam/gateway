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

graphql_object!(CompaniesPackages: Context as "CompaniesPackages" |&self| {
    description: "Companies Packages info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Delivery, Model::CompanyPackage, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Int id"{
        &self.id.0
    }

    field company_id() -> &i32 as "company_id"{
        &self.company_id.0
    }

    field company(&executor) -> FieldResult<Option<Company>> as "Fetches company." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Delivery),
            Model::Company.to_url(),
            self.company_id
        );

        context.request::<Option<Company>>(Method::Get, url, None)
            .wait()
    }

    field package_id() -> &i32 as "package_id"{
        &self.package_id.0
    }

    field package(&executor) -> FieldResult<Option<Packages>> as "Fetches package." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Delivery),
            Model::Package.to_url(),
            self.package_id
        );

        context.request::<Option<Packages>>(Method::Get, url, None)
            .wait()
    }

});
