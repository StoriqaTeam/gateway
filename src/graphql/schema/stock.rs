//! File containing wizard store object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_api::warehouses::Stock;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::{ProductId, WarehouseId, WarehouseIdentifier};

use super::*;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::warehouse as warehouse_module;

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

        warehouse_module::try_get_warehouse(context, WarehouseIdentifier::Id(self.0.warehouse_id))
    }

    field quantity() -> &i32 as "Quantity"{
        &self.0.quantity.0
    }

});

pub fn try_get_stock_for_warehouse(context: &Context, warehouse_id: WarehouseId, product_id: ProductId) -> FieldResult<Option<Stock>> {
    let url = format!(
        "{}/{}/by-id/{}/products/{}",
        context.config.service_url(Service::Warehouses),
        Model::Warehouse.to_url(),
        warehouse_id,
        product_id,
    );

    context.request::<Option<Stock>>(Method::Get, url, None).wait()
}

pub fn get_stocks_for_product(context: &Context, product_id: ProductId) -> FieldResult<Vec<Stock>> {
    let url = format!(
        "{}/{}/by-product-id/{}",
        context.config.service_url(Service::Warehouses),
        Model::Stock.to_url(),
        product_id,
    );

    context.request::<Vec<Stock>>(Method::Get, url, None).wait()
}
