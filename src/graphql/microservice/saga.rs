use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use serde_json;

use stq_routes::model::Model;
use stq_types::BaseProductId;

use graphql::context::Context;
use graphql::models::*;

pub trait SagaService {
    fn upsert_shipping(&self, context: &Context, base_product_id: BaseProductId, shipping: NewShipping) -> FieldResult<Shipping>;
}

pub struct SagaServiceImpl;

impl SagaServiceImpl {
    pub fn new() -> Self {
        SagaServiceImpl {}
    }
}

impl SagaService for SagaServiceImpl {
    fn upsert_shipping(&self, context: &Context, base_product_id: BaseProductId, shipping: NewShipping) -> FieldResult<Shipping> {
        let url = format!(
            "{}/{}/{}/upsert-shipping",
            context.config.saga_microservice.url,
            Model::BaseProduct.to_url(),
            base_product_id
        );

        let body: String = serde_json::to_string(&shipping)?;
        context.request::<Shipping>(Method::Post, url, Some(body)).wait()
    }
}
