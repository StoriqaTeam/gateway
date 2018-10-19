//! File containing wizard store object of graphql schema
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Currency;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(AvailablePackages: Context as "AvailablePackages" |&self| {
    description: "Available Packages info."

    field company_package_id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Delivery, Model::CompanyPackage, self.id.0).to_string().into()
    }

    field company_package_raw_id() -> &i32 as "Int company package id"{
        &self.id.0
    }

    field name() -> &str as "Available package name"{
        &self.name
    }

    field logo() -> &str as "Company logo"{
        &self.logo
    }

    field deliveries_to() -> &[Country] as "Deliveries to Countries." {
        &self.deliveries_to
    }

    field currency() -> Currency as "Company currency" {
        self.currency
    }

});

graphql_object!(AvailablePackagesOutput: Context as "AvailablePackagesOutput" |&self| {
    description: "Available Packages info."

    field local() -> &[AvailablePackages] as "Local packages"{
        &self.local
    }

    field international() -> &[AvailablePackages] as "International packages"{
        &self.international
    }

});

graphql_object!(AvailablePackageForUser: Context as "AvailablePackageForUser" |&self| {
    description: "Available Packages info."

    field company_package_id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Delivery, Model::CompanyPackage, self.id.0).to_string().into()
    }

    field company_package_raw_id() -> &i32 as "Int company package id"{
        &self.id.0
    }

    field name() -> &str as "Available package name"{
        &self.name
    }

    field logo() -> &str as "Company logo"{
        &self.logo
    }

    field price() -> Option<f64> as "Package price." {
        self.price.clone().map(|p| p.0)
    }

});

graphql_object!(AvailableShippingForUser: Context as "AvailableShippingForUser" |&self| {
    description: "Available Packages info."

    field packages() -> &[AvailablePackageForUser] as "Available Packages For Users"{
        &self.packages
    }

    field pickups() -> &Option<PickupsOutput> as "Pickups"{
        &self.pickups
    }

});
