use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::{InvoiceId, OrderId, StoreId, SubscriptionPaymentId};

use graphql::context::Context;
use graphql::microservice::requests::*;
use graphql::microservice::responses::PayoutCalculation;
use graphql::models::*;

pub trait BillingService {
    fn payment_intent_by_invoice(&self, invoice_id: InvoiceId) -> FieldResult<Option<PaymentIntent>>;

    fn create_customer_with_source(&self, input: NewCustomerWithSourceRequest) -> FieldResult<Customer>;

    fn update_customer(&self, input: UpdateCustomerInput) -> FieldResult<Customer>;

    fn get_current_customer(&self) -> FieldResult<Option<Customer>>;

    fn delete_customer(&self, payload: DeleteCustomerRequest) -> FieldResult<()>;

    fn add_role_to_user(&self, input: NewBillingRoleInput) -> FieldResult<NewRole<BillingMicroserviceRole>>;

    fn remove_role_from_user(&self, input: RemoveBillingRoleInput) -> FieldResult<NewRole<BillingMicroserviceRole>>;

    fn orders(&self, skip: i32, items_count: i32, input: OrderBillingSearch) -> FieldResult<OrderBillingSearchResults>;

    fn order(&self, order_id: OrderId) -> FieldResult<Option<OrderBilling>>;

    fn orders_billing_info(&self, skip: i32, count: i32, input: OrderBillingSearch) -> FieldResult<OrderBillingInfoSearchResults>;

    fn create_international_billing_info(&self, input: NewInternationalBillingInfoInput) -> FieldResult<InternationalBillingInfo>;

    fn update_international_billing_info(&self, input: UpdateInternationalBillingInfoInput) -> FieldResult<InternationalBillingInfo>;

    fn create_russia_billing_info(&self, input: NewRussiaBillingInfoInput) -> FieldResult<RussiaBillingInfo>;

    fn update_russia_billing_info(&self, input: UpdateRussiaBillingInfoInput) -> FieldResult<RussiaBillingInfo>;

    fn get_fee_by_order_id(&self, order_id: OrderId) -> FieldResult<Option<Fee>>;

    fn billing_type(&self, store_id: StoreId) -> FieldResult<Option<BillingType>>;

    fn international_billing_info(&self, store_id: StoreId) -> FieldResult<Option<InternationalBillingInfo>>;

    fn russia_billing_info(&self, store_id: StoreId) -> FieldResult<Option<RussiaBillingInfo>>;

    fn create_charge_fee_by_oder(&self, order_id: OrderId) -> FieldResult<Fee>;

    fn create_charge_fee_by_oders(&self, input: FeesPayByOrdersRequest) -> FieldResult<Vec<Fee>>;

    fn create_payment_intent_fee(&self, fee_id: FeeId) -> FieldResult<PaymentIntent>;

    fn calculate_payout(&self, input: CalculatePayoutPayload) -> FieldResult<PayoutCalculation>;

    fn get_payouts_by_store_id(&self, store_id: StoreId) -> FieldResult<PayoutsByStoreId>;

    fn pay_out_to_seller(&self, input: PayOutToSellerPayload) -> FieldResult<Payout>;

    fn get_balance_by_store_id(&self, store_id: StoreId) -> FieldResult<Balances>;

    fn create_store_subscription(&self, input: CreateStoreSubscriptionInput) -> FieldResult<StoreSubscription>;

    fn update_store_subscription(&self, input: UpdateStoreSubscriptionInput) -> FieldResult<StoreSubscription>;

    fn get_store_subscription(&self, store_id: StoreId) -> FieldResult<Option<StoreSubscription>>;

    fn subscription_payments(
        &self,
        skip: i32,
        count: i32,
        input: SubscriptionPaymentSearch,
    ) -> FieldResult<SubscriptionPaymentsSearchResults>;

    fn get_subscriptions(&self, subscription_payment_id: SubscriptionPaymentId) -> FieldResult<Vec<Subscription>>;
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

    fn update_customer(&self, input: UpdateCustomerInput) -> FieldResult<Customer> {
        let request_path = format!("{}", Model::Customer.to_url());
        let url = self.request_url(&request_path);

        let body: String = serde_json::to_string(&input)?;
        self.context.request::<Customer>(Method::Put, url, Some(body)).wait()
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

    fn orders_billing_info(&self, skip: i32, count: i32, input: OrderBillingSearch) -> FieldResult<OrderBillingInfoSearchResults> {
        let request_path = format!("order_billing_info?skip={}&count={}", skip, count);
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn orders(&self, skip: i32, count: i32, input: OrderBillingSearch) -> FieldResult<OrderBillingSearchResults> {
        let request_path = format!("orders/search?skip={}&count={}", skip, count);
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn order(&self, order_id: OrderId) -> FieldResult<Option<OrderBilling>> {
        let request_path = format!("orders/search?skip={}&count={}", 0, 1);
        let url = self.request_url(&request_path);
        let search = OrderBillingSearch {
            order_id: Some(order_id),
            ..Default::default()
        };
        let body: String = serde_json::to_string(&search)?;
        let mut result: OrderBillingSearchResults = self.context.request(Method::Post, url, Some(body)).wait()?;

        Ok(result.orders.pop())
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

    fn get_fee_by_order_id(&self, order_id: OrderId) -> FieldResult<Option<Fee>> {
        let request_path = format!("fees/by-order-id/{}", order_id);
        let url = self.request_url(&request_path);
        self.context.request::<Option<Fee>>(Method::Get, url, None).wait()
    }

    fn billing_type(&self, store_id: StoreId) -> FieldResult<Option<BillingType>> {
        let request_path = format!("billing_type/by-store-id/{}", store_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn international_billing_info(&self, store_id: StoreId) -> FieldResult<Option<InternationalBillingInfo>> {
        let request_path = format!("billing_info/international/by-store-id/{}", store_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn russia_billing_info(&self, store_id: StoreId) -> FieldResult<Option<RussiaBillingInfo>> {
        let request_path = format!("billing_info/russia/by-store-id/{}", store_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn create_charge_fee_by_oder(&self, order_id: OrderId) -> FieldResult<Fee> {
        let request_path = format!("fees/by-order-id/{}/pay", order_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Post, url, None).wait()
    }

    fn create_charge_fee_by_oders(&self, input: FeesPayByOrdersRequest) -> FieldResult<Vec<Fee>> {
        let request_path = "fees/by-order-ids/pay";
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn create_payment_intent_fee(&self, fee_id: FeeId) -> FieldResult<PaymentIntent> {
        let request_path = format!("payment_intents/fees/{}", fee_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Post, url, None).wait()
    }

    fn calculate_payout(&self, input: CalculatePayoutPayload) -> FieldResult<PayoutCalculation> {
        let request_path = "payouts/calculate";
        let url = self.request_url(&request_path);
        let body = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn get_payouts_by_store_id(&self, store_id: StoreId) -> FieldResult<PayoutsByStoreId> {
        let request_path = format!("payouts/by-store-id/{}", store_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn pay_out_to_seller(&self, input: PayOutToSellerPayload) -> FieldResult<Payout> {
        let request_path = "payouts";
        let url = self.request_url(&request_path);
        let body = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }
    fn get_balance_by_store_id(&self, store_id: StoreId) -> FieldResult<Balances> {
        let request_path = format!("balance/by-store-id/{}", store_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn create_store_subscription(&self, input: CreateStoreSubscriptionInput) -> FieldResult<StoreSubscription> {
        let request_path = format!("store_subscription/by-store-id/{}", input.store_id);
        let url = self.request_url(&request_path);
        let body = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn update_store_subscription(&self, input: UpdateStoreSubscriptionInput) -> FieldResult<StoreSubscription> {
        let request_path = format!("store_subscription/by-store-id/{}", input.store_id);
        let url = self.request_url(&request_path);
        let body = serde_json::to_string(&input)?;
        self.context.request(Method::Put, url, Some(body)).wait()
    }

    fn get_store_subscription(&self, store_id: StoreId) -> FieldResult<Option<StoreSubscription>> {
        let request_path = format!("store_subscription/by-store-id/{}", store_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn subscription_payments(
        &self,
        skip: i32,
        count: i32,
        input: SubscriptionPaymentSearch,
    ) -> FieldResult<SubscriptionPaymentsSearchResults> {
        let request_path = format!("subscription/payment/search?skip={}&count={}", skip, count);
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn get_subscriptions(&self, subscription_payment_id: SubscriptionPaymentId) -> FieldResult<Vec<Subscription>> {
        let request_path = format!("subscriptions/by-subscription-payment-id//{}", subscription_payment_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }
}
