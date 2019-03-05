use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

use stq_static_resources::Currency;
use stq_types::{Quantity, StoreId, SubscriptionId, SubscriptionPaymentId};

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "New store subscription input object")]
pub struct CreateStoreSubscriptionInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Store id.")]
    #[serde(skip_serializing)]
    pub store_id: i32,
    #[graphql(description = "Currency.")]
    pub currency: Currency,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "Update store subscripotion input object")]
pub struct UpdateStoreSubscriptionInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Store id.")]
    #[serde(skip_serializing)]
    pub store_id: i32,
    #[graphql(description = "Currency.")]
    pub currency: Option<Currency>,
    #[graphql(description = "Status.")]
    pub status: Option<SubscriptionPaymentStatus>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StoreSubscription {
    pub store_id: StoreId,
    pub currency: Currency,
    pub value: BigDecimal,
    pub wallet_address: Option<String>,
    pub trial_start_date: Option<NaiveDateTime>,
    pub trial_end_date: Option<NaiveDateTime>,
    pub status: StoreSubscriptionStatus,
}

#[derive(GraphQLEnum, Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[graphql(name = "SubscriptionPaymentStatus", description = "Subscription payment status")]
#[serde(rename_all = "lowercase")]
pub enum StoreSubscriptionStatus {
    Trial,
    Paid,
    Free,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SubscriptionPayment {
    pub id: SubscriptionPaymentId,
    pub store_id: StoreId,
    pub amount: BigDecimal,
    pub currency: Currency,
    pub charge_id: Option<String>,
    pub transaction_id: Option<String>,
    pub status: SubscriptionPaymentStatus,
    pub created_at: NaiveDateTime,
}

#[derive(GraphQLEnum, Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[graphql(name = "SubscriptionPaymentStatus", description = "Subscription payment status")]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionPaymentStatus {
    Paid,
    Failed,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub store_id: StoreId,
    pub published_base_products_quantity: Quantity,
    pub subscription_payment_id: Option<SubscriptionPaymentId>,
    pub created_at: NaiveDateTime,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[graphql(description = "Subscription payment search parameters object")]
pub struct SubscriptionPaymentSearch {
    #[graphql(description = "Store id.")]
    pub store_id: Option<i32>,
    #[graphql(description = "Payment status.")]
    pub status: Option<SubscriptionPaymentStatus>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SubscriptionPaymentsSearchResults {
    pub total_count: i64,
    pub subscription_payments: Vec<SubscriptionPayment>,
}
