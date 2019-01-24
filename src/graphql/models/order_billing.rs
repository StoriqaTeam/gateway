use stq_static_resources::Currency;
use stq_types::{Alpha3, InternationalBillingId, InvoiceId, OrderId, ProxyCompanyBillingInfoId, RussiaBillingId, StoreId, SwiftId};

use graphql::models::PaymentState;

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Orders search parameters object")]
pub struct OrderBillingSearchInput {
    #[graphql(description = "payment state.")]
    pub payment_state: Option<PaymentState>,
    #[graphql(description = "Store id.")]
    pub store_id: Option<i32>,
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
pub struct InternationalBillingInfo {
    pub id: InternationalBillingId,
    pub store_id: StoreId,
    pub swift_bic: SwiftId,
    pub bank_name: String,
    pub full_name: String,
    pub iban: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RussiaBillingInfo {
    pub id: RussiaBillingId,
    pub store_id: StoreId,
    pub kpp: String,
    pub bic: String,
    pub inn: String,
    pub full_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProxyCompanyBillingInfo {
    pub id: ProxyCompanyBillingInfoId,
    pub country: Alpha3,
    pub swift_bic: SwiftId,
    pub bank_name: String,
    pub full_name: String,
    pub iban: String,
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

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[graphql(name = "BillingType", description = "Billing type")]
pub enum BillingType {
    #[graphql(description = "International billing type.")]
    International,
    #[graphql(description = "Russian local billing type.")]
    Russia,
}
