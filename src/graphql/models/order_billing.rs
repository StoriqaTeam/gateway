use juniper::FieldResult;

use stq_static_resources::Currency;
use stq_types::{InvoiceId, OrderId, OrderSlug, StoreId};

use graphql::context::Context;
use graphql::models::{BillingType, InternationalBillingInfo, PaymentState, ProxyCompanyBillingInfo, RussiaBillingInfo};

#[derive(GraphQLInputObject, Serialize, Debug, Clone, Default)]
#[graphql(description = "Orders search parameters object")]
pub struct OrderBillingSearchInput {
    #[graphql(description = "payment state.")]
    pub payment_state: Option<PaymentState>,
    #[graphql(description = "Store id.")]
    pub store_id: Option<i32>,
    #[graphql(description = "Order id.")]
    pub order_slug: Option<i32>,
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
    pub stripe_fee: Option<f64>,
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

#[derive(Debug, Clone, Serialize, Default)]
pub struct OrderBillingSearch {
    pub payment_state: Option<PaymentState>,
    pub store_id: Option<StoreId>,
    pub order_id: Option<OrderId>,
}

pub fn convert_search_term(context: &Context, search_term: OrderBillingSearchInput) -> FieldResult<OrderBillingSearch> {
    let OrderBillingSearchInput {
        payment_state,
        store_id,
        order_slug,
    } = search_term;

    let order_id = if let Some(order_slug) = order_slug {
        let order_id = context
            .get_orders_microservice()
            .get_order_by_slug(OrderSlug(order_slug))?
            .map(|order| order.0.id)
            .unwrap_or(any_random_uuid_order_id());
        Some(order_id)
    } else {
        None
    };

    Ok(OrderBillingSearch {
        payment_state,
        store_id: store_id.map(StoreId),
        order_id,
    })
}

fn any_random_uuid_order_id() -> OrderId {
    OrderId::new()
}
