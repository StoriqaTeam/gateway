use stq_static_resources::Currency;
use stq_types::{InvoiceId, OrderId, StoreId};

use graphql::models::{BillingType, InternationalBillingInfo, PaymentState, ProxyCompanyBillingInfo, RussiaBillingInfo};

#[derive(GraphQLInputObject, Serialize, Debug, Clone, Default)]
#[graphql(description = "Orders search parameters object")]
pub struct OrderBillingSearchInput {
    #[graphql(description = "payment state.")]
    pub payment_state: Option<PaymentState>,
    #[graphql(description = "Store id.")]
    pub store_id: Option<i32>,
    #[graphql(description = "Order id.")]
    pub order_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderBillingInfo {
    pub order: OrderBilling,
    pub billing_type: BillingType,
    pub proxy_company_billing_info: Option<ProxyCompanyBillingInfo>,
    pub russia_billing_info: Option<RussiaBillingInfo>,
    pub international_billing_info: Option<InternationalBillingInfo>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderBilling {
    pub id: OrderId,
    pub seller_currency: Currency,
    pub total_amount: f64,
    pub cashback_amount: f64,
    pub invoice_id: InvoiceId,
    pub store_id: StoreId,
    pub state: PaymentState,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OrderBillingInfoSearchResults {
    pub total_count: u32,
    pub orders: Vec<OrderBillingInfo>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OrderBillingSearchResults {
    pub total_count: u32,
    pub orders: Vec<OrderBilling>,
}
