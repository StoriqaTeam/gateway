use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::InvoiceId;

use graphql::context::Context;
use graphql::microservice::requests::*;
use graphql::models::*;

pub trait BillingService {
    fn payment_intent_by_invoice(&self, invoice_id: InvoiceId) -> FieldResult<Option<PaymentIntent>>;

    fn create_customer_with_source(&self, input: NewCustomerWithSourceRequest) -> FieldResult<Customer>;

    fn get_current_customer(&self) -> FieldResult<Option<Customer>>;

    fn delete_customer(&self, payload: DeleteCustomerRequest) -> FieldResult<()>;

    fn add_role_to_user(&self, input: NewBillingRoleInput) -> FieldResult<NewRole<BillingMicroserviceRole>>;

    fn remove_role_from_user(&self, input: RemoveBillingRoleInput) -> FieldResult<NewRole<BillingMicroserviceRole>>;

    fn orders(&self, skip: i32, items_count: i32, input: OrderBillingSearchInput) -> FieldResult<OrderBillingSearchResults>;

    fn orders_billing_info(&self, skip: i32, count: i32, input: OrderBillingSearchInput) -> FieldResult<OrderBillingInfoSearchResults>;

    fn create_international_billing_info(&self, input: NewInternationalBillingInfoInput) -> FieldResult<InternationalBillingInfo>;

    fn update_international_billing_info(&self, input: UpdateInternationalBillingInfoInput) -> FieldResult<InternationalBillingInfo>;

    fn create_russia_billing_info(&self, input: NewRussiaBillingInfoInput) -> FieldResult<RussiaBillingInfo>;

    fn update_russia_billing_info(&self, input: UpdateRussiaBillingInfoInput) -> FieldResult<RussiaBillingInfo>;
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

    fn create_customer_with_source(&self, input: NewCustomerWithSourceRequest) -> FieldResult<Customer> {
        let request_path = format!("{}/with_source", Model::Customer.to_url());
        let url = self.request_url(&request_path);

        let body: String = serde_json::to_string(&input)?;
        self.context.request::<Customer>(Method::Post, url, Some(body)).wait()
    }

    fn get_current_customer(&self) -> FieldResult<Option<Customer>> {
        let request_path = format!("{}", Model::Customer.to_url());
        let url = self.request_url(&request_path);

        self.context.request::<Option<Customer>>(Method::Get, url, None).wait()
    }

    fn delete_customer(&self, payload: DeleteCustomerRequest) -> FieldResult<()> {
        let request_path = format!("{}", Model::Customer.to_url());
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&payload)?;
        self.context.request::<()>(Method::Delete, url, Some(body)).wait()
    }

    fn add_role_to_user(&self, input: NewBillingRoleInput) -> FieldResult<NewRole<BillingMicroserviceRole>> {
        let request_path = format!("{}", Model::Role.to_url());
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn remove_role_from_user(&self, input: RemoveBillingRoleInput) -> FieldResult<NewRole<BillingMicroserviceRole>> {
        let request_path = format!("{}", Model::Role.to_url());
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Delete, url, Some(body)).wait()
    }

    fn orders_billing_info(&self, skip: i32, count: i32, input: OrderBillingSearchInput) -> FieldResult<OrderBillingInfoSearchResults> {
        let request_path = format!("order_billing_info?skip={}&count={}", skip, count);
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn orders(&self, skip: i32, count: i32, input: OrderBillingSearchInput) -> FieldResult<OrderBillingSearchResults> {
        let request_path = format!("orders/search?skip={}&count={}", skip, count);
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn create_international_billing_info(&self, input: NewInternationalBillingInfoInput) -> FieldResult<InternationalBillingInfo> {
        let request_path = format!("billing_info/international");
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn update_international_billing_info(&self, input: UpdateInternationalBillingInfoInput) -> FieldResult<InternationalBillingInfo> {
        let request_path = format!("billing_info/international/{}", input.id);
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Put, url, Some(body)).wait()
    }

    fn create_russia_billing_info(&self, input: NewRussiaBillingInfoInput) -> FieldResult<RussiaBillingInfo> {
        let request_path = format!("billing_info/russia");
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn update_russia_billing_info(&self, input: UpdateRussiaBillingInfoInput) -> FieldResult<RussiaBillingInfo> {
        let request_path = format!("billing_info/russia/{}", input.id);
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Put, url, Some(body)).wait()
    }
}
