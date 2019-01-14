use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::InvoiceId;

use graphql::context::Context;
use graphql::models::*;

pub trait BillingService {
    fn payment_intent_by_invoice(&self, invoice_id: InvoiceId) -> FieldResult<Option<PaymentIntent>>;
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
}

impl<'ctx> BillingService for BillingServiceImpl<'ctx> {
    fn payment_intent_by_invoice(&self, invoice_id: InvoiceId) -> FieldResult<Option<PaymentIntent>> {
        let url = format!(
            "{}/{}/{}/{}",
            self.base_url(),
            Model::PaymentIntent.to_url(),
            Model::Invoice.to_url(),
            invoice_id
        );

        self.context.request::<Option<PaymentIntent>>(Method::Get, url, None).wait()
    }
}
