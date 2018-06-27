//! File containing wizard store object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(Stock: Context as "Stock" |&self| {
    description: "Warehouse Product info."

    field product_id() -> &i32 as "Product id"{
        &self.product_id
    }

    field product(&executor) -> FieldResult<Option<Product>> as "Fetches product." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.product_id.to_string()
        );

        context.request::<Option<Product>>(Method::Get, url, None)
            .wait()
    }

    field warehouse_id() -> &str as "Warehouse id"{
        &self.warehouse_id
    }

    field warehouse(&executor) -> FieldResult<Option<Warehouse>> as "Fetches warehouse." {
        let context = executor.context();

        let url = format!(
            "{}/{}/by-id/{}",
            &context.config.service_url(Service::Warehouses),
            Model::Warehouse.to_url(),
            self.warehouse_id.to_string()
        );

        context.request::<Option<Warehouse>>(Method::Get, url, None)
            .wait()
    }

    field quantity() -> &i32 as "Quantity"{
        &self.quantity
    }

});
