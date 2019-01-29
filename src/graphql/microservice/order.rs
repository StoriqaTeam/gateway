use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::OrderId;

use graphql::context::Context;
use graphql::models::*;

pub trait OrdersService {
    fn get_order_by_id(&self, order_id: OrderId) -> FieldResult<GraphQLOrder>;
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
    fn get_order_by_id(&self, order_id: OrderId) -> FieldResult<GraphQLOrder> {
        let request_path = format!("{}/by-id/{}", Model::Order.to_url(), order_id);
        let url = self.request_url(&request_path);
        let order = self.context.request(Method::Get, url, None).wait()?;
        Ok(GraphQLOrder(order))
    }
}
