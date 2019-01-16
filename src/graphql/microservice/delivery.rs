use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

pub trait DeliveryService {
    fn update_user_delivery_address(&self, input: UpdateUserDeliveryAddressFullInput) -> FieldResult<UserDeliveryAddress>;
}

pub struct DeliveryServiceImpl<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> DeliveryServiceImpl<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        DeliveryServiceImpl { context }
    }

    fn base_url(&self) -> String {
        self.context.config.service_url(Service::Delivery)
    }

    fn request_url(&self, request: &str) -> String {
        format!("{}/{}", self.base_url(), request)
    }
}

impl<'ctx> DeliveryService for DeliveryServiceImpl<'ctx> {
    fn update_user_delivery_address(&self, input: UpdateUserDeliveryAddressFullInput) -> FieldResult<UserDeliveryAddress> {
        let request_path = format!("{}/addresses/{}", Model::User.to_url(), input.id);
        let url = self.request_url(&request_path);

        let body: String = serde_json::to_string(&input)?.to_string();
        self.context.request::<UserDeliveryAddress>(Method::Put, url, Some(body)).wait()
    }
}
