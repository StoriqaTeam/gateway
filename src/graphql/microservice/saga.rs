use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use serde_json;

use stq_routes::model::Model;
use stq_types::BaseProductId;

use stq_api::orders::Order;

use graphql::context::Context;
use graphql::models::*;

pub trait SagaService {
    fn upsert_shipping(&self, base_product_id: BaseProductId, shipping: NewShipping) -> FieldResult<Shipping>;

    fn set_order_confirmed(&self, input: OrderConfirmed) -> FieldResult<Option<GraphQLOrder>>;

    fn create_orders(&self, input: CreateOrder) -> FieldResult<CreateOrdersOutput>;

    fn set_order_payment_state(&self, input: OrderPaymentState) -> FieldResult<()>;

    fn buy_now(&self, input: BuyNow) -> FieldResult<CreateOrdersOutput>;
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

    fn set_order_confirmed(&self, input: OrderConfirmed) -> FieldResult<Option<GraphQLOrder>> {
        let request_path = format!("{}/{}/set_state", Model::Order.to_url(), input.order_slug);
        let url = self.request_url(&request_path);
        let body = serde_json::to_string(&input)?;

        self.context
            .request::<Option<Order>>(Method::Post, url, Some(body))
            .wait()
            .map(|order| order.map(GraphQLOrder))
    }
    fn create_orders(&self, input: CreateOrder) -> FieldResult<CreateOrdersOutput> {
        let request_path = "create_order";
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?.to_string();
        self.context
            .request::<Invoice>(Method::Post, url, Some(body))
            .wait()
            .map(CreateOrdersOutput)
    }

    fn buy_now(&self, input: BuyNow) -> FieldResult<CreateOrdersOutput> {
        let request_path = "buy_now";
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?.to_string();
        self.context
            .request::<Invoice>(Method::Post, url, Some(body))
            .wait()
            .map(CreateOrdersOutput)
    }

    fn set_order_payment_state(&self, input: OrderPaymentState) -> FieldResult<()> {
        let request_path = format!("{}/{}/set_payment_state", Model::Order.to_url(), input.order_slug);
        let url = self.request_url(&request_path);
        let body = serde_json::to_string(&input)?;

        self.context.request::<()>(Method::Post, url, Some(body)).wait()
    }
}
