//! File containing wizard store object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_api::warehouses::WarehouseClient;
use stq_api::types::ApiFutureExt;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::WarehouseIdentifier;

use errors::into_graphql;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(GraphQLStock: Context as "Stock" |&self| {
    description: "Warehouse Product info."

    interfaces: [&Node]

    field id(&executor) -> GraphqlID as "Base64 Unique id"{
        let context = executor.context();

        let id = format!("{}{}", self.0.warehouse_id, self.0.product_id);

        id.into()
    }

    field product_id() -> &i32 as "Product id"{
        &self.0.product_id.0
    }

    field product(&executor) -> FieldResult<Option<Product>> as "Fetches product." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.0.product_id.to_string()
        );

        context.request::<Option<Product>>(Method::Get, url, None)
            .wait()
    }

    field warehouse_id() -> String as "Warehouse id"{
        self.0.warehouse_id.to_string()
    }

    field warehouse(&executor) -> FieldResult<Option<GraphQLWarehouse>> as "Fetches warehouse." {
        let context = executor.context();

        let rpc_client = context.get_rest_api_client(Service::Warehouses);
        rpc_client.get_warehouse(WarehouseIdentifier::Id(self.0.warehouse_id))
            .sync()
            .map_err(into_graphql)
            .map(|res| res.map(GraphQLWarehouse))
    }

    field quantity() -> &i32 as "Quantity"{
        &self.0.quantity.0
    }

});
