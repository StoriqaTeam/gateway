//! File containing Category object of graphql schema
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

    field company_id() -> &i32 as "company_id"{
        &self.company_id.0
    }

    field package_id() -> &i32 as "package_id"{
        &self.package_id.0
    }

});
