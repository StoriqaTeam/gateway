use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_api::orders::Order;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::{OrderId, OrderSlug};

use graphql::context::Context;
use graphql::models::*;

pub trait OrdersService {
    fn get_order_by_id(&self, order_id: OrderId) -> FieldResult<Option<GraphQLOrder>>;

    fn get_order_by_slug(&self, order_slug: OrderSlug) -> FieldResult<Option<GraphQLOrder>>;
}

pub struct OrdersServiceImpl<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> OrdersServiceImpl<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        OrdersServiceImpl { context }
    }

    fn base_url(&self) -> String {
        self.context.config.service_url(Service::Orders)
    }

    fn request_url(&self, request: &str) -> String {
        format!("{}/{}", self.base_url(), request)
    }
}

impl<'ctx> OrdersService for OrdersServiceImpl<'ctx> {
    fn get_order_by_id(&self, order_id: OrderId) -> FieldResult<Option<GraphQLOrder>> {
        let request_path = format!("{}/by-id/{}", Model::Order.to_url(), order_id);
        let url = self.request_url(&request_path);
        let order: Option<Order> = self.context.request(Method::Get, url, None).wait()?;
        Ok(order.map(GraphQLOrder))
    }

    fn get_order_by_slug(&self, order_slug: OrderSlug) -> FieldResult<Option<GraphQLOrder>> {
        let request_path = format!("{}/by-slug/{}", Model::Order.to_url(), order_slug);
        let url = self.request_url(&request_path);
        let order: Option<Order> = self.context.request(Method::Get, url, None).wait()?;
        Ok(order.map(GraphQLOrder))
    }
}
