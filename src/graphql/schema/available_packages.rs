//! File containing wizard store object of graphql schema
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(AvailablePackages: Context as "Available Packages" |&self| {
    description: "Available Packages info."

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Delivery, Model::CompanyPackage, self.id.0).to_string().into()
    }

    field name() -> &str as "Product id"{
        &self.name
    }

    field deliveries_to() -> &[Country] as "Countries." {
        &self.deliveries_to
    }

});

graphql_object!(AvailablePackagesOutput: Context as "Available Packages Output " |&self| {
    description: "Available Packages info."

    field local() -> &[AvailablePackages] as "Local packages"{
        &self.local
    }
    
    field international() -> &[AvailablePackages] as "International packages"{
        &self.international
    }

});
