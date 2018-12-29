use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use serde_json;

use stq_routes::model::Model;
use stq_types::BaseProductId;

use graphql::context::Context;
use graphql::models::*;

pub trait SagaService {
    fn upsert_shipping(&self, base_product_id: BaseProductId, shipping: NewShipping) -> FieldResult<Shipping>;
}

pub struct SagaServiceImpl<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> SagaServiceImpl<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        SagaServiceImpl { context }
    }
}

impl<'ctx> SagaService for SagaServiceImpl<'ctx> {
    fn upsert_shipping(&self, base_product_id: BaseProductId, shipping: NewShipping) -> FieldResult<Shipping> {
        let url = format!(
            "{}/{}/{}/upsert-shipping",
            self.context.config.saga_microservice.url,
            Model::BaseProduct.to_url(),
            base_product_id
        );

        let body: String = serde_json::to_string(&shipping)?;
        self.context.request::<Shipping>(Method::Post, url, Some(body)).wait()
    }
}
