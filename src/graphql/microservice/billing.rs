use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::InvoiceId;

use graphql::context::Context;
use graphql::microservice::requests::*;
use graphql::microservice::response::*;

pub trait BillingService {
    fn payment_intent_by_invoice(&self, invoice_id: InvoiceId) -> FieldResult<Option<PaymentIntent>>;

    fn create_customer_with_source(&self, input: NewCustomerWithSourceRequest) -> FieldResult<()>;
}

pub struct BillingServiceImpl<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> BillingServiceImpl<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        BillingServiceImpl { context }
    }

    fn base_url(&self) -> String {
        self.context.config.service_url(Service::Billing)
    }

    fn request_url(&self, request: &str) -> String {
        format!("{}/{}", self.base_url(), request)
    }
}

impl<'ctx> BillingService for BillingServiceImpl<'ctx> {
    fn payment_intent_by_invoice(&self, invoice_id: InvoiceId) -> FieldResult<Option<PaymentIntent>> {
        let request_path = format!("{}/{}/{}", Model::PaymentIntent.to_url(), Model::Invoice.to_url(), invoice_id);
        let url = self.request_url(&request_path);

        self.context.request::<Option<PaymentIntent>>(Method::Get, url, None).wait()
    }

    fn create_customer_with_source(&self, input: NewCustomerWithSourceRequest) -> FieldResult<()> {
        let request_path = format!("{}/with_source", Model::Customer.to_url());
        let url = self.request_url(&request_path);

        let body: String = serde_json::to_string(&input)?;
        self.context.request::<()>(Method::Post, url, Some(body)).wait()
    }
}
