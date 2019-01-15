use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use serde_json;

use stq_routes::model::Model;
use stq_types::{BaseProductId, OrderSlug};

use stq_api::orders::Order;
use stq_static_resources::OrderState;

use graphql::context::Context;
use graphql::models::*;

pub trait SagaService {
    fn upsert_shipping(&self, base_product_id: BaseProductId, shipping: NewShipping) -> FieldResult<Shipping>;

    fn set_order_state(&self, order_slug: OrderSlug, state: OrderState) -> FieldResult<Option<GraphQLOrder>>;
}

pub struct SagaServiceImpl<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> SagaServiceImpl<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        SagaServiceImpl { context }
    }

    fn base_url(&self) -> String {
        self.context.config.saga_microservice.url.clone()
    }

    fn request_url(&self, request: &str) -> String {
        format!("{}/{}", self.base_url(), request)
    }
}

impl<'ctx> SagaService for SagaServiceImpl<'ctx> {
    fn upsert_shipping(&self, base_product_id: BaseProductId, shipping: NewShipping) -> FieldResult<Shipping> {
        let request_path = format!("{}/{}/upsert-shipping", Model::BaseProduct.to_url(), base_product_id);
        let url = self.request_url(&request_path);

        let body: String = serde_json::to_string(&shipping)?;
        self.context.request::<Shipping>(Method::Post, url, Some(body)).wait()
    }

    fn set_order_state(&self, order_slug: OrderSlug, state: OrderState) -> FieldResult<Option<GraphQLOrder>> {
        let request_path = format!("{}/{}/set_state", Model::Order.to_url(), order_slug);
        let url = self.request_url(&request_path);
        let body = serde_json::to_string(&state)?;

        self.context
            .request::<Option<Order>>(Method::Post, url, Some(body))
            .wait()
            .map(|order| order.map(GraphQLOrder))
    }
}
