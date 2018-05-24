//! File containing wizard store object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Warehouse: Context as "Warehouse" |&self| {
    description: "Warehouse info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Warehouses, Model::Warehouse, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field name() -> &Option<String> as "Name"{
        &self.name
    }
    
    field location() -> &Option<GeoPoint> as "Location"{
        &self.location
    }

    field admins() -> &[i32] as "admins"{
        &self.admins
    }

    field managers() -> &[i32] as "managers"{
        &self.managers
    }

    field kind() -> &WarehouseKind as "Warehouse Kind"{
        &self.kind
    }

    field address_full() -> Address as "Address full"{
        self.clone().into()
    }

    field products(&executor) -> FieldResult<Option<Vec<WarehouseProduct>>> as "Fetches all products of warehouse." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Warehouses),
            Model::Warehouse.to_url(),
            self.id.to_string()
        );

        context.request::<Vec<WarehouseProduct>>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }

});
